use crate::Bitboard;

// Bit and Square utils
fn is_bit_active(bitboard: Bitboard, square_number: u8) -> bool {
    (bitboard & (1u64 << square_number)) != 0
}

pub fn get_square_number(square: &str) -> u8 {
    let mut iter = square.chars();
    let file = iter.next().unwrap();
    let file_n: u32 = file.to_ascii_lowercase() as u32 - 'a' as u32;
    let rank = iter.next().unwrap().to_digit(10).unwrap() - 1;
    (rank * 8 + file_n) as u8
}

pub fn print_bitboard(bitboard: Bitboard) {
    const LAST_BIT: u64 = 63;
    for rank in 0..8 {
        print!("{} |", 8 - rank);
        for file in (0..8).rev() {
            let mask = 1u64 << (LAST_BIT - (rank * 8) - file);
            let char = if bitboard & mask != 0 { '1' } else { '0' };
            print!("{char} ");
        }
        println!();
    }
    println!("   ---------------");
    println!("   a b c d e f g h");
    println!("Bitboard:\t{}", bitboard);
}

pub fn set_bit_active(bitboard: Bitboard, square_number: u8) -> Bitboard {
    bitboard | (1u64 << square_number)
}

fn set_bit(bitboard: Bitboard, square: &str) -> Bitboard {
    let square_number = get_square_number(square);
    set_bit_active(bitboard, square_number)
}

fn reset_bit(bitboard: Bitboard, square: &str) -> Bitboard {
    let square_number = get_square_number(square);
    if is_bit_active(bitboard, square_number) {
        bitboard ^ (1u64 << square_number)
    } else {
        bitboard
    }
}

fn get_bitboard(square: &str) -> Bitboard {
    let two_exponent = get_square_number(square);
    (1u64 << two_exponent)
}

fn get_algebraic_notation(bitboard: Bitboard) -> String {
    let square = (bitboard as f64).log2() as u32;
    from_square_to_algebraic_notation(square)
}

pub fn from_square_to_algebraic_notation(square: u32) -> String {
    let i = square % 8 + 'a' as u32;
    let file: char = char::from_u32(i as u32).unwrap();
    let rank: u32 = square / 8 + 1;
    file.to_string() + &rank.to_string()
}

fn print_bitboard_file(file_to_check: &[u8]) {
    const LAST_BIT: u64 = 63;
    let mut bitboard: Bitboard = 0u64;

    for rank in (0..8).rev() {
        for file in (0..8) {
            let square = (rank * 8) + file;
            //bitboard = bitboard | (1u64 << square);
            //let char = if file != 0  { '1' } else { '0' };
            if file_to_check.contains(&file) {
                bitboard = set_bit_active(bitboard, square);
            }
            print!("{square} ");
        }
        println!();
    }
    println!("   ---------------");
    println!("   a b c d e f g h");
    println!("Bitboard:\t{}", bitboard);
    print_bitboard(bitboard);
}

fn print_bitboard_rank(ranks_to_check: &[u8]) {
    const LAST_BIT: u64 = 63;
    let mut bitboard: Bitboard = 0u64;
    for rank in (0..8).rev() {
        if ranks_to_check.contains(&rank) {
            for file in (0..8) {
                let square = (rank * 8) + file;
                //bitboard = bitboard | (1u64 << square);
                //let char = if file != 0  { '1' } else { '0' };
                bitboard = set_bit_active(bitboard, square);
                print!("{square} ");
            }
        }
        println!();
    }

    println!("   ---------------");
    println!("   a b c d e f g h");
    println!("Bitboard:\t{}", bitboard);
    print_bitboard(bitboard);
}

pub fn count_bits(board: Bitboard) {
    let mut count = 0;
    let mut board_aux: Bitboard = board;
    loop {
        board_aux &= board_aux - 1;
        count += 1;
        if ((board_aux | 0u64) == 0) {
            break;
        }
    }
}

pub fn get_lsb_index(board: Bitboard) -> u32{
    board.trailing_zeros()
}

pub fn get_msb_index(board: Bitboard) -> u32{
    63 - board.leading_zeros()
}