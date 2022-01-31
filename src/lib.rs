#[macro_use]
mod loading_macros;
use colored_truecolor::*;

pub struct Peice {
    color: Color,
    style: Type,
}

pub enum Color {
    White,
    Black,
}

enum Type {
    Pawn,
    Rook,
    Knight,
    Bishiop,
    Queen,
    King,
}

pub struct Game {
    board: [[Option<Peice>;8];8],
//  turn: Color,
}

impl Game {
    pub fn new() -> Game {
        use Type::*;
        Game {
            board:
                [[b!(Rook), b!(Knight), b!(Bishiop), b!(Queen), b!(King), b!(Bishiop), b!(Knight), b!(Rook)],
                [b!(Pawn), b!(Pawn), b!(Pawn), b!(Pawn), b!(Pawn), b!(Pawn), b!(Pawn), b!(Pawn)],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [w!(Pawn), w!(Pawn), w!(Pawn), w!(Pawn), w!(Pawn), w!(Pawn), w!(Pawn), w!(Pawn)],
                [w!(Rook), w!(Knight), w!(Bishiop), w!(Queen), w!(King), w!(Bishiop), w!(Knight), w!(Rook)]],
//              turn: Color::White,
        }
    }

    pub fn draw(&self) {
        for (i, line) in (&self.board).into_iter().enumerate() {
            print!("{} ", 8 - i);
            for (k, peice) in line.into_iter().enumerate() {
                if let Some(item) = peice {
                    use Type::*;
                    let peice = match item.style {
                        Pawn => "♟ ",
                        Rook => "♜ ",
                        Knight => "♞ ",
                        Bishiop => "♝ ",
                        Queen => "♛ ",
                        King => "♚ ",
                    };

                    let peice = match item.color {
                        Color::White => peice.bright_white(),
                        Color::Black => peice.bright_black(),
                    }; 
                    let peice = if (i + k) % 2 == 1 {
                        peice.on_black()
                    } else {
                        peice.on_bright_green()
                    };

                    print!("{}", peice);
                } else {
                    let peice = "  ";
                    let peice = if (i + k) % 2 == 1 {
                        peice.on_black()
                    } else {
                        peice.on_bright_green()
                    };

                    print!("{}", peice);
                }
            }
            println!();
        }
        println!("  A B C D E F G H");
    }
}
