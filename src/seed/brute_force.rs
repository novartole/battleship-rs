use rand::Rng;

use super::Seed;
use crate::{field::Cell, ship::Ship};
use std::borrow::Cow;

pub struct BruteForce;

impl BruteForce {
    fn get_longest_subline(line: Cow<'_, [Cell]>, c: Cell) -> Option<(usize, usize)> {
        let mut res = None;
        let mut start = None;
        let mut len = 0;

        for (id, val) in line.iter().enumerate() {
            if c == *val {
                len += 1;

                let start_id = if let Some(start) = start {
                    start
                } else {
                    start = Some(id);
                    id
                };

                if res.is_none() {
                    res = Some((start_id, len));
                } else if let Some((res_id, res_len)) = res.as_mut() {
                    if *res_len < len {
                        *res_id = start_id;
                        *res_len = len;
                    }
                }
            } else {
                len = 0;
                start = None;
            }
        }

        res
    }
}

impl Seed for BruteForce {
    fn get_fit_line(ship: &Ship, matrix: &[Vec<Cell>]) -> Option<((usize, usize), (usize, usize))> {
        use rand::seq::SliceRandom;

        let mut rnd = rand::thread_rng();
        let mut dirs = Vec::with_capacity(matrix.len() * 2);

        for i in 0..matrix.len() {
            dirs.push((0, i));
            dirs.push((1, i));
        }

        dirs.shuffle(&mut rnd);

        while let Some((dir, id)) = dirs.pop() {
            let line = if dir == 0 {
                matrix[id].as_slice().into()
            } else {
                (0..matrix.len())
                    .map(|x| matrix[x][id])
                    .collect::<Vec<_>>()
                    .into()
            };

            let (mut start, len) = match Self::get_longest_subline(line, Cell::Empty) {
                Some(subline) => subline,
                None => continue,
            };

            let occupied = match ship.get_fit_len(len) {
                Some(len) => len,
                None => continue,
            };

            let delta = len - occupied;

            if delta > 0 {
                start = rnd.gen_range(start..=start + delta);
            }

            let res = if dir == 0 {
                let x = (id, start);
                let y = (id, start + occupied);

                (x, y)
            } else {
                let x = (start, id);
                let y = (start + occupied, id);

                (x, y)
            };

            return Some(res);
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use crate::{field::Cell, seed::Seed, ship::Ship};

    use super::BruteForce;

    #[test]
    fn fit_empty_matrix() {
        let ship = Ship::new(4);
        let matrix = vec![vec![Cell::Empty; 10]; 10];

        assert!(BruteForce::get_fit_line(&ship, &matrix).is_some());
    }

    #[test]
    fn fit_full_matrix() {
        let ship = Ship::new(4);
        let matrix = vec![vec![Cell::Dummy; 10]; 10];

        assert!(BruteForce::get_fit_line(&ship, &matrix).is_none());
    }

    #[test]
    fn fit_exact_first_row() {
        let x = Cell::Dummy;
        let empty = Cell::Empty;

        let matrix = vec![
            vec![x, x, x, x, x, empty, empty, empty, empty, x],
            vec![x; 10],
            vec![x; 10],
            vec![x; 10],
            vec![x; 10],
            vec![x; 10],
            vec![x; 10],
            vec![x; 10],
            vec![x; 10],
            vec![x; 10],
        ];

        let ship = Ship::new(4);

        assert!(BruteForce::get_fit_line(&ship, &matrix).is_some());
    }

    #[test]
    fn fit_exact_first_col() {
        let x = Cell::Dummy;
        let empty = Cell::Empty;

        let matrix = vec![
            vec![empty, x, x, x, x, x, x, x, x, x],
            vec![empty, x, x, x, x, x, x, x, x, x],
            vec![empty, x, x, x, x, x, x, x, x, x],
            vec![empty, x, x, x, x, x, x, x, x, x],
            vec![x; 10],
            vec![x; 10],
            vec![x; 10],
            vec![x; 10],
            vec![x; 10],
            vec![x; 10],
        ];

        let ship = Ship::new(3);

        assert!(BruteForce::get_fit_line(&ship, &matrix).is_some());
    }

    #[test]
    fn longest_6_3() {
        let x = Cell::Dummy;
        let o = Cell::Empty;

        let line = [x, o, x, x, o, o, x, x, x, o, o, o, x, x, o, o, o, o, o]
            .into_iter()
            .collect();

        assert_eq!(BruteForce::get_longest_subline(line, x), Some((6, 3)));
    }

    #[test]
    fn longest_14_5() {
        let x = Cell::Dummy;
        let o = Cell::Empty;

        let line = [x, o, x, x, o, o, x, x, x, o, o, o, x, x, o, o, o, o, o]
            .into_iter()
            .collect();

        assert_eq!(BruteForce::get_longest_subline(line, o), Some((14, 5)));
    }

    #[test]
    fn longest_whole() {
        let x = Cell::Dummy;
        let line = [x, x, x, x, x, x, x, x].into_iter().collect();

        assert_eq!(BruteForce::get_longest_subline(line, x), Some((0, 8)));
    }

    #[test]
    fn no_longest() {
        let x = Cell::Dummy;
        let o = Cell::Empty;
        let line = [x, x, x, x, x, x, x, x].into_iter().collect();

        assert_eq!(BruteForce::get_longest_subline(line, o), None);
    }
}
