mod bitboard;
mod movegenerator;

fn main() {
    let mn_board = bitboard::Board {
        wpieces : [16, 8, 1,128 ,66, 36, 65280],
        bpieces : [1152921504606846976, 9223372036854775808, 72057594037927936,2594073385365405696, 4755801206503243776, 9295429630892703744, 71776119061217280],
        side : true,
        castling : 0b11000011,
    };
    let wpiecesbb = mn_board.concat_wpieces();
    let bpiecesbb = mn_board.concat_bpieces();
    let apiecesbb = mn_board.concat_apieces();

    bitboard::print_bitboard(movegenerator::bpawnmv(mn_board.bpieces[bitboard::Pieces::PAWN], wpiecesbb));
}

fn black_castling(kingpos: u64, castling_rights: u8, ownside: u64) -> u64 {
    if castling_rights & 0b00000010 == 0b00000010 {
        //todo
    } else if castling_rights & 0b00000001 == 0b00000001 {
        //todo
    }

}

fn white_castling(kingpos: u64, castling_rights: u8, ownside: u64) -> u64 {
    if castling_rights & 0b10000000 == 0b10000000 {
        //todo
    }
    if castling_rights & 0b01000000 == 0b010000000{
        //todo
    }
}