use std::fmt::{self, Display};

#[derive(Debug)]
pub struct Ship {
    size: u8,
    state: u8,
}

pub enum DamageResult {
    Miss,
    Hit,
    Destroyed,
}

impl Display for DamageResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                DamageResult::Miss => "miss..",
                DamageResult::Hit => "HIT!",
                DamageResult::Destroyed => "Destroyed!",
            }
        )
    }
}

impl Ship {
    pub fn new(size: u8) -> Self {
        Self { size, state: 0 }
    }

    pub fn damage(&mut self, segment: u8) -> anyhow::Result<DamageResult> {
        if segment >= self.size {
            anyhow::bail!("ship does not contain {}", segment);
        }

        let mask = 1 << segment;

        if self.state & mask == mask {
            return Ok(DamageResult::Miss);
        }

        self.state |= mask;

        if self.is_dead() {
            Ok(DamageResult::Destroyed)
        } else {
            Ok(DamageResult::Hit)
        }
    }

    /// Check wheather Ship can fit provided *size*.
    /// Return fit value or None otherwise.
    ///
    pub fn get_fit_len(&self, size: usize) -> Option<usize> {
        let ship_size = self.size as usize;

        if ship_size <= size {
            Some(ship_size)
        } else {
            None
        }
    }

    pub fn is_dead(&self) -> bool {
        self.size == self.state.count_ones() as u8
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn damage_existed_seqments() {
        let mut ship = Ship::new(3);

        assert!(ship
            .damage(1)
            .is_ok_and(|res| matches!(res, DamageResult::Hit)));
        assert_eq!(ship.state & 2, 2);
        assert!(!ship.is_dead());

        assert!(ship
            .damage(0)
            .is_ok_and(|res| matches!(res, DamageResult::Hit)));
        assert_eq!(ship.state & 1, 1);
        assert!(!ship.is_dead());

        assert!(ship
            .damage(0)
            .is_ok_and(|res| matches!(res, DamageResult::Miss)));
        assert_eq!(ship.state & 1, 1);
        assert!(!ship.is_dead());

        assert!(ship
            .damage(2)
            .is_ok_and(|res| matches!(res, DamageResult::Destroyed)));
        assert_eq!(ship.state & 4, 4);
        assert!(ship.is_dead());
    }

    #[test]
    fn damage_bad_segment() {
        let mut ship = Ship::new(2);

        assert!(ship.damage(2).is_err());
    }
}
