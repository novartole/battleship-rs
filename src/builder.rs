use crate::{field::Field, seed::Seed, ship::Ship};

/// Create a classic *10x10* field and place *10* ships: 1 of size 4, 2 of size 3, 3 of size 4, and 4 of size 1.
/// *S* is a seed strategy to fill out the result field with ships.
///
pub fn build_classic_field<S: Seed>() -> anyhow::Result<Field<S>> {
    let mut ships = Vec::with_capacity(10);

    for (size, count) in (1..=4).rev().zip(1..=4) {
        for _ in 0..count {
            ships.push(Ship::new(size));
        }
    }

    Field::try_create(10, ships)
}
