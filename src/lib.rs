pub mod board;
pub mod fen;
pub mod game;
pub mod piece;

use board::{Board, MoveRequest, MoveState};
use game::Game;
use piece::Side;

#[derive(Debug)]
pub struct ParseError(String);

impl ParseError {
    pub fn new(error: &str) -> ParseError {
        ParseError(String::from(error))
    }
}

pub mod game_options {
    pub const MOVE_OPTION: &str = "1";
    pub const PREVIOUS_OPTION: &str = "2";
    pub const NEXT_OPTION: &str = "3";
    pub const RESIGN_OPTION: &str = "4";
    pub const QUIT_OPTION: &str = "5";
}

pub mod post_game_options {
    pub const NEW_GAME_OPTION: &str = "1";
    pub const PREVIOUS_OPTION: &str = "2";
    pub const NEXT_OPTION: &str = "3";
    pub const QUIT_OPTION: &str = "4";
}

pub fn run() {
    let mut game = Game::new(Board::default());

    let mut keep_going = true;
    while keep_going {
        let black_score = game.get_black_score();
        let white_score = game.get_white_score();

        if black_score > white_score {
            let relative_score = black_score - white_score;
            println!("+{relative_score}");
        }
        
        println!("{}", game.get_board());

        if white_score > black_score {
            let relative_score = white_score - black_score;
            println!("+{relative_score}");
        }

        println!("");

        let move_state = board::get_move_state(game.get_board());

        let mut game_over = false;
        match move_state {
            MoveState::CanMove | MoveState::Check => {
                println!(concat!(
                    "Select one of the following options:\n",
                    "1) Move\n",
                    "2) Previous\n",
                    "3) Next\n",
                    "4) Resign\n",
                    "5) Quit\n"
                ));

                println!("Enter choice: ");

                let mut option = String::new();
                std::io::stdin()
                    .read_line(&mut option)
                    .expect("Failed to read stdin.");
                let option = option.trim();

                match option {
                    game_options::MOVE_OPTION => {
                        let mut coordinates = String::new();

                        println!("Enter move: ");

                        std::io::stdin()
                            .read_line(&mut coordinates)
                            .expect("Failed to read stdin.");

                        let coordinates = coordinates.trim();

                        if let Ok(request) = MoveRequest::from_coordinate(coordinates) {
                            if let Err(error) = game.attempt_move(request) {
                                println!("Move Error: {}", error);
                            }
                        }

                        println!("");
                    }
                    game_options::PREVIOUS_OPTION => {
                        game.previous();
                    }
                    game_options::NEXT_OPTION => {
                        game.next();
                    }
                    game_options::RESIGN_OPTION => {
                        let winning_side = match game.get_board().get_current_turn() {
                            Side::White => "black",
                            Side::Black => "white",
                        };
                        println!("Player resigned, {winning_side} won!\n");

                        game_over = true;
                    }
                    game_options::QUIT_OPTION => keep_going = false,
                    _ => (),
                }
            }
            MoveState::Stalemate => {
                println!("The game has ended in a stalemate.\n");

                game_over = true;
            }
            MoveState::Checkmate => {
                let winning_side = match game.get_board().get_current_turn() {
                    Side::White => "black",
                    Side::Black => "white",
                };
                println!("Checkmate, {winning_side} won!\n");

                game_over = true;
            }
        }

        if game_over {
            println!(concat!(
                "Select one of the following options:\n",
                "1) New game\n",
                "2) Previous\n",
                "3) Next\n",
                "4) Quit\n"
            ));

            println!("Enter choice: ");

            let mut option = String::new();
            std::io::stdin()
                .read_line(&mut option)
                .expect("Failed to read stdin.");
            let option = option.trim();

            match option {
                post_game_options::NEW_GAME_OPTION => {
                    game = Game::new(Board::default());
                }
                post_game_options::PREVIOUS_OPTION => {
                    game.previous();
                }
                post_game_options::NEXT_OPTION => {
                    game.next();
                }
                post_game_options::QUIT_OPTION => keep_going = false,
                _ => (),
            }
        }
    }
}

pub fn perform_moves(game: &mut Game, move_requests: Vec<MoveRequest>) {
    println!("{}\n", game.get_board());

    for request in move_requests {
        match game.attempt_move(request) {
            Ok(_) => {
                println!("{}\n", game.get_board());
                println!("{:?}\n", board::get_move_state(game.get_board()));
            }
            Err(error) => {
                println!("{error:?}");
                break;
            }
        }
    }
}
