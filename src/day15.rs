use std::{collections::BinaryHeap, io::BufRead};

#[derive(Debug, PartialEq, Eq)]
struct Node {
    pos: (usize, usize),
    score: usize,
    cost: usize, // uses distance as well.
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.cost.cmp(&other.cost) {
            core::cmp::Ordering::Equal => {}
            ord => return ord,
        }
        match self.score.cmp(&other.score) {
            core::cmp::Ordering::Equal => {}
            ord => return ord,
        }
        self.pos.cmp(&other.pos)
    }
}

#[allow(dead_code)]
pub fn star_one_old(input: impl BufRead) -> usize {
    let map: Vec<Vec<_>> = input
        .lines()
        .map(|line| {
            let line = line.unwrap();
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect();

    let mut stack = BinaryHeap::new();

    stack.push(Node {
        pos: (0, 0),
        score: 0,
        cost: 0,
    });

    let dirs: [(isize, isize); 2] = [
        // (1, 1),
        (1, 0),
        (0, 1),
        // (0, -1),
        // (1, -1),
        // (-1, 0),
        // (-1, 1),
        // (-1, -1),
    ];

    let mut min_total = usize::MAX;
    let end_pos = (map.len() - 1, map[0].len() - 1);

    while let Some(Node { pos, score, .. }) = stack.pop() {
        if score < min_total {
            if pos == end_pos {
                println!("GOt a min {}", score);
                min_total = min_total.min(score);
            } else {
                stack.extend(
                    dirs.iter()
                        .filter_map(|(di, dj)| {
                            if pos.0 as isize + di >= 0 && pos.1 as isize + dj >= 0 {
                                let new_pos = (
                                    (pos.0 as isize + di) as usize,
                                    (pos.1 as isize + dj) as usize,
                                );
                                if new_pos.0 < map.len() && new_pos.1 < map[new_pos.0].len() {
                                    return Some(new_pos);
                                }
                            }
                            None
                        })
                        .map(|new_pos| {
                            let delta_x = end_pos.0 - new_pos.0;
                            let delta_y = end_pos.1 - new_pos.1;
                            let distance =
                                f64::sqrt((delta_x.pow(2) + delta_y.pow(2)) as f64) as usize + 1;
                            Node {
                                pos: new_pos,
                                score: score + map[pos.0][pos.1],
                                cost: score + map[pos.0][pos.1] + distance,
                            }
                        }),
                );
            }
        }
    }
    min_total
}

fn get(costs: &[Vec<Option<usize>>], i: isize, j: isize) -> Option<usize> {
    if i < 0 || j < 0 {
        return None;
    }
    costs
        .get(i as usize)
        .and_then(|row| row.get(j as usize).cloned())
        .flatten()
}

fn get_min_costs(costs: &[Vec<Option<usize>>], i: isize, j: isize) -> Option<usize> {
    let dirs: [(isize, isize); 4] = [
        // (1, 1),
        (1, 0),
        (0, 1),
        (0, -1),
        // (1, -1),
        (-1, 0),
        // (-1, 1),
        // (-1, -1),
    ];
    dirs.iter()
        .filter_map(|(di, dj)| get(costs, i + di, j + dj))
        .min()
}

pub fn star_one(input: impl BufRead) -> usize {
    let map: Vec<Vec<_>> = input
        .lines()
        .map(|line| {
            let line = line.unwrap();
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect();

    let mut costs: Vec<Vec<Option<usize>>> = (0..map.len())
        .map(|x| (0..map[x].len()).map(|_| None).collect())
        .collect();

    costs[0][0] = Some(0);

    for i in 0..costs.len() {
        for j in 0..costs[i].len() {
            if let Some(v) = get_min_costs(&costs, i as isize, j as isize) {
                costs[i][j] = Some(v + map[i][j] as usize);
            }
        }
    }

    costs.last().unwrap().last().unwrap().unwrap()
}

fn map_create(input: impl BufRead) -> Vec<Vec<u8>> {
    let template: Vec<Vec<_>> = input
        .lines()
        .map(|line| {
            let line = line.unwrap();
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect();

    let height = template.len();
    let width = template[0].len();

    (0..height * 5)
        .map(|i| {
            (0..width * 5)
                .map(|j| {
                    if i < height && j < width {
                        template[i][j]
                    } else {
                        let inc = (i / height) + (j / width);
                        let new_num = inc + template[i % height][j % width] as usize;
                        if new_num > 9 {
                            (new_num % 9) as u8
                        } else {
                            new_num as u8
                        }
                    }
                })
                .collect()
        })
        .collect()
}

pub fn star_two(input: impl BufRead) -> usize {
    let map = map_create(input);
    let mut costs: Vec<Vec<Option<usize>>> = (0..map.len())
        .map(|x| (0..map[x].len()).map(|_| None).collect())
        .collect();

    costs[0][0] = Some(0);

    let mut changed = true;

    while changed {
        changed = false;
        for i in 0..costs.len() {
            for j in 0..costs[i].len() {
                if let Some(v) = get_min_costs(&costs, i as isize, j as isize) {
                    let new_value = v + map[i][j] as usize;
                    if costs[i][j].map(|cost| cost > new_value).unwrap_or(true) {
                        costs[i][j] = Some(new_value);
                        changed = true;
                    }
                }
            }
        }
    }

    costs.last().unwrap().last().unwrap().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    const INPUT: &[u8] = b"1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";

    #[test]
    fn test_star_one() {
        assert_eq!(star_one(Cursor::new(INPUT)), 40);
    }

    #[test]
    fn test_star_two_map_create() {
        let expected: Vec<Vec<u8>> = b"11637517422274862853338597396444961841755517295286
13813736722492484783351359589446246169155735727126
21365113283247622439435873354154698446526571955763
36949315694715142671582625378269373648937148475914
74634171118574528222968563933317967414442817852555
13191281372421239248353234135946434524615754563572
13599124212461123532357223464346833457545794456865
31254216394236532741534764385264587549637569865174
12931385212314249632342535174345364628545647573965
23119445813422155692453326671356443778246755488935
22748628533385973964449618417555172952866628316397
24924847833513595894462461691557357271266846838237
32476224394358733541546984465265719557637682166874
47151426715826253782693736489371484759148259586125
85745282229685639333179674144428178525553928963666
24212392483532341359464345246157545635726865674683
24611235323572234643468334575457944568656815567976
42365327415347643852645875496375698651748671976285
23142496323425351743453646285456475739656758684176
34221556924533266713564437782467554889357866599146
33859739644496184175551729528666283163977739427418
35135958944624616915573572712668468382377957949348
43587335415469844652657195576376821668748793277985
58262537826937364893714847591482595861259361697236
96856393331796741444281785255539289636664139174777
35323413594643452461575456357268656746837976785794
35722346434683345754579445686568155679767926678187
53476438526458754963756986517486719762859782187396
34253517434536462854564757396567586841767869795287
45332667135644377824675548893578665991468977611257
44961841755517295286662831639777394274188841538529
46246169155735727126684683823779579493488168151459
54698446526571955763768216687487932779859814388196
69373648937148475914825958612593616972361472718347
17967414442817852555392896366641391747775241285888
46434524615754563572686567468379767857948187896815
46833457545794456865681556797679266781878137789298
64587549637569865174867197628597821873961893298417
45364628545647573965675868417678697952878971816398
56443778246755488935786659914689776112579188722368
55172952866628316397773942741888415385299952649631
57357271266846838237795794934881681514599279262561
65719557637682166874879327798598143881961925499217
71484759148259586125936169723614727183472583829458
28178525553928963666413917477752412858886352396999
57545635726865674683797678579481878968159298917926
57944568656815567976792667818781377892989248891319
75698651748671976285978218739618932984172914319528
56475739656758684176786979528789718163989182927419
67554889357866599146897761125791887223681299833479"
            .lines()
            .map(|r| {
                r.unwrap()
                    .chars()
                    .map(|c| c.to_digit(10).unwrap() as u8)
                    .collect()
            })
            .collect();
        let p = map_create(Cursor::new(INPUT));

        assert_eq!(p, expected);
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two(Cursor::new(INPUT)), 315);
    }
}
