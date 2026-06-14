mod bitboard;
mod movegenerator;

fn main() {
    let mn_board = bitboard::Board {
        wpieces : [16, 8, 129, 66, 36, 65280],
        bpieces : [1152921504606846976, 576460752303423488, 2594073385365405696, 4755801206503243776, 9295429630892703744, 71776119061217280],
    };
    let wpiecesbb = mn_board.concat_wpieces();
    let bpiecesbb = mn_board.concat_bpieces();
    let apiecesbb = mn_board.concat_apieces();

    bitboard::print_bitboard(movegenerator::bpawnmv(mn_board.bpieces[bitboard::Pieces::PAWN], wpiecesbb));
}