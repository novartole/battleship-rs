use crate::{builder, cmd::Cmd, field::FieldError, seed::brute_force::BruteForce};

use std::cell::Cell;

pub struct Game {
    debug: Cell<bool>,
}

impl Game {
    pub fn new() -> Self {
        Self {
            debug: Cell::default(),
        }
    }

    /// Handle external args.
    ///
    pub fn setup(&self) {
        if std::env::args()
            .nth(1)
            .is_some_and(|val| matches!(val.as_str(), "-v" | "--verbose"))
        {
            self.debug.set(true);
        }
    }

    /// Log is active only if -v (or --verbose) argument has been passed.
    ///
    fn log(&self, msg: String) {
        if self.debug.get() {
            println!("{}", msg);
        }
    }

    fn promt() {
        const WELCOME: &str = "Welcome to Battleship!\n
This is a battle againt PC. To start the battle type coordinate in form 'LD' (A2, b8, etc.),\nwhere L is an ASCII letter (upper or lower case) with notion of COL coordinate, and D is a digit - ROW number.\n
To exit type 'exit'.\n
To win the battle distroy all PC's battleships. Good luck!\n";

        println!("{}", WELCOME);
    }

    /// Start main game loop.
    ///
    pub fn start(&self) -> anyhow::Result<()> {
        Self::promt();

        let mut field = builder::build_classic_field::<BruteForce>()?;
        let mut buf = String::new();

        self.log(format!("Init:    \n{}", field));

        loop {
            buf.clear();

            let input = {
                std::io::stdin().read_line(&mut buf)?;
                buf.make_ascii_lowercase();
                buf.trim_end()
            };

            match Cmd::try_from(input) {
                Ok(cmd) => match cmd {
                    Cmd::Attack(x, y) => {
                        match field.attack(x, y) {
                            Ok(res) => println!("{}\n", res),
                            Err(e) => {
                                if let FieldError::OutOfRange(_) = e {
                                    println!("{}", e);
                                    continue;
                                } else {
                                    return Err(e.into());
                                }
                            }
                        }

                        self.log(format!("Last step:   \n{}", field));

                        if !field.any_fine_ship() {
                            println!("you WIN!");
                            break;
                        }
                    }
                    Cmd::Exit => {
                        println!("You lose a battle but not the war!");
                        break;
                    }
                },
                Err(e) => eprintln!("error: {}", e),
            };
        }

        Ok(())
    }
}
