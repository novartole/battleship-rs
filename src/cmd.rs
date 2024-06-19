pub enum Cmd {
    Attack(usize, usize),
    Exit,
}

impl TryFrom<&str> for Cmd {
    type Error = anyhow::Error;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        if input == "exit" {
            return Ok(Cmd::Exit);
        } else if input.len() != 2 {
            anyhow::bail!("undefined command");
        }

        let bytes = input.as_bytes();

        let y = if bytes[0].is_ascii_lowercase() {
            (bytes[0] - b'a') as usize
        } else {
            anyhow::bail!("2nd coord must be an ASCII letter");
        };

        let x = if bytes[1].is_ascii_digit() {
            (bytes[1] - b'0') as usize
        } else {
            anyhow::bail!("1st coord must be a number");
        };

        Ok(Cmd::Attack(x, y))
    }
}
