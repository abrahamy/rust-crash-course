use crate::game::Frame;

#[derive(Debug)]
pub enum ParseError {
    TooFewArgs,
    TooManyArgs,
    InvalidInteger(String),
}

struct ParseArgs(std::env::Args);

impl ParseArgs {
    fn new() -> ParseArgs {
        ParseArgs(std::env::args())
    }


    fn require_arg(&mut self) -> Result<String, ParseError> {
        match self.0.next() {
            None => Err(ParseError::TooFewArgs),
            Some(arg) => Ok(arg),
        }
    }

    fn require_no_args(&mut self) -> Result<(), ParseError> {
        match self.0.next() {
            None => Ok(()),
            Some(_) => Err(ParseError::TooManyArgs)
        }
    }
}

fn parse_u32(int_str: String) -> Result<u32, ParseError> {
    match int_str.parse::<u32>() {
        Err(_) => Err(ParseError::InvalidInteger(int_str)),
        Ok(int) => Ok(int),
    }
}

pub fn parse_args() -> Result<Frame, ParseError> {
    let mut args = ParseArgs::new();

    // skip the command name
    args.require_arg()?;

    let height_str = args.require_arg()?;
    let width_str = args.require_arg()?;
    args.require_no_args()?;
    let height = parse_u32(height_str)?;
    let width = parse_u32(width_str)?;

    Ok(Frame::new(height, width))
}