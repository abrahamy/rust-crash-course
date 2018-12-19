use std::fmt::{Display, Formatter};

pub enum VerticalDirection {
    Up,
    Down,
}

pub enum HorizontalDirection {
    Left,
    Right,
}

pub struct Position {
    x: u32,
    y: u32,
}

impl Position {
    fn new(x: u32, y: u32) -> Position {
        Position { x, y }
    }
}

pub struct Ball {
    pos: Position,
    vd: VerticalDirection,
    hd: HorizontalDirection,
}

impl Ball {
    fn new(pos: Position, vd: VerticalDirection, hd: HorizontalDirection) -> Ball {
        Ball { pos, vd, hd }
    }

    fn bounce(&mut self, frame: &Frame) {
        if self.pos.x == 0 {
            self.hd = HorizontalDirection::Right;
        } else if self.pos.x == frame.width - 1 {
            self.hd = HorizontalDirection::Left;
        }

        if self.pos.y == 0 {
            self.vd = VerticalDirection::Up;
        } else if self.pos.y == frame.height - 1 {
            self.vd = VerticalDirection::Down;
        }
    }

    fn mv(&mut self) {
        match self.hd {
            HorizontalDirection::Left => self.pos.x -= 1,
            HorizontalDirection::Right => self.pos.x += 1,
        }

        match self.vd {
            VerticalDirection::Down => self.pos.y -= 1,
            VerticalDirection::Up => self.pos.y += 1,
        }
    }
}

pub struct Frame {
    height: u32,
    width: u32,
}

impl Frame {
    fn new(height: u32, width: u32) -> Frame {
        Frame { height, width }
    }
}

pub struct Game {
    frame: Frame,
    ball: Ball,
}

impl Game {
    pub fn new() -> Game {
        Game {
            frame: Frame::new(20, 120),
            ball: Ball::new(
                Position::new(0, 0),
                VerticalDirection::Up,
                HorizontalDirection::Right,
            ),
        }
    }

    pub fn step(&mut self) {
        self.ball.bounce(&self.frame);
        self.ball.mv();
    }
}

impl Display for Game {
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
        let top_bottom = |fmt: &mut Formatter| {
            write!(fmt, "+")?;
            for _ in 0..self.frame.width {
                write!(fmt, "-")?;
            }
            writeln!(fmt, "+")
        };
        let print_row = |row: u32, fmt: &mut Formatter| {
            write!(fmt, "|").is_ok();

            for column in 0..self.frame.width {
                let c = if row == self.ball.pos.y && column == self.ball.pos.x {
                    'o'
                } else {
                    ' '
                };
                write!(fmt, "{}", c).is_ok();
            }

            writeln!(fmt, "|").is_ok();
        };

        top_bottom(fmt)?;

        for row in 0..self.frame.height {
            print_row(row, fmt);
        }

        top_bottom(fmt)
    }
}
