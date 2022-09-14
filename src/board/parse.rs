use super::*;
use std::collections::VecDeque;
use std::fmt::Display;
use std::str::FromStr;

impl Display for OctiMove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OctiMove::Arrow(pos, arr) => {
                write!(f, "arr {} {}", pos, arr)
            }
            OctiMove::Move(pos, arrs) => {
                let mut arrs_s = String::new();

                for arr in arrs {
                    arrs_s.push(' ');
                    arrs_s.push_str(&arr.to_string());
                }

                write!(f, "mov {}{}", pos, arrs_s)
            }
        }
    }
}

impl FromStr for OctiMove {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let args: Vec<&str> = s.split(' ').collect();

        if args.is_empty() {
            return Err(String::from("Empty string"));
        };

        match args[0] {
            "arr" => {
                if args.len() != 3 {
                    Err(format!("Invalid number of arguments: {}", args.len()))?;
                }

                Ok(OctiMove::Arrow(args[1].parse()?, args[2].parse()?))
            }
            "mov" => {
                if args.len() < 3 {
                    Err(format!("Invalid number of arguments: {}", args.len()))?;
                }

                let mut arrs = Vec::with_capacity(args.len() - 2);
                for arr in args[2..].iter() {
                    arrs.push(arr.parse()?);
                }

                Ok(OctiMove::Move(args[1].parse()?, arrs))
            }
            _ => Err(format!("Unrecognized move type: {}", args[0])),
        }
    }
}

impl Display for Arrow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value())
    }
}

impl FromStr for Arrow {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = s
            .parse::<usize>()
            .map_err(|_| format!("Invalid value: {}", s))?;
        Arrow::new(value)
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x(), self.y())
    }
}

impl FromStr for Position {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars: VecDeque<_> = s.trim().chars().collect();
        let chars_len = chars.len();

        if chars_len < 5 {
            Err(format!("Position string too short: {}", s))?;
        }

        if chars.pop_front().unwrap() != '(' || chars.pop_back().unwrap() != ')' {
            Err(format!(
                "Position does not start or does not end with parentheses: {}",
                s
            ))?;
        }

        let s: String = chars.into_iter().collect();
        let coords: Vec<_> = s.split(',').collect();

        let x = coords[0]
            .parse::<i32>()
            .map_err(|_| format!("Invalid x: {}", coords[0]))?;
        let y = coords[1]
            .parse::<i32>()
            .map_err(|_| format!("Invalid y: {}", coords[1]))?;

        Ok(Position::new(x, y))
    }
}
