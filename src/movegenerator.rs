//bitboards for afile and hfile
pub const bbafile: u64 = 72340172838076673;
pub const bbhfile: u64 = 9259542123273814144;
pub const notafile: u64 = !bbafile;
pub const nothfile: u64 = !bbhfile;

pub fn kingmoves(kingpos: u64, ownside: u64) -> u64 {
    let kingpos_clip_file_h = kingpos & nothfile;
    let kingpos_clip_file_a = kingpos & notafile;
    let spot_1: u64 = kingpos_clip_file_a << 7;
    let spot_2: u64 = kingpos << 6;
    let spot_3: u64 = kingpos_clip_file_h << 5;
    let spot_4: u64 = kingpos_clip_file_h << 1;
    let spot_5: u64 = kingpos_clip_file_h >> 7;
    let spot_6: u64 = kingpos >> 8;
    let spot_7: u64 = kingpos_clip_file_a >> 9;
    let spot_8: u64 = kingpos_clip_file_a >> 1;
    
    let king_moves: u64 = spot_1 | spot_2 | spot_3 | spot_4 | spot_5 | spot_6 | spot_7 | spot_8;

    let king_valid: u64 = king_moves & !ownside;

    king_valid
}