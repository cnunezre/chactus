struct Board {
    pieces: [u64; 12],
    turn: bool,
}

fn get_bit(bitboard: u64, square: u64) -> bool {
    bitboard & (1u64 << square) > 0
}

fn print_bitboard(bitboard: u64) {
    //loop over rank
    for r in 0..8 {
        //loop over every file
        for f in 0..8 {
            if f == 0 {
                print!("{}   ", 8 - r);
            }
            let square = r * 8 + f;
            print!("{} ", if get_bit(bitboard, square) { 1 } else { 0 });
        }
        println!();
    }
    println!("\n    a b c d e f g h");
    println!("    Bitboard: {}", bitboard);
}

fn main() {
    println!("Elephant Chess Engine v.0\n");
    print_bitboard(65);
}
