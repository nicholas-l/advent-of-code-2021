use std::io::{Read, Result};
use std::{collections::HashMap, io::BufRead};

#[derive(Debug, PartialEq, Eq)]
enum Packet {
    Literal {
        version: usize,
        value: usize,
    },
    Operator {
        version: usize,
        type_id: usize,
        packets: Vec<Packet>,
    },
}

impl Packet {
    fn sum_version(&self) -> usize {
        match self {
            Packet::Literal { version, .. } => *version,
            Packet::Operator {
                version, packets, ..
            } => *version + packets.iter().map(|p| p.sum_version()).sum::<usize>(),
        }
    }

    fn compute(&self) -> usize {
        match self {
            Packet::Literal { value, .. } => *value,
            Packet::Operator {
                type_id, packets, ..
            } => match type_id {
                0 => packets.iter().map(|p| p.compute()).sum(),
                1 => packets.iter().map(|p| p.compute()).product(),
                2 => packets.iter().map(|p| p.compute()).min().unwrap(),
                3 => packets.iter().map(|p| p.compute()).max().unwrap(),
                4 => panic!(),
                5 => {
                    if packets[0].compute() > packets[1].compute() {
                        1
                    } else {
                        0
                    }
                }
                6 => {
                    if packets[0].compute() < packets[1].compute() {
                        1
                    } else {
                        0
                    }
                }
                7 => {
                    if packets[0].compute() == packets[1].compute() {
                        1
                    } else {
                        0
                    }
                }
                _ => unreachable!(),
            },
        }
    }
}

fn parse_binary(binary: &[u8]) -> usize {
    binary.iter().fold(0, |acc, b| acc * 2 + *b as usize)
}

fn parse_packet(binary: &mut &[u8]) -> Result<Packet> {
    let version = {
        let mut buf = [0u8; 3];
        binary.read_exact(&mut buf)?;
        parse_binary(&buf)
    };

    let packet_type = {
        let mut buf = [0u8; 3];
        binary.read_exact(&mut buf)?;
        parse_binary(&buf)
    };
    match packet_type {
        4 => {
            let mut buf_value = Vec::new();
            let mut buf = [1u8; 5];
            while buf[0] != 0 {
                binary.read_exact(&mut buf)?;
                buf_value.extend(buf.iter().skip(1));
            }
            Ok(Packet::Literal {
                version,
                value: parse_binary(&buf_value),
            })
        }
        x => {
            // Length bit
            let length_of_bits = {
                let mut buf = [0u8; 1];
                binary.read_exact(&mut buf)?;
                match buf[0] {
                    0 => 15,
                    1 => 11,
                    _ => panic!(),
                }
            };
            let sub_packets = if length_of_bits == 11 {
                let number_of_packets = {
                    let mut buf = vec![0; length_of_bits];
                    binary.read_exact(&mut buf).unwrap();
                    parse_binary(&buf)
                };
                (0..number_of_packets)
                    .map(|_x| parse_packet(binary).unwrap())
                    .collect()
            } else {
                let length_of_bits = {
                    let mut buf = vec![0; length_of_bits];
                    binary.read_exact(&mut buf).unwrap();
                    parse_binary(&buf)
                };

                let mut buf = vec![0; length_of_bits];
                binary.read_exact(&mut buf).unwrap();
                let mut packets = Vec::new();
                let mut sub_packets_buf = &buf[..];
                while !sub_packets_buf.is_empty() && !buf.iter().all(|x| x == &0) {
                    packets.push(parse_packet(&mut sub_packets_buf)?);
                }
                packets
            };
            Ok(Packet::Operator {
                version,
                type_id: x,
                packets: sub_packets,
            })
        }
    }
}

fn parse_hex(mut input: impl BufRead) -> Vec<u8> {
    let lookup = HashMap::from([
        ('0', [0, 0, 0, 0]),
        ('1', [0, 0, 0, 1]),
        ('2', [0, 0, 1, 0]),
        ('3', [0, 0, 1, 1]),
        ('4', [0, 1, 0, 0]),
        ('5', [0, 1, 0, 1]),
        ('6', [0, 1, 1, 0]),
        ('7', [0, 1, 1, 1]),
        ('8', [1, 0, 0, 0]),
        ('9', [1, 0, 0, 1]),
        ('A', [1, 0, 1, 0]),
        ('B', [1, 0, 1, 1]),
        ('C', [1, 1, 0, 0]),
        ('D', [1, 1, 0, 1]),
        ('E', [1, 1, 1, 0]),
        ('F', [1, 1, 1, 1]),
    ]);

    let mut buf = String::new();
    let _res = input.read_to_string(&mut buf);

    let binary: Vec<u8> = buf.chars().flat_map(|c| lookup[&c]).collect();

    binary
}

pub fn star_one(input: impl BufRead) -> usize {
    let binary = parse_hex(input);
    let packets = parse_packet(&mut &binary[..]).unwrap();

    packets.sum_version()
}

pub fn star_two(input: impl BufRead) -> usize {
    let binary = parse_hex(input);
    let packets = parse_packet(&mut &binary[..]).unwrap();

    packets.compute()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_parse_packet_literal() {
        let input = &[
            1, 1, 0, 1, 0, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 0, 1, 0, 0, 0,
        ];
        assert_eq!(
            parse_packet(&mut &input[..]).unwrap(),
            Packet::Literal {
                version: 6,
                value: 2021
            }
        );

        let input = [
            0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 1, 1, 1, 1, 0, 1, 0, 0, 0,
            1, 0, 1, 0, 0, 1, 0, 1, 0, 0, 1, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];
        assert_eq!(
            parse_packet(&mut &input[..]).unwrap(),
            Packet::Operator {
                version: 1,
                type_id: 6,
                packets: vec![
                    Packet::Literal {
                        version: 6,
                        value: 10
                    },
                    Packet::Literal {
                        version: 2,
                        value: 20
                    }
                ]
            }
        );

        let input = [
            1, 1, 1, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 1,
            1, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0,
        ];

        assert_eq!(
            parse_packet(&mut &input[..]).unwrap(),
            Packet::Operator {
                version: 7,
                type_id: 3,
                packets: vec![
                    Packet::Literal {
                        version: 2,
                        value: 1
                    },
                    Packet::Literal {
                        version: 4,
                        value: 2
                    },
                    Packet::Literal {
                        version: 1,
                        value: 3
                    }
                ]
            }
        );
    }

    #[test]
    fn test_parse_hex() {
        let input = b"D2FE28";
        let expected = vec![
            1, 1, 0, 1, 0, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 0, 1, 0, 0, 0,
        ];
        assert_eq!(parse_hex(Cursor::new(input)), expected);

        let input = b"EE00D40C823060";
        let expected = vec![
            1, 1, 1, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 1,
            1, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0,
        ];
        assert_eq!(parse_hex(Cursor::new(input)), expected);
    }

    #[test]
    fn test_star_one() {
        let input = b"38006F45291200";
        assert_eq!(star_one(Cursor::new(input)), 9);

        let input = b"EE00D40C823060";
        assert_eq!(star_one(Cursor::new(input)), 14);

        let input = b"8A004A801A8002F478";
        assert_eq!(star_one(Cursor::new(input)), 16);

        let input = b"620080001611562C8802118E34";
        assert_eq!(star_one(Cursor::new(input)), 12);

        let input = b"C0015000016115A2E0802F182340";
        assert_eq!(star_one(Cursor::new(input)), 23);

        let input = b"A0016C880162017C3686B18A3D4780";
        assert_eq!(star_one(Cursor::new(input)), 31);
    }

    #[test]
    fn test_star_two() {
        let input = b"C200B40A82";
        assert_eq!(star_two(Cursor::new(input)), 3);
    }
}
