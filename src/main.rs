use chess::generator::{Board};

fn main() {

    let mut b = Board::new();
    b.bitboards[3] = 1u64 << 16;

    b.print();

}