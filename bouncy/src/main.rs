mod game;
mod args;

fn main() {
    let frame = match args::parse_args() {
        Ok(frame) => frame,
        Err(e) => {
            match e {
                args::ParseError::InvalidInteger(i) => println!("Invalid integer: {}", i),
                args::ParseError::TooFewArgs => println!("Too few arguments, must specify height and width."),
                args::ParseError::TooManyArgs => println!("Too many arguments given.")
            }
            return
        }
    };
    let mut game = game::Game::new(frame);
    let sleep_duration = std::time::Duration::from_millis(33);
    loop {
        println!("{}", game);
        game.step();
        std::thread::sleep(sleep_duration);
    }
}
