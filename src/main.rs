use crate::masks::{attack_king_by_square_number, attack_knight_by_square_number, bishops_attacks_on_the_fly, initialize_pawn_attack_by_square_number, mask_rooks_attack, rooks_attack_on_the_fly, Side};
use crate::utils::{count_bits, from_square_to_algebraic_notation, get_lsb_index, get_msb_index, print_bitboard, set_bit_active};

mod masks;
mod utils;

type Bitboard = u64;

fn main() {
    println!("Chess Engine for Elephants v.0\n");
    let mut white_pawns_attacks: [Bitboard; 64] = [0u64; 64];
    let mut black_pawns_attacks: [Bitboard; 64] = [0u64; 64];
    let mut knights_attacks: [Bitboard; 64] = [0u64; 64];
    let mut king_attacks: [Bitboard; 64] = [0u64; 64];
    let mut bishops_attacks: [Bitboard; 64] = [0u64; 64];
    let mut rooks_attacks: [Bitboard; 64] = [0u64; 64];

    // for i in 0..64 {
    //     white_pawns_attacks[i] = initialize_pawn_attack_by_square_number(Side::White, i as u8);
    //     black_pawns_attacks[i] = initialize_pawn_attack_by_square_number(Side::Black, i as u8);
    //     knights_attacks[i] = attack_knight_by_square_number(i as u8);
    //     king_attacks[i] = attack_king_by_square_number((i as u8));
    //     bishops_attacks[i] = mask_bishops_attack(i as u8);
    //     rooks_attacks[i] = mask_rooks_attack(i as u8);
    // }
    let mut block: Bitboard = 0u64;
    block = set_bit_active(block, 30);
    block = set_bit_active(block, 51);
    block = set_bit_active(block, 25);
    block = set_bit_active(block, 11);

    block = set_bit_active(block, 3);
    print_bitboard(block);
    println!("{:064b}", block);
    println!("trailing zeroes: {} leading zeroes: {}", get_lsb_index(block), get_msb_index(block));
    println!("coordinate lsb: {} coordinate msb: {}", from_square_to_algebraic_notation(get_lsb_index(block)), from_square_to_algebraic_notation(get_msb_index(block)));

    let msb = set_bit_active(0u64, get_msb_index(block) as u8);
    let lsb = set_bit_active(0u64, get_lsb_index(block) as u8);

    print_bitboard(msb);
    print_bitboard(lsb);

    //count_bits(block);



    //print_bitboard(rooks_attack_on_the_fly(27, block));
    //for i in 0..64 {
    //     print_bitboard(white_pawns_attacks[i]);
    //     print_bitboard(black_pawns_attacks[i]);
    //     print_bitboard(knights_attacks[i]);
    //     print_bitboard(king_attacks[i]);
    //     print_bitboard(bishops_attacks[i]);
    //    print_bitboard(rooks_attacks[i]);
   // }
    //print_bitboard_rank(&[0]); }
}
