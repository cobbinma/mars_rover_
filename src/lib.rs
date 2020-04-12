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
                Command::LeftTurn => rover.turn_left(),
                Command::RightTurn => rover.turn_right(),
                Command::MoveForward => {
                    let planned_coordinates = rover.get_planned_move();
                    plateau.can_rover_move(&planned_coordinates)?;
                    plateau.update_rover_position(rover.get_coordinates(), &planned_coordinates)?;
                    rover.move_rover();
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
        let max_x_grid = args[1].parse::<u64>()?;
        let max_y_grid = args[2].parse::<u64>()?;

        let mut instructions = vec![];

        for i in (3..args.len()).step_by(4) {
            let starting_x = args[i].parse::<u64>()?;
            let starting_y = args[i + 1].parse::<u64>()?;
            let bearing = parse_bearing(
                args[i + 2]
                    .chars()
                    .next()
                    .ok_or_else(|| Box::new(ParseError::new("could not parse bearing")))?,
            )?;
            let commands = parse_commands(args[i + 3].chars().collect())?;
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
enum Command {
    MoveForward,
    RightTurn,
    LeftTurn,
}

fn parse_bearing(c: char) -> Result<rover::Bearing, ParseError> {
    match c {
        'N' => Ok(rover::Bearing::North),
        'E' => Ok(rover::Bearing::East),
        'S' => Ok(rover::Bearing::South),
        'W' => Ok(rover::Bearing::South),
        _ => Err(ParseError::new("could not parse bearing")),
    }
}

fn parse_commands(chars: Vec<char>) -> Result<Vec<Command>, ParseError> {
    if chars.is_empty() {
        return Err(ParseError::new("can't parse empty commands"));
    };
    let commands = chars
        .into_iter()
        .map(map_command)
        .map(|x| x.unwrap())
        .collect();
    Ok(commands)
}

fn map_command(c: char) -> Result<Command, ParseError> {
    match c {
        'M' => Ok(Command::MoveForward),
        'R' => Ok(Command::RightTurn),
        'L' => Ok(Command::LeftTurn),
        _ => Err(ParseError::new("could not parse command")),
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
