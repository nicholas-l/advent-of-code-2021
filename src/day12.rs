use std::{
    collections::{HashMap, HashSet},
    io::BufRead,
};

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
enum Node {
    Start,
    End,
    LowerNode(String),
    UpperNode(String),
}

fn is_all_lowercase(s: &str) -> bool {
    s.chars().all(|c| c.is_ascii_lowercase())
}

pub fn star_one(input: impl BufRead) -> usize {
    let graph = input
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let (node1, node2) = line.split_once('-').unwrap();
            (node1.to_owned(), node2.to_owned())
        })
        .fold(HashMap::new(), |mut hm, (n1, n2)| {
            hm.entry(n1.clone())
                .or_insert_with(Vec::new)
                .push(n2.clone());
            hm.entry(n2).or_insert_with(Vec::new).push(n1);
            hm
        });

    let mut stack = vec![("start".to_owned(), Vec::new())];

    let mut paths = HashSet::new();

    while let Some((node, mut path)) = stack.pop() {
        if node == "end" {
            path.push("end".to_owned());
            paths.insert(path);
        } else if !is_all_lowercase(&node) || !path.contains(&node) {
            path.push(node.clone());
            stack.extend(
                graph
                    .get(&node)
                    .unwrap()
                    .iter()
                    .map(|node| (node.to_owned(), path.clone())),
            );
        }
    }
    paths.len()
}

pub fn star_two(input: impl BufRead) -> usize {
    let graph = input
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let (node1, node2) = line.split_once('-').unwrap();
            (node1.to_owned(), node2.to_owned())
        })
        .map(|v| {
            let n1 = match v.0.as_str() {
                "start" => Node::Start,
                "end" => Node::End,
                s if is_all_lowercase(s) => Node::LowerNode(v.0),
                s => Node::UpperNode(v.0),
                _ => panic!(),
            };
            let n2 = match v.1.as_str() {
                "start" => Node::Start,
                "end" => Node::End,
                s if is_all_lowercase(s) => Node::LowerNode(v.1),
                s => Node::UpperNode(v.1),
                _ => panic!(),
            };
            (n1, n2)
        })
        .fold(HashMap::new(), |mut hm, (n1, n2)| {
            hm.entry(n1.clone())
                .or_insert_with(Vec::new)
                .push(n2.clone());
            hm.entry(n2).or_insert_with(Vec::new).push(n1);
            hm
        });

    let small_caves = graph.keys().filter(|n| matches!(n, Node::LowerNode(_)));

    let mut stack: Vec<(_, _, _, _)> = small_caves
        .map(|cave| (Node::Start, Vec::new(), cave, HashMap::new()))
        .collect();

    let mut paths = HashSet::new();

    while let Some((node, mut path, twice_small, mut visit_count)) = stack.pop() {
        match node {
            Node::Start => {
                if path.is_empty() {
                    path.push(node.clone());
                    stack.extend(graph.get(&node).unwrap().iter().map(|node| {
                        (
                            node.to_owned(),
                            path.clone(),
                            twice_small,
                            visit_count.clone(),
                        )
                    }));
                }
            }
            Node::End => {
                path.push(Node::End);
                paths.insert(path);
            }
            Node::LowerNode(_) => {
                let max_count = if twice_small == &node { 2 } else { 1 };
                if visit_count.get(&node).unwrap_or(&0) < &max_count {
                    path.push(node.clone());
                    *visit_count.entry(node.clone()).or_insert(0) += 1;
                    stack.extend(graph.get(&node).unwrap().iter().map(|node| {
                        (
                            node.to_owned(),
                            path.clone(),
                            twice_small,
                            visit_count.clone(),
                        )
                    }));
                }
            }
            Node::UpperNode(_) => {
                path.push(node.clone());
                stack.extend(graph.get(&node).unwrap().iter().map(|node| {
                    (
                        node.to_owned(),
                        path.clone(),
                        twice_small,
                        visit_count.clone(),
                    )
                }));
            }
        }
    }
    paths.len()
}

#[cfg(test)]
mod tests {
    use super::{star_one, star_two};
    use std::io::Cursor;

    const INPUT1: &[u8] = b"start-A
start-b
A-c
A-b
b-d
A-end
b-end";

    const INPUT2: &[u8] = b"dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc";

    const INPUT3: &[u8] = b"fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW";

    #[test]
    fn test_star_one() {
        assert_eq!(star_one(Cursor::new(INPUT1)), 10);
        assert_eq!(star_one(Cursor::new(INPUT2)), 19);
        assert_eq!(star_one(Cursor::new(INPUT3)), 226);
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two(Cursor::new(INPUT1)), 36);
        assert_eq!(star_two(Cursor::new(INPUT2)), 103);
        assert_eq!(star_two(Cursor::new(INPUT3)), 3509);
    }
}
