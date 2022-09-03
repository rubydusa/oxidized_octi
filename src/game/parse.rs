use super::super::board::OctiMove;
use super::Action;

use std::fmt::Display;
use std::str::FromStr;

// impl Display for Action

impl FromStr for Action {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let args: Vec<&str> = s.split(' ').collect();

        if args.is_empty() {
            return Err(String::from("Empty string"));
        };

        match args[0] {
            "start" => Ok(Action::Start),
            "end" => Ok(Action::End),
            "forward" => {
                if args.len() != 2 {
                    Err(format!("Invalid number of arguments: {}", args.len()))?;
                }

                Ok(Action::Forward(
                    args[1].parse().or(Err("Invalid forward argument"))?,
                ))
            }
            "backward" => {
                if args.len() != 2 {
                    Err(format!("Invalid number of arguments: {}", args.len()))?;
                }

                Ok(Action::Forward(
                    args[1].parse().or(Err("Invalid backward argument"))?,
                ))
            }
            "move" => Ok(Action::OctiMove(args[1..].join(" ").parse::<OctiMove>()?)),
            _ => Err(format!("Unrecognized move type: {}", args[0])),
        }
    }
}
