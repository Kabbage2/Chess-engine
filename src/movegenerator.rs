//bitboards for afile and hfile
pub const bbafile: u64 = 72340172838076673;
pub const bbhfile: u64 = 9259542123273814144;
pub const bbgfile: u64 = 4629771061636907072;
pub const bbbfile: u64 = 144680345676153346;
pub const 8rank: u64 = 18374686479671623680;
pub const 1rank: u64 = 255;
pub const !1rank: u64 = !1rank;
pub const not8rank: u64 = !8rank;
pub const notafile: u64 = !bbafile;
pub const nothfile: u64 = !bbhfile;
pub const bbabfile: u64 = bbafile | bbbfile;
pub const bbghfile: u64 = bbgfile | bbhfile;
pub const notabfile: u64 = !bbabfile;
pub const notghfile: u64 = !bbghfile;

pub fn kingmoves(kingpos: u64, ownside: u64) -> u64 {
    let kingpos_clip_file_h = kingpos & &nothfile;
    let kingpos_clip_file_a = kingpos & &notafile;

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

pub fn knightmoves(knpos: u64, ownside: u64) -> u64 {
    let knpos_clip_fileab = knpos & &notabfile;
    let knpos_clip_filegh = knpos & &notghfile;
    let knpos_clip_filea = knpos & &notafile;
    let knpos_clip_fileh = knpos & &nothfile;

    let spot_1: u64 = knpos_clip_fileab << 6;
    let spot_2: u64 = knpos_clip_filea << 15;
    let spot_3: u64 = knpos_clip_fileh << 17;
    let spot_4: u64 = knpos_clip_filegh << 10;
    let spot_5: u64 = knpos_clip_filegh >> 6;
    let spot_6: u64 = knpos_clip_fileh >> 15;
    let spot_7: u64 = knpos_clip_filea >> 17;
    let spot_8: u64 = knpos_clip_fileab >> 10;

    let posmov: u64 = spot_1 | spot_2 | spot_3 | spot_4 | spot_5 | spot_6 | spot_7 | spot_8;
    let knight_valid = posmov & !ownside;
    
    knight_valid
}

pub fn wpawnmv (pawnpos: u64, bpieces: u64) -> u64 {
    let mask_rank_3: u64 = 16711680;
    let wpawn_one_step: u64 = pawnpos << 8;
    let wpawn_two_step: u64 = (wpawn_one_step & mask_rank_3) << 8;
    let wpawn_valid = wpawn_one_step | wpawn_two_step; 
    let wpawn_left_attack = (pawnpos & &notafile) << 7;
    let wpawn_right_attack = (pawnpos & &nothfile) >> 7;
    let wpawn_attacks = wpawn_left_attack | wpawn_right_attack;
    let wpawn_valid_attacks = wpawn_attacks & bpieces;
    let wpawn_valid = wpawn_valid | wpawn_valid_attacks;
    wpawn_valid
}

pub fn bpawnmv (pawnpos: u64, wpieces: u64) -> u64 {
    let mask_rank_7: u64 = 71776119061217280;
    let bpawn_one_step: u64 = pawnpos >> 8;
    let bpawn_two_step: u64 = (bpawn_one_step & mask_rank_7) >> 8;
    let bpawn_valid = bpawn_one_step | bpawn_two_step;
    let bpawn_left_attack = (pawnpos & &notafile) >> 7;
    let bpawn_right_attack = (pawnpos & &nothfile) << 7;
    let bpawn_attacks = bpawn_left_attack | bpawn_right_attack;
    let bpawn_valid_attacks = bpawn_attacks & wpieces;
    let bpawn_valid = bpawn_valid | bpawn_valid_attacks;
    bpawn_valid
}

pub fn wrookmv (rookpos: u64, apieces: u64) -> u64 {
    let upmv u64;
    while (rookpos & &not8rank)  != 0 {
        if (rookpos >> 8) + apieces == (rookpos >> 8) ^ apieces {
            let upmv = upmv | (rookpos >> 8);
        } else {
            break;
        }
    }
    let dwnmv u64;
    while (rookpos & &not1rank) != 0 {
        if (rookpos << 8) + apieces == (rookpos << 8) ^ apieces {
            let downmv = downmv | (rookpos << 8);
        } else {
            break;
        }
    }
    let rtmv u64;
    while (rookpos & &nothfile) != 0 {
        if (rookpos > 1) + apieces == (rookpos > 1) ^ apieces {
            let rtmv  = rtmv | (rookpos > 1); 
        } else {
            break;
        }
    }
    let lftmv u64;
    while (rookpos & &notafile) != 0 {
        if (rookpos < 1) + apieces == (rookpos < 1) ^ apieces {
            let lftmv = lftmv | (rookpos < 1);
        } else {
            break;
        }
    }
    let posmov u64 = upmv | dwnmv | rtmv | lftmv;
    posmov
}