use std::io::BufRead;

fn is_lowest(data: &[Vec<u32>], i: usize, j: usize) -> bool {
    let x = data[i][j];
    let i = i as isize;
    let j = j as isize;
    let dirs = [(-1, 0), (0, -1), (1, 0), (0, 1)];

    dirs.into_iter().all(|(up, left)| {
        let pos_i = i + up;
        let pos_j = j + left;

        if pos_i >= 0
            && pos_i < data.len() as isize
            && pos_j >= 0
            && pos_j < data[i as usize].len() as isize
        {
            data[pos_i as usize][pos_j as usize] > x
        } else {
            true
        }
    })
}

pub fn star_one(input: impl BufRead) -> usize {
    let data: Vec<Vec<u32>> = input
        .lines()
        .map(|l| {
            l.unwrap()
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect()
        })
        .collect();
    data.iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter(|(j, _x)| is_lowest(&data, i, *j))
                // .inspect(|x| println!("{:?}", x))
                .map(|(_j, x)| (x + 1) as usize)
                .collect::<Vec<usize>>()
        })
        .sum()
}

fn get_size(visited: &mut [Vec<bool>], i: usize, j: usize) -> usize {
    // Flood fill

    let mut total = 0;
    let mut stack = vec![(i, j)];

    while let Some((i, j)) = stack.pop() {
        if !visited[i][j] {
            total += 1;
            visited[i][j] = true;

            let dirs = [(-1, 0), (0, -1), (1, 0), (0, 1)];
            for (up, left) in dirs {
                let pos_i = i as isize + up;
                let pos_j = j as isize + left;

                // Is the next one within the bounds and have we visited it.
                if pos_i >= 0
                    && pos_i < visited.len() as isize
                    && pos_j >= 0
                    && pos_j < visited[i].len() as isize
                    && !visited[pos_i as usize][pos_j as usize]
                {
                    stack.push((pos_i as usize, pos_j as usize));
                }
            }
        }
    }
    total
}

pub fn star_two(input: impl BufRead) -> usize {
    let data: Vec<Vec<u32>> = input
        .lines()
        .map(|l| {
            l.unwrap()
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect()
        })
        .collect();

    let mut visited: Vec<Vec<_>> = data
        .iter()
        // Create barriers if the value is 9 (highest value)
        .map(|row| row.iter().map(|&x| x == 9).collect::<Vec<_>>())
        .collect();

    let mut total = Vec::new();

    // Get next starting point
    for i in 0..data.len() {
        for j in 0..data[i].len() {
            if !visited[i][j] {
                total.push(get_size(&mut visited, i, j));
            }
        }
    }
    total.sort_unstable();
    total.into_iter().rev().take(3).product()
}

#[cfg(test)]
mod tests {
    use super::{star_one, star_two};
    use std::io::Cursor;

    const INPUT: &[u8] = b"2199943210
3987894921
9856789892
8767896789
9899965678";

    #[test]
    fn test_star_one() {
        assert_eq!(star_one(Cursor::new(INPUT)), 15);
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two(Cursor::new(INPUT)), 1134);
    }
}
