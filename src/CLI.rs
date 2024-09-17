use std::io::{self, Write};

use crate::{Board, Chess, File};

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
            "white" => println!("{}", chess.board.white),
            "black" => println!("{}", chess.board.black),
            _ => {}
        }
    }
}

pub fn notation_to_index(move_notation: &str) -> i8 {
    let lowercase = move_notation.to_lowercase();
    let mut chars = lowercase.chars();
    let file = (chars.next().unwrap() as u8) - b'a';
    let rank = (chars.next().unwrap() as u8) - b'1';
    (rank * 8 + file) as i8
}
