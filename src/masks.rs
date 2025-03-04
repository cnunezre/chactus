use crate::{Bitboard};
use crate::utils::{get_square_number, print_bitboard, set_bit_active};

pub enum Side {
    White,
    Black,
}

const FIRST_SECOND_RANK: Bitboard = 65535;
const SEVENTH_EIGHTH_RANK: Bitboard = 18446462598732840960;

const AB_FILES: Bitboard = 217020518514230019;

const GH_FILES: Bitboard = 13889313184910721216;

const EIGHT_RANK: Bitboard = 18374686479671623680;
const FIRST_RANK: Bitboard = 255;
const A_FILE: Bitboard = 72340172838076673;
const H_FILE: Bitboard = 9259542123273814144;

// The board
// 8 |0 0 0 0 0 0 0 0=63
// 7 |0 0 0 0 0 0 0 0
// 6 |0 0 0 0 0 0 0 0
// 5 |0 0 0 0 0 0 0 0
// 4 |0 0 0 0 0 0 0 0
// 3 |0 0 0 0 0 0 0 0
// 2 |0=8 0 0 0 0 0 0 0
// 1 |0=0 0 0 0 0 0 0 0=7
// ---------------
// a b c d e f g h

//Knights attacks
pub fn attack_knight_by_square_number(square_number: u8) -> Bitboard {
    let board: Bitboard = set_bit_active(0u64, square_number);

    let mut attacks = 0u64;
    // println!("square: {}", square_number);
    //print_bitboard(board);

    //top 2
    if (board & SEVENTH_EIGHTH_RANK) == 0 {
        //top left
        if (board & A_FILE) == 0 {
            attacks = attacks | (board << 15);
        }

        //top right
        if (board & H_FILE) == 0 {
            attacks = attacks | (board << 17);
        }
    }

    // middle top left
    if (board & AB_FILES) == 0 {
        attacks = attacks | (board << 6);
    }

    //middle top right
    if (board & GH_FILES) == 0 {
        attacks = attacks | (board << 10);
    }

    //middle bottom right
    if (board & GH_FILES) == 0 {
        attacks = attacks | (board >> 6);
    }

    //middle bottom left
    if (board & AB_FILES) == 0 {
        attacks = attacks | (board >> 10);
    }


    //bottom
    if ((board & FIRST_SECOND_RANK) == 0) {
        //bottom left
        if (board & A_FILE) == 0 {
            attacks = attacks | (board >> 17);
        }

        //bottom right
        if (board & H_FILE) == 0 {
            attacks = attacks | (board >> 15);
        }
    }
    attacks
}

pub fn attack_king_by_square_number(square_number: u8) -> Bitboard {
    let board: Bitboard = set_bit_active(0u64, square_number);

    let mut attacks = 0u64;
    // println!("square: {}", square_number);
    // print_bitboard(board);

    //top
    if (board & A_FILE) == 0 && (board & EIGHT_RANK) == 0 {
        attacks = attacks | (board << 7);
    }

    if (board & EIGHT_RANK) == 0 {
        attacks = attacks | (board << 8);
    }

    if (board & EIGHT_RANK == 0) && (board & H_FILE == 0) {
        attacks = attacks | (board << 9);
    }

    //Middle
    if (board & H_FILE) == 0 {
        attacks = attacks | (board << 1);
    }

    if (board & A_FILE) == 0 {
        attacks = attacks | (board >> 1);
    }

    //Bottom
    if (board & A_FILE) == 0 && (board & FIRST_RANK) == 0 {
        attacks = attacks | (board >> 9);
    }

    if (board & FIRST_RANK) == 0 {
        attacks = attacks | (board >> 8);
    }
    //
    if (board & FIRST_RANK == 0) && (board & H_FILE == 0) {
        attacks = attacks | (board >> 7);
    }

    attacks
}

pub fn bishops_attacks_on_the_fly(square: u8, blocking_piece: Bitboard) -> Bitboard {
    let mut attacks = 0u64;

    let target_rank: i8 = (square / 8) as i8;
    let target_file: i8 = (square % 8) as i8;

    let mut rank: i8 = 0;
    let mut file: i8 = 0;

    // going right and up
    if target_rank < 7 && target_file < 7 {
        rank = target_rank + 1;
        file = target_file + 1;
    } else {
        rank = target_rank;
        file = target_file;
    }

    while (rank <= 7 && file <= 7) {
        let mask = (1u64 << (rank * 8 + file));
        attacks = attacks | mask;
        if (mask & blocking_piece) == 0 {
            rank += 1;
            file += 1;
        } else { break; }
    }

    //going left and down
    if target_rank > 0 && target_file > 0 {
        rank = target_rank - 1;
        file = target_file - 1;
    } else {
        rank = target_rank;
        file = target_file;
    }

    while (rank >= 0 && file >= 0) {
        let mask = (1u64 << (rank * 8 + file));
        attacks = attacks | mask;
        if (mask & blocking_piece) == 0 {
            rank -= 1;
            file -= 1;
        } else {
            break;
        }
    }

    //going right and down
    if target_rank > 0 && target_file < 7 {
        rank = target_rank - 1;
        file = target_file + 1;
    } else {
        rank = target_rank;
        file = target_file;
    }

    while (rank >= 0 && file <= 7) {
        let mask = (1u64 << (rank * 8 + file));
        attacks = attacks | mask;
        if (mask & blocking_piece) == 0 {
            rank -= 1;
            file += 1;
        } else {
            break;
        }
    }

    //going left and up
    if target_rank < 7 && target_file > 0 {
        rank = target_rank + 1;
        file = target_file - 1;
    } else {
        rank = target_rank;
        file = target_file;
    }

    while (rank <= 7 && file >= 0) {
        let mask = (1u64 << (rank * 8 + file));
        attacks = attacks | mask;
        if (mask & blocking_piece) == 0 {
            rank += 1;
            file -= 1;
        } else {
            break;
        }
    }

    attacks
}


pub fn mask_bishops_attack(square: u8) -> Bitboard {
    let mut attacks = 0u64;

    let target_rank: u8 = square / 8;
    let target_file: u8 = square % 8;

    let mut rank: u8 = 0;
    let mut file: u8 = 0;

    // going right and up
    if target_rank < 7 && target_file < 7 {
        rank = target_rank + 1;
        file = target_file + 1;
    } else {
        rank = target_rank;
        file = target_file;
    }

    while (rank < 7 && file < 7) {
        attacks = attacks | (1u64 << (rank * 8 + file));
        rank += 1;
        file += 1;
    }

    //going left and down
    if target_rank > 0 && target_file > 0 {
        rank = target_rank - 1;
        file = target_file - 1;
    } else {
        rank = target_rank;
        file = target_file;
    }


    while (rank > 0 && file > 0) {
        attacks = attacks | (1u64 << (rank * 8 + file));
        rank -= 1;
        file -= 1;
    }

    //going right and down
    if target_rank > 0 && target_file < 7 {
        rank = target_rank - 1;
        file = target_file + 1;
    } else {
        rank = target_rank;
        file = target_file;
    }

    while (rank > 0 && file < 7) {
        attacks = attacks | (1u64 << (rank * 8 + file));
        rank -= 1;
        file += 1;
    }

    //going left and up
    if target_rank < 7 && target_file > 0 {
        rank = target_rank + 1;
        file = target_file - 1;
    } else {
        rank = target_rank;
        file = target_file;
    }


    while (rank < 7 && file > 0) {
        attacks = attacks | (1u64 << (rank * 8 + file));
        rank += 1;
        file -= 1;
    }

    attacks
}

pub fn mask_rooks_attack(square: u8) -> Bitboard {
    let mut attacks = 0u64;

    // println!("square: {}", square);
    // let board: Bitboard = 0;
    // print_bitboard(set_bit_active(board, square));

    let target_rank: u8 = square / 8;
    let target_file: u8 = square % 8;

    // Going up
    let mut rank = if target_rank < 7 { target_rank + 1 } else { target_rank };
    while (rank < 7) {
        attacks = attacks | (1u64 << (rank * 8 + target_file));
        rank += 1;
    }

    //Going down
    rank = if target_rank > 0 { target_rank - 1 } else { target_rank };
    while (rank > 0) {
        attacks = attacks | (1u64 << (rank * 8 + target_file));
        rank -= 1;
    }


    //going left
    let mut file: u8 = if (target_file > 0) { target_file - 1 } else { target_file };
    while (file > 0) {
        attacks = attacks | (1u64 << (target_rank * 8 + file));
        file -= 1;
    }

    //going right
    file = if (target_file < 7) { target_file + 1 } else { target_file };
    while (file < 7) {
        attacks = attacks | (1u64 << (target_rank * 8 + file));
        file += 1;
    }

    attacks
}

pub fn rooks_attack_on_the_fly(square: u8, blocking_piece: Bitboard) -> Bitboard {
    let mut attacks = 0u64;

    let target_rank: i8 = (square / 8) as i8;
    let target_file: i8 = (square % 8) as i8;

    // Going up
    let mut rank: i8 = if target_rank < 7 { target_rank + 1 } else { target_rank };
    while (rank <= 7) {
        let mask = (1u64 << (rank * 8 + target_file));
        attacks = attacks | mask;
        if mask & blocking_piece == 0 {
            rank += 1;
        } else {
            break;
        }
    }

    //Going down
    rank = if target_rank > 0 { target_rank - 1 } else { target_rank };
    while (rank >= 0) {
        let mask = (1u64 << (rank * 8 + target_file));
        attacks = attacks | mask;
        if mask & blocking_piece == 0 {
            rank -= 1;
        } else { break; }
    }


    //going left
    let mut file: i8 = if (target_file > 0) { target_file - 1 } else { target_file };
    while (file >= 0) {
        let mask = (1u64 << (target_rank * 8 + file));
        attacks = attacks | mask;
        if mask & blocking_piece == 0 {
            file -= 1;
        } else {
            break;
        }
    }

    //going right
    file = if (target_file < 7) { target_file + 1 } else { target_file };
    while (file <= 7) {
        let mask = (1u64 << (target_rank * 8 + file));
        attacks = attacks | mask;
        if mask & blocking_piece == 0 {
            file += 1;
        } else { break; }
    }

    attacks
}

//Pawn Attacks
pub fn initialize_pawn_attack_by_square_number(side: Side, square_number: u8) -> Bitboard {
    match side {
        Side::White => { attack_pawn_white(square_number) }
        Side::Black => { attack_pawn_black(square_number) }
    }
}
fn initialize_pawn_attacks(square: &str, side: Side) -> Bitboard {
    let square_number = get_square_number(square);
    initialize_pawn_attack_by_square_number(side, square_number)
}

fn attack_pawn_black(square_number: u8) -> Bitboard {
    let board: Bitboard = set_bit_active(0u64, square_number);
    let mut attacks = 0u64;

    // println!("square: {}", square_number);
    // println!("check for not a file H: {}", ((board >> 7) & H_FILE));
    // println!("check for not a file A: {}", ((board >> 9) & A_FILE));

    if ((board >> 9) & H_FILE) == 0 {
        attacks = attacks | (board >> 9);
    }

    if ((board >> 7) & A_FILE) == 0 {
        attacks | (board >> 7)
    } else {
        attacks
    }
}

fn attack_pawn_white(square_number: u8) -> Bitboard {
    let board: Bitboard = set_bit_active(0u64, square_number);
    let mut attacks = 0u64;
    // println!("square: {}", square_number);
    // println!("check for not a file H: {}", ((board << 7) & H_FILE));
    // println!("check for not a file A: {}", ((board << 9) & A_FILE));

    if ((board << 9) & A_FILE) == 0 {
        attacks = attacks | (board << 9);
    }

    if ((board << 7) & H_FILE) == 0 {
        attacks | (board << 7)
    } else {
        attacks
    }
}