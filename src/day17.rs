use std::convert::identity;

const INPUT: &str = include_str!("../resources/input17.txt");

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Shape {
    HLine,
    Cross,
    L,
    VLine,
    Square,
}

impl Shape {
    pub const fn points(self) -> &'static [(u32, u32)] {
        match self {
            Self::HLine => &[(0, 0), (1, 0), (2, 0), (3, 0)],
            Self::Cross => &[(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)],
            Self::L => &[(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],
            Self::VLine => &[(0, 0), (0, 1), (0, 2), (0, 3)],
            Self::Square => &[(0, 0), (1, 0), (0, 1), (1, 1)],
        }
    }
    pub fn write(self, (x, y): (u32, u32), field: &mut [[bool; 7]]) {
        let x = x as usize;
        let y = y as usize;

        for &(px, py) in self.points() {
            field[y + py as usize][x + px as usize] = true;
        }
    }

    pub fn is_clear(self, (x, y): (u32, u32), field: &[[bool; 7]]) -> bool {
        self.points()
            .iter()
            .all(|&(px, py)| !field[(y + py) as usize][(x + px) as usize])
    }

    pub(crate) fn can_drop(self, (x, y): (u32, u32), field: &[[bool; 7]]) -> bool {
        y > 0 && self.is_clear((x, y - 1), field)
    }

    pub fn try_jet(self, j: char, (x, y): (u32, u32), field: &[[bool; 7]]) -> (u32, u32) {
        match j {
            '<' => {
                if x != 0 && self.is_clear((x - 1, y), field) {
                    (x - 1, y)
                } else {
                    (x, y)
                }
            }
            '>' => {
                if x + self.bound().0 < 7 && self.is_clear((x + 1, y), field) {
                    (x + 1, y)
                } else {
                    (x, y)
                }
            }
            _ => panic!("No direction"),
        }
    }

    pub const fn bound(self) -> (u32, u32) {
        match self {
            Self::HLine => (4, 1),
            Self::Cross | Self::L => (3, 3),
            Self::VLine => (1, 4),
            Self::Square => (2, 2),
        }
    }
}

pub fn solve_part_a() -> Result<u32, anyhow::Error> {
    part_a(INPUT)
}

pub fn solve_part_b() -> Result<u64, anyhow::Error> {
    part_b(INPUT)
}

fn part_a(input: &str) -> Result<u32, anyhow::Error> {
    let mut field = Vec::new();
    let mut first_free = 0;

    let mut jets = input.chars().cycle();

    //drop 2022 rocks
    for (_, s) in (0..2022).zip(
        [
            Shape::HLine,
            Shape::Cross,
            Shape::L,
            Shape::VLine,
            Shape::Square,
        ]
        .into_iter()
        .cycle(),
    ) {
        // println!("{} {:?}", i, s);
        let (_, height) = s.bound();

        //init more field lines
        for _ in field.len()..(first_free + 3 + height) as usize {
            field.push([false; 7]);
        }

        let mut loc = (2, first_free + 3);

        for j in &mut jets {
            // do jet
            loc = s.try_jet(j, loc, &field);

            // do drop
            if s.can_drop(loc, &field) {
                loc = (loc.0, loc.1 - 1);
            } else {
                s.write(loc, &mut field);
                first_free = calc_first_free(&field).try_into()?;

                break;
            }
        }
    }
    //measure height

    Ok(calc_first_free(&field).try_into()?)
}

fn calc_first_free(field: &[[bool; 7]]) -> usize {
    field
        .iter()
        .enumerate()
        .rev()
        .find(|(_, &r)| r.into_iter().any(identity))
        .map_or(0, |(i, _)| i + 1)
}

fn part_b(input: &str) -> Result<u64, anyhow::Error> {
    let mut field = Vec::new();
    let mut first_free = 0;

    let mut jets = input.chars().cycle();

    for (_, s) in (0..10_000).zip(
        [
            Shape::HLine,
            Shape::Cross,
            Shape::L,
            Shape::VLine,
            Shape::Square,
        ]
        .into_iter()
        .cycle(),
    ) {
        // println!("{} {:?}", i, s);
        let (_, height) = s.bound();

        //init more field lines
        for _ in field.len()..(first_free + 3 + height) as usize {
            field.push([false; 7]);
        }

        let mut loc = (2, first_free + 3);

        for j in &mut jets {
            // do jet
            loc = s.try_jet(j, loc, &field);

            // do drop
            if s.can_drop(loc, &field) {
                loc = (loc.0, loc.1 - 1);
            } else {
                s.write(loc, &mut field);
                first_free = calc_first_free(&field).try_into()?;

                break;
            }
        }
    }

    find_repeat(&mut field);
    // modify above to provide heights
    // look up block count vs height

    Ok(calc_first_free(&field).try_into()?)
}

fn find_repeat(mut field: &[[bool; 7]]) -> Option<(usize, usize)> {
    for size in 5..10000 {
        for offset in 1..10000 {
            if (0..size).all(|i| field[i + offset] == field[size + i + offset]) {
                println!("size {}, offset {}", size, offset);
                return Some((size, offset));
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = include_str!("../resources/example17.txt");

    #[test]
    fn example_a() -> Result<(), anyhow::Error> {
        assert_eq!(part_a(EXAMPLE).unwrap(), 3068);
        Ok(())
    }

    #[test]
    fn example_b() -> Result<(), anyhow::Error> {
        assert_eq!(part_b(EXAMPLE).unwrap(), 1514285714288);
        Ok(())
    }
}
