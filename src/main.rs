fn main() {
    let mn_board = Board {
        wpieces : [16, 8, 129, 66, 36, 65280],
        bpieces : [1152921504606846976, 576460752303423488, 2594073385365405696, 4755801206503243776, 9295429630892703744, 71776119061217280],
    };
    print_bitboard(mn_board.wpieces[Pieces::ROOK]);
    println!();
    print_bitboard(mn_board.bpieces[Pieces::KING]);
}

fn print_bitboard(bitboard: u64) {
    const LAST_BIT: u64 = 63;
    for rank in 0..8 {
        for file in (0..8).rev() {
            let mask = 1u64 << (LAST_BIT - (rank * 8) - file);
            let char = if bitboard & mask != 0 {1} else {0};
            print!("{}", char);
        }
        println!();
    }
}

pub struct Board {
    wpieces: [u64; 6],
    bpieces: [u64; 6],
//    turn: bool,
//    epSquare: u64,
//    castling: u8,
}

pub struct Pieces;
impl Pieces {
    pub const KING: usize = 0;
    pub const QUEEN: usize = 1;
    pub const ROOK: usize = 2;
    pub const BISHOP: usize = 3;
    pub const KNIGHT: usize = 4;
    pub const PAWN: usize = 5;
    pub const NONE: usize = 6;
}