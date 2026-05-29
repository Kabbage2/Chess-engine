//function for printing out bitboards
pub fn print_bitboard(bitboard: u64) {
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

//struct to represent board
pub struct Board {
    pub wpieces: [u64; 6],
    pub bpieces: [u64; 6],
//    turn: bool,
//    epSquare: u64,
//    castling: u8,
}

// functions to create bitboards for all the pieces
impl Board{
    pub fn concat_wpieces(&self) -> u64 {
        let wpiecesbb = self.wpieces.iter().fold(0, |acc, &x| acc ^ x);
        wpiecesbb
    }
    pub fn concat_bpieces(&self) -> u64 {
        let bpiecesbb = self.bpieces.iter().fold(0, |acc, &x| acc ^ x);
        bpiecesbb
    }
    pub fn concat_apieces(&self) -> u64 {
        let wpiecesbb = self.wpieces.iter().fold(0, |acc, &x| acc ^ x);
        let bpiecesbb = self.bpieces.iter().fold(0, |acc, &x| acc ^ x);
        let apiecesbb = wpiecesbb ^ bpiecesbb;
        apiecesbb
    }
}

//struct to assign number to each type to make indexing array of pieces easier to read
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