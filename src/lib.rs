mod random;

use random::random_range;
use std::{cmp, collections::HashSet};

pub type Position = (usize, usize);

pub enum OpenResult {
    Mine,
    NoMine(u8),
}

#[derive(Debug)]
pub struct Minesweeper {
    width: usize,
    height: usize,
    open_cells: HashSet<Position>,
    mines: HashSet<Position>,
    flagged_cells: HashSet<Position>,
}

impl TryFrom<String> for Minesweeper {
    type Error = String;

    fn try_from(layout: String) -> Result<Self, Self::Error> {
        let mut mines = HashSet::new();

        let (mut width, mut height) = (0, 0);
        let lines = layout.lines().map(|l| l.trim()).filter(|&l| l != "");
        for (i, l) in lines.enumerate() {
            height += 1;

            if width == 0 {
                width = l.len();
            } else {
                if l.len() != width {
                    return Err("all rows must have the same length".to_owned())
                }
            }

            for (j, c) in l.chars().enumerate() {
                if c == 'B' {
                    mines.insert((i, j));
                }
            }
        }

        Ok(Minesweeper { width, height, open_cells: HashSet::new(), mines, flagged_cells: HashSet::new() })
    }
}

impl Minesweeper {
    pub fn new(width: usize, height: usize, mines: usize) -> Self {
        let mut m = Minesweeper {
            width,
            height,
            open_cells: HashSet::new(),
            mines: HashSet::new(),
            flagged_cells: HashSet::new(),
        };

        while m.mines.len() < mines {
            m.mines
                .insert((random_range(0, width), random_range(0, height)));
        }

        m
    }

    pub fn open(&self, pos: Position) -> OpenResult {
        // check if mine
        if self.mines.contains(&pos) {
            OpenResult::Mine
        } else {
            // get all the neighbors
            self.iter_neighbors(pos);
            // count the mines
            OpenResult::NoMine(0)
        }
    }

    fn iter_neighbors(&self, (x, y): Position) -> impl Iterator<Item = Position> + '_ {
        (cmp::max(x, 1) - 1..=cmp::min(x + 1, self.width))
            .flat_map(move |i| {
                (cmp::max(y, 1) - 1..=cmp::min(y + 1, self.height)).map(move |j| (i, j))
            })
            .filter(move |&pos| pos != (x, y))
    }

    fn num_neighbor_mines(&self, (x, y): Position) -> u8 {
        0
    }
}

#[cfg(test)]
mod tests {
    use crate::{Minesweeper, Position};
    use std::collections::HashSet;

    #[test]
    fn test_try_from_string() -> Result<(), String> {
        let map = "
        ##BB#
        B####
        ####B
        ".to_string();

        let ms = Minesweeper::try_from(map)?;
        assert_eq!(5, ms.width);
        assert_eq!(3, ms.height);
        assert_eq!(HashSet::from([(0, 2), (0, 3), (1, 0), (2, 4)]), ms.mines);
        Ok(())
    }

    #[test]
    fn test_try_from_string_inconsistent_width_fails() {
        let map = "
        ##BB##
        ####
        ".to_string();

        assert!(Minesweeper::try_from(map).is_err());
    }

    #[test]
    fn test_new() {
        let m = Minesweeper::new(8, 8, 3);
        println!("{:?}", m);
    }

    // Inspired by the follwing:
    // https://github.com/BurntSushi/fst/blob/master/src/raw/tests.rs#L120-L147
    macro_rules! test_iter_neighbors {
        ($($name:ident, $ms:expr, $pos:expr, $output:expr)*) => {
            $(
                #[test]
                fn $name() {
                    let neighbors: Vec<Position> = $ms.iter_neighbors($pos).collect();
                    assert_eq!($output, neighbors)
                }
            )*
        };
    }

    test_iter_neighbors!(
        neighbors_0_0,
        Minesweeper::new(4, 4, 2),
        (0, 0),
        vec![(0, 1), (1, 0), (1, 1)]
    );

    test_iter_neighbors!(
        neighbors_1_1,
        Minesweeper::new(4, 4, 2),
        (1, 1),
        vec![(0, 0), (0, 1), (0, 2), (1, 0), (1, 2), (2, 0), (2, 1), (2, 2)]
    );
}
