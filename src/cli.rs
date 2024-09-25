use std::{char, io::{self, Write}, time::{SystemTime, UNIX_EPOCH}};

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

        let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

        match split[0] {
            "fen" => {
                if split.len() <= 1 {
                    continue;
                }
                chess.board = Board::from_fen(split[1..].join(" ")).unwrap()
            },
            "move" | "m" => { 
                if split.len() <= 2 {
                    continue;
                }
                chess.make_move(notation_to_index(split[1]), notation_to_index(split[2]));
            },
            "attacks" | "a" => {
                if split.len() <= 1 {
                    continue;
                }
                let index = notation_to_index(split[1]);

                if let Some(piece) = chess.board.get_piece_at_pos(index) {
                    println!("{}", piece);
                }
                for pmove in chess.get_moves(index) {
                    println!("{}{}", (pmove.0 as u8 + b'A') as char, pmove.1 + 1);
                }
            },
            "pinned" => {
                if split.len() <= 1 {
                    continue;
                }
                match split[1] {
                    "white" | "w" => println!("{}", chess.board.get_side_computed_boards(Side::White).0),
                    "black" | "b" => println!("{}", chess.board.get_side_computed_boards(Side::White).0),
                    _ => {}
                }
            },
            "aa" => {
                if split.len() <= 1 {
                    continue;
                }
                match split[1] {
                    "white" | "w" => println!("{}", chess.board.get_side_computed_boards(Side::White).2),
                    "black" | "b" => println!("{}", chess.board.get_side_computed_boards(Side::Black).2),
                    _ => {}
                }
            }
            "white" => chess.board.print_side(Side::White),
            "black" => chess.board.print_side(Side::Black),
            "state" => println!("{:?}", chess.get_state()),
            "turn" => println!("{:?}", chess.board.get_playing_side()),
            _ => {}
        }

        let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

        println!("Time taken: {}us", (end - start).as_micros());
    }
}

fn notation_to_index(move_notation: &str) -> usize {
    let lowercase = move_notation.to_lowercase();
    let mut chars = lowercase.chars();
    let file = (chars.next().unwrap() as u8) - b'a';
    let rank = (chars.next().unwrap() as u8) - b'1';
    (rank * 8 + file) as usize
}
