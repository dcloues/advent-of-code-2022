use std::{
    error::Error,
    fmt::{Display, Write},
};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Move {
    Left,
    Right,
}

fn rev<T>(mut v: Vec<T>) -> Vec<T> {
    v.reverse();
    v
}

#[derive(Clone)]
struct Piece {
    col: usize,
    width: usize,
    height: usize,
    blocks: Vec<bool>,
}

fn pieces() -> Vec<Piece> {
    let hbar = Piece {
        col: 2,
        width: 4,
        height: 1,
        blocks: rev(vec![false, false, true, true, true, true, false]),
    };

    let plus = Piece {
        col: 2,
        width: 3,
        height: 3,
        blocks: rev(vec![
            false, false, false, true, false, false, false, false, false, true, true, true, false,
            false, false, false, false, true, false, false, false,
        ]),
    };

    let bigl = Piece {
        col: 2,
        width: 3,
        height: 3,
        blocks: rev(vec![
            false, false, true, false, false, false, false, false, false, true, false, false,
            false, false, true, true, true, false, false, false, false,
        ]),
    };

    let vbar = Piece {
        col: 2,
        width: 1,
        height: 4,
        blocks: rev(vec![
            true, false, false, false, false, false, false, true, false, false, false, false,
            false, false, true, false, false, false, false, false, false, true, false, false,
            false, false, false, false,
        ]),
    };

    let block = Piece {
        col: 2,
        width: 2,
        height: 2,
        blocks: rev(vec![
            true, true, false, false, false, false, false, true, true, false, false, false, false,
            false,
        ]),
    };
    vec![hbar, plus, bigl, vbar, block]
}

const BOARD_WIDTH: usize = 7;

fn parse_input(s: &str) -> Result<Vec<Move>> {
    s.chars()
        .map(|c| match c {
            '<' => Ok(Move::Left),
            '>' => Ok(Move::Right),
            _ => Err(format!("invalid move '{c}'").into()),
        })
        .collect::<Result<_>>()
}

impl Piece {
    fn apply_move(&mut self, mv: Move) {
        match mv {
            Move::Left if self.col > 0 => {
                println!("Moving: {mv:?}");
                self.col = self.col - 1;
                self.blocks.rotate_right(1);
            }
            Move::Right if self.col + self.width < BOARD_WIDTH => {
                println!("Moving: {mv:?}");
                self.col = self.col + 1;
                self.blocks.rotate_left(1);
            }
            _ => {}
        }
    }
}

struct Game<'a> {
    pieces: Box<dyn Iterator<Item = Piece> + 'a>,
    moves: Box<dyn Iterator<Item = Move> + 'a>,
    board: Vec<bool>,
    piece_count: i32,
}

impl<'a> Game<'a> {
    fn new(piece_spec: &'a Vec<Piece>, move_spec: &'a Vec<Move>) -> Self {
        Self {
            pieces: Box::new(piece_spec.iter().cloned()),
            moves: Box::new(move_spec.iter().cloned()),
            board: vec![],
            piece_count: 0,
        }
    }
    fn run_piece(&mut self) {
        self.piece_count += 1;
        println!("---- spawning new piece ----");
        let mut piece = self.pieces.next().unwrap().clone();
        self.expand_board(piece.height);
        let mut piece_row = self.block_height() + 3;
        println!("  set piece_row={piece_row}");

        self.print_board(&piece, piece_row);

        println!("---- beginning drop loop ----");
        // Piece always starts out three rows above the top of the grid.
        // For these first three moves, we don't need collision checks,
        // because pieces cannot get stuck at this stage.
        for _ in 0..3 {
            // alternate applying gas jets...
            piece.apply_move(self.moves.next().unwrap());
            self.print_board(&piece, piece_row);
            // and applying gravity
            piece_row -= 1;
            self.print_board(&piece, piece_row);
        }

        println!("---- preparing to land ----");

        // Now, we're overlapping with the board, which means pieces
        // can collide with the existing pieces.
        loop {
            self.print_board(&piece, piece_row);
            let mv = self.moves.next().unwrap();
            let mut newpiece = piece.clone();
            newpiece.apply_move(mv);
            if !self.check_collision(&newpiece, piece_row) {
                piece = newpiece;
            }

            self.print_board(&piece, piece_row);

            if piece_row == 0 || self.check_collision(&piece, piece_row - 1) {
                self.land(piece, piece_row);
                break;
            } else {
                piece_row -= 1;
            }

            self.print_board(&piece, piece_row);
        }
    }

    // land a piece at a particular row, copying its blocks into
    // the game grid and dropping the original piece
    fn land(&mut self, piece: Piece, row: usize) {
        let start = row * BOARD_WIDTH;
        for (i, &b) in piece.blocks.iter().enumerate() {
            if b {
                self.board[start + i] = b;
            }
        }
        // self.board
        //     .splice(start..start + piece.blocks.len(), piece.blocks);
    }

    #[must_use]
    fn check_collision(&self, piece: &Piece, row: usize) -> bool {
        let start = row * BOARD_WIDTH;
        piece
            .blocks
            .iter()
            .zip(&self.board[start..])
            .any(|(pb, bb)| *pb && *bb)
    }

    fn expand_board(&mut self, piece: usize) {
        let required = (self.block_height() + 3 + piece) * BOARD_WIDTH;
        println!("resizing board to {required} rows");
        if self.board.len() < required {
            self.board.resize(required, false);
        }
    }

    fn board_height(&self) -> usize {
        self.board.len() / 7
    }

    fn block_height(&self) -> usize {
        if let Some(last_set) = self.board.iter().rev().position(|b| *b) {
            1 + (self.board.len() - last_set) / BOARD_WIDTH
        } else {
            0
        }
        // self.board.len()
        //     - (self
        //         .board
        //         .iter()
        //         .rev()
        //         .position(|p| *p)
        //         .unwrap_or(self.board_height())
        //         / BOARD_WIDTH)
    }

    fn print_board(&mut self, piece: &Piece, row: usize) {
        let restore_board = self.board.clone();
        self.land(piece.clone(), row);
        println!("{self}");
        self.board = restore_board;
    }
}

impl<'a> Display for Game<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for chunk in self.board.chunks(BOARD_WIDTH).rev() {
            f.write_char('|')?;
            for b in chunk.iter().rev() {
                f.write_char(if *b { '#' } else { '.' })?;
            }
            f.write_char('|')?;
            f.write_char('\n')?;
        }
        Ok(())
    }
}

pub fn part1(input: &str) -> Result<String> {
    let moves = parse_input(input)?;

    let all_pieces = pieces();
    let mut game = Game::new(&all_pieces, &moves);

    loop {
        game.run_piece();
        if game.piece_count == 2022 {
            return Ok(game.block_height().to_string());
        }
    }
}

pub fn part2(input: &str) -> Result<String> {
    todo!("unimplemented")
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = include_str!("tests/day17test.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT).unwrap(), "3068")
    }

    fn test_part2() {
        todo!("unimplemented");
        assert_eq!(part2(INPUT).unwrap(), "")
    }
}
