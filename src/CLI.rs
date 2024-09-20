use std::io::{self, Write};

use crate::{Board, Chess, File, Side};

pub fn start() {
    let mut chess = Chess::default();
    
    while true {
        let mut input = String::new();

        input.clear();
        print!("> ");
        let _ = io::stdout().flush();
        io::stdin().read_line(&mut input).unwrap();

        let split : Vec<_> = input.trim().split_whitespace().collect();

        if split.len() == 0 {
            continue;
        }

        match split[0] {
            "fen" => chess.board = Board::from_fen(split[1..].join(" ")).unwrap(),
            "move" | "m" => { 
                chess.make_move(notation_to_index(split[1]), notation_to_index(split[2]));
            },
            "attacks" | "a" => {
                let index = notation_to_index(split[1]);

                if let Some(piece) = chess.board.get_piece_at_pos(index) {
                    println!("{}", piece.get_possible_moves(&chess.board));
                }
            },
            "pinned" => {
                match split[1] {
                    "white" | "w" => println!("{}", chess.board.get_side_computed_boards(Side::White).0),
                    "black" | "b" => println!("{}", chess.board.get_side_computed_boards(Side::White).0),
                    _ => {}
                }
            },
            "aa" => {
                match split[1] {
                    "white" | "w" => println!("{}", chess.board.get_side_computed_boards(Side::White).2),
                    "black" | "b" => println!("{}", chess.board.get_side_computed_boards(Side::Black).2),
                    _ => {}
                }
            }
            "white" => chess.board.print_side(Side::White),
            "black" => chess.board.print_side(Side::Black),
            _ => {}
        }
    }
}

pub fn notation_to_index(move_notation: &str) -> usize {
    let lowercase = move_notation.to_lowercase();
    let mut chars = lowercase.chars();
    let file = (chars.next().unwrap() as u8) - b'a';
    let rank = (chars.next().unwrap() as u8) - b'1';
    (rank * 8 + file) as usize
}
