use core::fmt;
use std::error::Error;

pub mod plateau;
pub mod rover;

pub fn deploy_rovers(config: Config) -> Result<Vec<rover::Rover>, Box<dyn Error>> {
    let mut plateau = plateau::Plateau::new(config.max_x_grid, config.max_y_grid);

    let mut rovers = vec![];

    for instruction in config.instructions.iter() {
        let mut rover = rover::Rover::new(
            instruction.starting_x,
            instruction.starting_y,
            instruction.bearing,
        );
        plateau.drop_rover(rover::Coordinates::new(
            instruction.starting_x,
            instruction.starting_y,
        ))?;

        for command in &instruction.commands {
            match command {
                Command::LeftTurn => rover.execute_command(command),
                Command::RightTurn => rover.execute_command(command),
                Command::MoveForward => {
                    let planned_coordinates = rover.planned_move();
                    plateau.can_rover_move(&planned_coordinates)?;
                    plateau.update_rover_position(rover.coordinates(), &planned_coordinates)?;
                    rover.execute_command(command);
                }
            }
        }

        rovers.push(rover);
    }

    Ok(rovers)
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Config {
    max_x_grid: u64,
    max_y_grid: u64,
    instructions: Vec<RoverInstructions>,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, Box<dyn Error>> {
        if args.len() < 7 {
            return Err(Box::new(ParseError::new(
                "can't have less than 6 arguments",
            )));
        };

        let mut args_iter = args.iter();
        args_iter.next();

        let max_x_grid = args_iter
            .next()
            .ok_or("could not read first argument")?
            .parse()?;

        let max_y_grid = args_iter
            .next()
            .ok_or("could not read second argument")?
            .parse()?;

        let mut instructions = vec![];

        for next_rover_args in args_iter.as_slice().chunks_exact(4) {
            let mut iter = next_rover_args.iter();

            let starting_x = iter.next().ok_or("starting x not given")?.parse()?;
            let starting_y = iter.next().ok_or("starting y not given")?.parse()?;
            let bearing = iter.next().ok_or("no bearing given")?.parse()?;
            let commands = iter
                .next()
                .ok_or("no commands given")?
                .chars()
                .map(Command::new)
                .filter_map(Result::ok)
                .collect();
            instructions.push(RoverInstructions::new(
                starting_x, starting_y, bearing, commands,
            ))
        }

        Ok(Config {
            max_x_grid,
            max_y_grid,
            instructions,
        })
    }
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
struct RoverInstructions {
    starting_x: u64,
    starting_y: u64,
    bearing: rover::Bearing,
    commands: Vec<Command>,
}

impl RoverInstructions {
    fn new(
        starting_x: u64,
        starting_y: u64,
        bearing: rover::Bearing,
        commands: Vec<Command>,
    ) -> RoverInstructions {
        RoverInstructions {
            starting_x,
            starting_y,
            bearing,
            commands,
        }
    }
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub enum Command {
    MoveForward,
    RightTurn,
    LeftTurn,
}

impl Command {
    fn new(c: char) -> Result<Command, String> {
        match c {
            'M' => Ok(Command::MoveForward),
            'R' => Ok(Command::RightTurn),
            'L' => Ok(Command::LeftTurn),
            _ => Err(String::from("could not parse command")),
        }
    }
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct ParseError {
    details: String,
}

impl ParseError {
    fn new(msg: &str) -> ParseError {
        ParseError {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for ParseError {
    fn description(&self) -> &str {
        &self.details
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rover::Bearing;

    #[test]
    fn parse_one_rover() {
        let args = vec![
            "test".to_string(),
            "5".to_string(),
            "5".to_string(),
            "3".to_string(),
            "3".to_string(),
            "N".to_string(),
            "MRLM".to_string(),
        ];

        let expected = Config {
            max_x_grid: 5,
            max_y_grid: 5,
            instructions: vec![RoverInstructions {
                starting_x: 3,
                starting_y: 3,
                bearing: Bearing::North,
                commands: vec![
                    Command::MoveForward,
                    Command::RightTurn,
                    Command::LeftTurn,
                    Command::MoveForward,
                ],
            }],
        };

        let config = Config::new(&args).expect("should create config");

        assert_eq!(expected, config);
    }

    #[test]
    fn parse_multiple_rovers() {
        let args = vec![
            "test".to_string(),
            "5".to_string(),
            "5".to_string(),
            "3".to_string(),
            "3".to_string(),
            "N".to_string(),
            "MRLM".to_string(),
            "3".to_string(),
            "3".to_string(),
            "N".to_string(),
            "MRLM".to_string(),
        ];

        let expected = Config {
            max_x_grid: 5,
            max_y_grid: 5,
            instructions: vec![
                RoverInstructions {
                    starting_x: 3,
                    starting_y: 3,
                    bearing: Bearing::North,
                    commands: vec![
                        Command::MoveForward,
                        Command::RightTurn,
                        Command::LeftTurn,
                        Command::MoveForward,
                    ],
                },
                RoverInstructions {
                    starting_x: 3,
                    starting_y: 3,
                    bearing: Bearing::North,
                    commands: vec![
                        Command::MoveForward,
                        Command::RightTurn,
                        Command::LeftTurn,
                        Command::MoveForward,
                    ],
                },
            ],
        };

        let config = Config::new(&args).expect("should create config");

        assert_eq!(expected, config);
    }
}
