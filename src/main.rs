use crate::masks::{mask_rooks_attack, Side};
use crate::utils::print_bitboard;

mod masks;
mod utils;

type Bitboard = u64;

fn main() {
    println!("Chess Engine for Elephants v.0\n");
    let mut white_pawns_attacks: [Bitboard; 64] = [0u64; 64];
    let mut black_pawns_attacks: [Bitboard; 64] = [0u64; 64];
    let mut knights_attacks: [Bitboard; 64] = [0u64; 64];
    let mut king_attacks: [Bitboard; 64] = [0u64; 64];
    let mut bishops_attacks: [Bitboard; 64] = [0u64;64];

    for i in 0..64 {
        white_pawns_attacks[i] = crate::masks::initialize_pawn_attack_by_square_number(Side::White, i as u8);
        black_pawns_attacks[i] = crate::masks::initialize_pawn_attack_by_square_number(Side::Black, i as u8);
        knights_attacks[i] = crate::masks::attack_knight_by_square_number(i as u8);
        king_attacks[i] = crate::masks::attack_king_by_square_number((i as u8));
        bishops_attacks[i] = crate::masks::mask_bishops_attack(i as u8);
    }

    print_bitboard(mask_rooks_attack(63));
    // for i in 0..64 {
    //     print_bitboard(white_pawns_attacks[i]);
    //     print_bitboard(black_pawns_attacks[i]);
    //     print_bitboard(knights_attacks[i]);
    //     print_bitboard(king_attacks[i]);
    //     print_bitboard(bishops_attacks[i]);
    // }
    //print_bitboard_rank(&[0]); }
}
