use std::{cmp::Ordering, io::BufRead};

fn is_within(ix: isize, iy: isize, target: (isize, isize, isize, isize)) -> Option<isize> {
    let mut dx = ix;
    let mut dy = iy;

    let mut c_x = 0;
    let mut c_y = 0;
    let mut max_height = 0;
    loop {
        // println!("{} {}", dx, dy);
        if target.0 <= c_x && c_x <= target.1 && target.2 <= c_y && c_y <= target.3 {
            // println!("{} {} - {}", ix, iy, max_height);
            return Some(max_height);
        }
        if dx == 0 && (c_x < target.0 || c_x > target.1 || c_y < target.3) {
            break;
        }
        c_x += dx;
        c_y += dy;

        max_height = max_height.max(c_y);

        dx = match dx.cmp(&0) {
            Ordering::Less => dx + 1,
            Ordering::Equal => 0,
            Ordering::Greater => dx - 1,
        };
        dy -= 1;
    }
    None
}

pub fn star_one(mut input: impl BufRead) -> usize {
    let target = {
        let mut buf = String::new();

        let _res = input.read_to_string(&mut buf);
        let s = buf.strip_prefix("target area: x=").unwrap();
        let (left, right) = s.split_once(", y=").unwrap();
        let (lower_x, upper_x) = left.split_once("..").unwrap();
        let (lower_y, upper_y) = right.split_once("..").unwrap();
        (
            lower_x.parse::<isize>().unwrap(),
            upper_x.parse::<isize>().unwrap(),
            lower_y.parse::<isize>().unwrap(),
            upper_y.parse::<isize>().unwrap(),
        )
    };

    let mut max_height = 0;
    for y in 0..1000 {
        for x in 0..=target.1 {
            if let Some(height) = is_within(x, y, target) {
                max_height = max_height.max(height);
            }
        }
    }
    max_height as usize
}

pub fn star_two(mut input: impl BufRead) -> usize {
    let target = {
        let mut buf = String::new();

        let _res = input.read_to_string(&mut buf);
        let s = buf.strip_prefix("target area: x=").unwrap();
        let (left, right) = s.split_once(", y=").unwrap();
        let (lower_x, upper_x) = left.split_once("..").unwrap();
        let (lower_y, upper_y) = right.split_once("..").unwrap();
        (
            lower_x.parse::<isize>().unwrap(),
            upper_x.parse::<isize>().unwrap(),
            lower_y.parse::<isize>().unwrap(),
            upper_y.parse::<isize>().unwrap(),
        )
    };

    ((target.3 * 10)..100)
        .flat_map(|y| (0..=target.1).filter_map(move |x| is_within(x, y, target)))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_star_one() {
        let input = b"target area: x=20..30, y=-10..-5";
        assert_eq!(star_one(Cursor::new(input)), 45);
    }

    #[test]
    fn test_star_two() {
        let input = b"target area: x=20..30, y=-10..-5";
        assert_eq!(star_two(Cursor::new(input)), 112);
    }
}
