pub mod brute_force;

use crate::{field::Cell, ship::Ship};

/// Any algorithm that tries to put Ship into Field should implement this trait.
///
pub trait Seed {
    /// Return *head* and *tail* of a fit line for provided Ship.
    /// If such line cannot be contructed, return *None*.
    ///
    fn get_fit_line(ship: &Ship, matrix: &[Vec<Cell>]) -> Option<((usize, usize), (usize, usize))>;
}
