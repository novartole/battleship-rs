use crate::{
    seed::Seed,
    ship::{DamageResult, Ship},
};

use std::{
    fmt::{self, Display},
    marker::PhantomData,
};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum FieldError {
    #[error("out of range: coord ids must be less than {0}")]
    OutOfRange(usize),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

#[derive(Clone, PartialEq, Copy)]
pub enum Cell {
    Empty,
    Segment {
        ship_id: usize,
        state: SegmentState,
    },
    #[cfg(test)]
    Dummy,
}

#[derive(Clone, PartialEq, Copy)]
pub enum SegmentState {
    Fine,
    Damaged,
}

impl Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Cell::Empty => String::from("•"),
                Cell::Segment { ship_id, state } => match state {
                    SegmentState::Fine => format!("{}", ship_id),
                    SegmentState::Damaged => String::from("X"),
                },
                #[cfg(test)]
                Cell::Dummy => String::from('T'),
            }
        )
    }
}

pub struct Field<S>
where
    S: Seed,
{
    matrix: Vec<Vec<Cell>>,
    ships: Vec<((usize, usize), Ship)>,
    size: usize,
    // Keep it being PhantomData to staticly use Seed algorithm.
    _seed: PhantomData<S>,
}

impl<S> Field<S>
where
    S: Seed,
{
    pub fn try_create(size: usize, ships: Vec<Ship>) -> anyhow::Result<Self> {
        let mut field = Self {
            matrix: vec![vec![Cell::Empty; size]; size],
            ships: Vec::default(),
            size,

            _seed: PhantomData,
        };

        for ship in ships {
            field.put_ship(ship)?;
        }

        Ok(field)
    }

    pub fn any_fine_ship(&self) -> bool {
        self.ships.iter().any(|(_, ship)| !ship.is_dead())
    }

    pub fn attack(&mut self, x: usize, y: usize) -> Result<DamageResult, FieldError> {
        if x >= self.size || y >= self.size {
            return Err(FieldError::OutOfRange(self.size));
        }

        let (ship_id, ship, (head_x, head_y)) = match self.matrix[x][y] {
            Cell::Segment { ship_id, .. } => {
                let (ship_head, ship) = &mut self.ships[ship_id];
                (ship_id, ship, *ship_head)
            }
            Cell::Empty => return Ok(DamageResult::Miss),
            #[cfg(test)]
            Cell::Dummy => unimplemented!("trying to attack a dummy cell"),
        };

        // cast into u8 is fine since Ship was initialized with size of being u8
        let segment = if head_x == x {
            head_y.abs_diff(y)
        } else if head_y == y {
            head_x.abs_diff(x)
        } else {
            return Err(anyhow::anyhow!(
                "broken field: ship is not along the line: head=({}, {}), coord=({}, {})",
                head_x,
                head_y,
                x,
                y
            )
            .into());
        } as u8;

        let res = ship.damage(segment)?;

        if matches!(res, DamageResult::Hit | DamageResult::Destroyed) {
            self.matrix[x][y] = Cell::Segment {
                ship_id,
                state: SegmentState::Damaged,
            };
        }

        Ok(res)
    }

    fn put_ship(&mut self, ship: Ship) -> anyhow::Result<()> {
        let id = self.ships.len();

        let ((x0, y0), (x1, y1)) =
            S::get_fit_line(&ship, &self.matrix).ok_or(anyhow::anyhow!("field is full"))?;

        let head = if x0 == x1 {
            let (start, end) = (y0, y1).min((y1, y0));

            for y in &mut self.matrix[x0][start..end] {
                *y = Cell::Segment {
                    ship_id: id,
                    state: SegmentState::Fine,
                };
            }

            (x0, start)
        } else if y0 == y1 {
            let (start, end) = (x0, x1).min((x1, x0));

            for x in &mut self.matrix[start..end] {
                x[y0] = Cell::Segment {
                    ship_id: id,
                    state: SegmentState::Fine,
                };
            }

            (start, y0)
        } else {
            anyhow::bail!(
                "failed to build a line from pair ({}, {}) and ({}, {})",
                x0,
                y0,
                x1,
                y1
            );
        };

        self.ships.push((head, ship));

        Ok(())
    }
}

impl<S: Seed> Display for Field<S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use std::iter;

        let mut res = "  "
            .chars()
            .chain('A'..)
            .take(self.matrix.len() + 2)
            .chain(iter::once('\n'))
            .collect::<String>();

        for (id, line) in self.matrix.iter().enumerate() {
            res.push_str(format!("{id} ").as_str());

            for cell in line {
                res.push_str(cell.to_string().as_str());
            }

            if id < self.matrix.len() {
                res.push('\n');
            }
        }

        write!(f, "{}", res)
    }
}

#[cfg(test)]
mod tests {
    use crate::{seed::brute_force::BruteForce, ship::Ship};

    use super::{Cell, Field};

    fn print_matrix(matrix: &[Vec<Cell>]) {
        for row in matrix {
            let mut line = String::new();

            for cell in row {
                line.push_str(cell.to_string().as_str());
            }

            println!("{}", line);
        }
    }

    #[test]
    fn put_1_ship_4() {
        let ship = Ship::new(4);
        let field = match Field::<BruteForce>::try_create(10, vec![ship]) {
            Ok(field) => field,
            Err(_) => panic!("failed to create field"),
        };

        println!("\n");
        print_matrix(&field.matrix);
    }

    #[test]
    fn put_classic() {
        let mut ships = vec![];

        for (size, count) in (1..=4).rev().zip(1..=4) {
            for _ in 0..count {
                ships.push(Ship::new(size));
            }
        }

        let field = match Field::<BruteForce>::try_create(10, ships) {
            Ok(field) => field,
            Err(_) => panic!("failed to create field"),
        };

        println!("\n");
        print_matrix(&field.matrix);
    }

    #[test]
    fn attack_classic_4_head() {
        let mut ships = vec![];

        for (size, count) in (1..=4).rev().zip(1..=4) {
            for _ in 0..count {
                ships.push(Ship::new(size));
            }
        }

        let mut field = match Field::<BruteForce>::try_create(10, ships) {
            Ok(field) => field,
            Err(_) => panic!("failed to create field"),
        };

        println!("\n");
        print_matrix(&field.matrix);

        let ((x, y), _) = field.ships[0];
        assert!(field
            .attack(x, y)
            .is_ok_and(|res| !matches!(res, crate::ship::DamageResult::Miss)));

        println!("\nATTACKED: ({}, {})\n", x, y);
        print_matrix(&field.matrix);

        let ((x, y), _) = field.ships[0];
        assert!(field
            .attack(x, y)
            .is_ok_and(|res| matches!(res, crate::ship::DamageResult::Miss)));
    }
}
