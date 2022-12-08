// use anyhow::anyhow;

use itertools::Itertools;

const INPUT: &str = include_str!("../resources/input08.txt");

#[must_use]
pub fn solve_part_a() -> usize {
    part_a(INPUT)
}

#[must_use]
pub fn solve_part_b() -> usize {
    part_b(INPUT)
}

fn part_a(input: &str) -> usize {
    let mut trees = input
        .lines()
        .with_position()
        .map(|l| match l {
            itertools::Position::Middle(s) => s
                .chars()
                .with_position()
                .map(|p| match p {
                    itertools::Position::Middle(c) => (c, false),
                    itertools::Position::First(c)
                    | itertools::Position::Last(c)
                    | itertools::Position::Only(c) => (c, true),
                })
                .collect_vec(),
            itertools::Position::First(s)
            | itertools::Position::Last(s)
            | itertools::Position::Only(s) => s.chars().map(|c| (c, true)).collect_vec(),
        })
        .collect_vec();

    let height = trees.len();
    let width = trees[0].len();

    //left to right
    #[allow(clippy::needless_range_loop)]
    for y in 1..=(height - 2) {
        let (mut highest_vis, _) = trees[y][0];
        for x in 1..(width - 1) {
            let (h, _) = trees[y][x];
            if h > highest_vis {
                highest_vis = h;
                trees[y][x] = (h, true);
            }
        }
    }

    //right to left
    #[allow(clippy::needless_range_loop)]
    for y in 1..=(height - 2) {
        let (mut highest_vis, _) = trees[y][width - 1];
        for x in (1..(width - 1)).rev() {
            let (h, _) = trees[y][x];
            if h > highest_vis {
                highest_vis = h;
                trees[y][x] = (h, true);
            }
        }
    }

    //top to bottom
    for x in 1..(width - 1) {
        let (mut highest_vis, _) = trees[0][x];

        #[allow(clippy::needless_range_loop)]
        for y in 1..(height - 1) {
            let (h, _) = trees[y][x];
            if h > highest_vis {
                highest_vis = h;
                trees[y][x] = (h, true);
            }
        }
    }

    //bottom to top
    for x in 1..(width - 1) {
        let (mut highest_vis, _) = trees[height - 1][x];
        #[allow(clippy::needless_range_loop)]
        for y in (1..(height - 1)).rev() {
            let (h, _) = trees[y][x];
            if h > highest_vis {
                highest_vis = h;
                trees[y][x] = (h, true);
            }
        }
    }

    trees
        .into_iter()
        .map(|r| r.iter().filter(|(_, v)| *v).count())
        .sum()
}

fn part_b(input: &str) -> usize {
    let trees = input.lines().map(|s| s.chars().collect_vec()).collect_vec();
    let height = trees.len();
    let width = trees[0].len();

    let mut max_score = 0;

    for y in 0..(height - 1) {
        for x in 0..(width - 1) {
            max_score = max_score.max(
                score_neg_x(&trees, x, y)
                    * score_pos_x(&trees, x, y)
                    * score_neg_y(&trees, x, y)
                    * score_pos_y(&trees, x, y),
            );
        }
    }

    max_score
}

fn score_neg_x(trees: &[Vec<char>], x: usize, y: usize) -> usize {
    let h = trees[y][x];

    for i in (0..x).rev() {
        if trees[y][i] >= h {
            return x - i;
        }
    }

    x
}

fn score_pos_x(trees: &[Vec<char>], x: usize, y: usize) -> usize {
    let h = trees[y][x];

    for i in (x + 1)..trees[y].len() {
        if trees[y][i] >= h {
            return i - x;
        }
    }
    trees[y].len() - x - 1
}

fn score_neg_y(trees: &[Vec<char>], x: usize, y: usize) -> usize {
    let h = trees[y][x];

    for i in (0..y).rev() {
        if trees[i][x] >= h {
            return y - i;
        }
    }
    y
}

fn score_pos_y(trees: &[Vec<char>], x: usize, y: usize) -> usize {
    let h = trees[y][x];

    #[allow(clippy::needless_range_loop)]
    for i in (y + 1)..trees.len() {
        if trees[i][x] >= h {
            return i - y;
        }
    }
    trees[x].len() - y - 1
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = include_str!("../resources/example08.txt");

    #[test]
    fn example_a() -> Result<(), anyhow::Error> {
        assert_eq!(part_a(EXAMPLE), 21);
        Ok(())
    }

    #[test]
    fn example_b() -> Result<(), anyhow::Error> {
        assert_eq!(part_b(EXAMPLE), 8);
        Ok(())
    }
}
