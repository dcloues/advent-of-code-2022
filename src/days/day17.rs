use std::{
    error::Error,
    fmt::{Display, Write},
    time::Instant,
};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Move {
    Left,
    Right,
}

fn rev<T>(mut v: Vec<T>) -> Vec<T> {
    v.reverse();
    v
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum PieceType {
    HBar,
    Plus,
    BigL,
    VBar,
    Block,
}

impl Into<Piece> for PieceType {
    fn into(self) -> Piece {
        match self {
            PieceType::HBar => Piece {
                col: 2,
                width: 4,
                height: 1,
                blocks: rev(vec![false, false, true, true, true, true, false]),
            },
            PieceType::Plus => Piece {
                col: 2,
                width: 3,
                height: 3,
                blocks: rev(vec![
                    false, false, false, true, false, false, false, false, false, true, true, true,
                    false, false, false, false, false, true, false, false, false,
                ]),
            },
            PieceType::BigL => Piece {
                col: 2,
                width: 3,
                height: 3,
                blocks: rev(vec![
                    false, false, false, false, true, false, false, false, false, false, false,
                    true, false, false, false, false, true, true, true, false, false,
                ]),
            },
            PieceType::VBar => Piece {
                col: 2,
                width: 1,
                height: 4,
                blocks: rev(vec![
                    false, false, true, false, false, false, false, false, false, true, false,
                    false, false, false, false, false, true, false, false, false, false, false,
                    false, true, false, false, false, false,
                ]),
            },
            PieceType::Block => Piece {
                col: 2,
                width: 2,
                height: 2,
                blocks: rev(vec![
                    false, false, true, true, false, false, false, false, false, true, true, false,
                    false, false,
                ]),
            },
        }
    }
}

#[derive(Clone)]
struct Piece {
    col: usize,
    width: usize,
    height: usize,
    blocks: Vec<bool>,
}

fn pieces() -> Vec<PieceType> {
    vec![
        PieceType::HBar,
        PieceType::Plus,
        PieceType::BigL,
        PieceType::VBar,
        PieceType::Block,
    ]
}

const BOARD_WIDTH: usize = 7;

fn parse_input(s: &str) -> Result<Vec<Move>> {
    s.trim()
        .chars()
        .map(|c| match c {
            '<' => Ok(Move::Left),
            '>' => Ok(Move::Right),
            _ => Err(format!("invalid move '{c}'").into()),
        })
        .collect::<Result<_>>()
}

impl Piece {
    fn apply_move(&mut self, mv: Move) -> bool {
        match mv {
            Move::Left if self.col > 0 => {
                self.col = self.col - 1;
                self.blocks.rotate_right(1);
                true
            }
            Move::Right if self.col + self.width < BOARD_WIDTH => {
                self.col = self.col + 1;
                self.blocks.rotate_left(1);
                true
            }
            _ => false,
        }
    }
}

struct Game<'a> {
    pieces: Box<dyn Iterator<Item = PieceType> + 'a>,
    moves: Box<dyn Iterator<Item = Move> + 'a>,
    board: Vec<bool>,
    piece_count: i64,
}

impl<'a> Game<'a> {
    fn new(piece_spec: &'a Vec<PieceType>, move_spec: &'a Vec<Move>) -> Self {
        Self {
            pieces: Box::new(piece_spec.iter().cloned().cycle()),
            moves: Box::new(move_spec.iter().cloned().cycle()),
            board: vec![],
            piece_count: 0,
        }
    }
    fn run_piece(&mut self) -> (PieceType, usize) {
        self.piece_count += 1;
        let piece_type = self.pieces.next().unwrap().clone();
        let mut piece: Piece = piece_type.into();
        self.expand_board(piece.height);
        let mut piece_row = self.block_height() + 3;

        // self.print_board(&piece, piece_row, "Spawned new piece");
        // Piece always starts out three rows above the top of the grid.
        // For these first three moves, we don't need collision checks,
        // because pieces cannot get stuck at this stage.
        for _ in 0..3 {
            // alternate applying gas jets...
            let mv = self.moves.next().unwrap();
            #[allow(unused)]
            let moved = piece.apply_move(mv);
            // if moved {
            //     self.print_board(&piece, piece_row, &format!("Applied move {mv:?}"));
            // } else {
            //     self.print_board(&piece, piece_row, &format!("Skipped move {mv:?} (wall)"));
            // }
            // and applying gravity
            piece_row -= 1;
            // self.print_board(&piece, piece_row, "Applied gravity");
        }

        // Now, we're overlapping with the board, which means pieces
        // can collide with the existing pieces.
        loop {
            let mv = self.moves.next().unwrap();
            let mut newpiece = piece.clone();
            newpiece.apply_move(mv.clone());
            if !self.check_collision(&newpiece, piece_row) {
                piece = newpiece;
            }
            // self.print_board(
            //     &piece,
            //     piece_row,
            //     &format!("Applied move {mv:?} w/ collision detection"),
            // );

            if piece_row == 0 || self.check_collision(&piece, piece_row - 1) {
                self.print_board(&piece, piece_row, "Piece landed");
                self.land(piece, piece_row);
                break;
            } else {
                piece_row -= 1;
                // self.print_board(&piece, piece_row, "Applied gravity");
            }
        }

        return (piece_type, self.block_height());
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
        if self.board.len() < required {
            self.board.resize(required, false);
        }
    }

    fn board_height(&self) -> usize {
        self.board.len() / 7
    }

    fn block_height(&self) -> usize {
        if let Some(last_set) = self.board.iter().rev().position(|b| *b) {
            (self.board.len() / BOARD_WIDTH) - (last_set / BOARD_WIDTH)
        } else {
            0
        }
    }

    #[allow(unused)]
    fn print_board(&mut self, piece: &Piece, row: usize, desc: &str) {
        return;
        // ????
        if self.piece_count < 8000 {
            return;
        }
        let restore_board = self.board.clone();
        self.land(piece.clone(), row);
        print!("{}[2J", 27 as char);
        println!("{self}");
        println!(
            "pieces={} board_height={} block_height={}",
            self.piece_count,
            self.board_height(),
            self.block_height()
        );
        println!("> {desc}");
        if self.piece_count > 28 {
            let mut buffer = String::new();
            std::io::stdin().read_line(&mut buffer);
        }
        self.board = restore_board;
    }
}

impl<'a> Display for Game<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (rowinv, chunk) in self.board.chunks(BOARD_WIDTH).enumerate().rev().take(500) {
            f.write_char('|')?;
            for b in chunk.iter().rev() {
                f.write_char(if *b { '#' } else { '.' })?;
            }
            f.write_char('|')?;
            f.write_str(&format!(" {}", rowinv))?;
            f.write_char('\n')?;
        }
        f.write_str("+-------+\n")?;
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
    let moves = parse_input(input)?;

    let all_pieces = pieces();
    let mut game = Game::new(&all_pieces, &moves);

    let start = Instant::now();
    let test_tick = 100000;
    let mut history: Vec<(PieceType, usize)> = Vec::with_capacity(10000);
    let target_pieces: i64 = 1000000000000;
    loop {
        let height = game.block_height();
        let (piecetype, _) = game.run_piece();
        let height_delta = game.block_height() - height;
        history.push((piecetype, height_delta));
        if history.len() > 20000 {
            if let Some(cycle) = find_cycle(&history[history.len() - 10000..], 20) {
                println!(
                    "Found cycle at {} with len {}",
                    history.len() - 10000,
                    cycle.len(),
                );

                let pre_len = history.len() - 10000;
                let cycle_delta: i64 = cycle.iter().map(|(_, dh)| *dh as i64).sum();
                let total_height: i64 = history[0..pre_len].iter().map(|(_, dh)| *dh as i64).sum();

                let repetitions = (target_pieces - pre_len as i64) / cycle.len() as i64;
                let remainder: usize = (target_pieces as usize - pre_len) % cycle.len();
                let remainder_delta: i64 =
                    cycle.iter().take(remainder).map(|(_, dh)| *dh as i64).sum();

                let grand_total = total_height + repetitions * cycle_delta + remainder_delta;
                return Ok(grand_total.to_string());
            }
        }

        if game.piece_count == test_tick {
            let now = Instant::now();
            let elapsed = now - start;
            println!("{test_tick} ticks in {}ms", elapsed.as_millis());
            println!(
                "estimated total runtime: {}s",
                (elapsed * ((1000000000000 / test_tick) as u32)).as_secs()
            );
        }
        if game.piece_count == 1000000000000 {
            return Ok(game.block_height().to_string());
        }
    }
}

fn find_cycle<'a, T>(ts: &'a [T], minlen: usize) -> Option<&'a [T]>
where
    T: PartialEq + std::fmt::Debug,
{
    assert!(minlen > 1, "minimum length must be > 1");
    let maxlen = ts.len() / 2;
    for len in minlen..=maxlen {
        let start = 0;
        if ts[start..start + len] == ts[start + len..start + 2 * len] {
            return Some(&ts[start..start + len]);
        }
    }
    None
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = include_str!("tests/day17test.txt");
    const LINE_BY_LINE: &str = include_str!("tests/day17testlines.txt");

    #[test]
    fn test_find_cycle() {
        assert_eq!(find_cycle(&[1, 2, 3, 4, 5, 6, 7, 8], 2), None);
        assert_eq!(
            find_cycle(&[1, 2, 3, 4, 1, 2, 3, 4], 2).unwrap(),
            &[1, 2, 3, 4][..]
        );
        assert_eq!(find_cycle(&[1, 2, 3, 4, 2, 3, 4], 2), None);
        assert_eq!(
            find_cycle(&[1, 2, 3, 4, 2, 3, 4][1..], 2).unwrap(),
            &[2, 3, 4][..]
        );

        assert_eq!(
            find_cycle(&[1, 2, 3, 4, 3, 4, 2, 3, 4, 3, 4][1..], 5).unwrap(),
            &[2, 3, 4, 3, 4][..]
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT).unwrap(), "3068")
    }

    #[test]
    fn test_part1_line_by_line() {
        let moves = parse_input(INPUT).unwrap();
        let expect: Vec<usize> = LINE_BY_LINE
            .lines()
            .map(|l| l.parse())
            .collect::<std::result::Result<_, _>>()
            .unwrap();

        let all_pieces = pieces();
        let mut game = Game::new(&all_pieces, &moves);

        for height in expect {
            game.run_piece();
            println!(
                "Piece={} expect={} got={}",
                game.piece_count,
                height,
                game.block_height()
            );
            println!("{game}");
            assert_eq!(
                height,
                game.block_height(),
                "at iteration {}",
                game.piece_count
            );
        }
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT).unwrap(), "1514285714288")
    }
}
