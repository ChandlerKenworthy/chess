use crate::generator::{Bitboard, bitboard::{FILE_A, FILE_H, PRIMARY_DIAGONAL_RAYS, PRIMARY_STRAIGHT_RAYS, RANK_1, RANK_8, SECONDARY_DIAGONAL_RAYS, SECONDARY_STRAIGHT_RAYS}, movegen::hyperbola_quintessence};
use std::sync::OnceLock;

/// Implementation of the magic bitboards
/// Principle is:
/// index = ((occupancy & mask[sq]) * magic[sq]) >> (64 - mask[sq].count_ones())
/// attacks = table[sq][index] <-- pre-computed tables for the given piece

const EDGES: Bitboard = RANK_1 | RANK_8 | FILE_A | FILE_H;

pub fn init() {
    let _ = magics();
}

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        // xorshift64*
        self.0 ^= self.0 >> 12;
        self.0 ^= self.0 << 25;
        self.0 ^= self.0 >> 27;
        self.0.wrapping_mul(0x2545F4914F6CDD1D)
    }
    // Magics with few set bits work far better
    fn sparse(&mut self) -> u64 {
        self.next() & self.next() & self.next()
    }
}

#[derive(Clone, Copy, Default)]
struct Magic {
    mask: Bitboard,
    magic: u64,
    shift: u32,
    offset: usize, // start of this square's slice in the shared table
}

fn find_magic(sq: usize, is_diagonal: bool, rng: &mut Rng) -> (Magic, Vec<Bitboard>) {
    let mask = relevant_mask(sq, is_diagonal);
    let bits = mask.count_ones();
    let size = 1usize << bits;

    // Enumerate all subsets of mask (Carry-Rippler)
    let mut occs = Vec::with_capacity(size);
    let mut atts = Vec::with_capacity(size);
    let mut subset: Bitboard = 0;
    loop {
        occs.push(subset);
        atts.push(slow_attacks(sq, subset, is_diagonal));
        subset = subset.wrapping_sub(mask) & mask;
        if subset == 0 {
            break;
        }
    }

    let shift = 64 - bits;
    let mut table = vec![0u64; size]; // 0 = unused (a slider's attack set is never empty)

    loop {
        let magic = rng.sparse();
        // Quick reject: too few high bits means poor mixing
        if (mask.wrapping_mul(magic) >> 56).count_ones() < 6 {
            continue;
        }
        table.iter_mut().for_each(|e| *e = 0);

        let ok = occs.iter().zip(&atts).all(|(&occ, &att)| {
            let idx = (occ.wrapping_mul(magic) >> shift) as usize;
            if table[idx] == 0 {
                table[idx] = att;
                true
            } else {
                table[idx] == att // constructive collision is fine
            }
        });

        if ok {
            return (Magic { mask, magic, shift, offset: 0 }, table);
        }
    }
}


// Masks for each square, excluding the board edge
fn relevant_mask(square_idx: usize, is_diagonal: bool) -> Bitboard {
    // Primary straight rays for e.g. square a1 gives you a2,a3...a8
    if is_diagonal {
        let rays = PRIMARY_DIAGONAL_RAYS[square_idx] | SECONDARY_DIAGONAL_RAYS[square_idx];
        rays & !EDGES
    } else {
        let rays = PRIMARY_STRAIGHT_RAYS[square_idx] | SECONDARY_STRAIGHT_RAYS[square_idx];

        // Only clip the edges the rook is NOT on
        let rank_mask = RANK_1 << (8 * (square_idx / 8));
        let file_mask = FILE_A << (square_idx % 8);
        let edges = ((RANK_1 | RANK_8) & !rank_mask) | ((FILE_A | FILE_H) & !file_mask);

        rays & !edges
    }
}

fn slow_attacks(sq: usize, occ: Bitboard, is_diagonal: bool) -> Bitboard {
    let from: Bitboard = 1u64 << sq;
    if is_diagonal {
        hyperbola_quintessence(occ, from, PRIMARY_DIAGONAL_RAYS[sq])
            | hyperbola_quintessence(occ, from, SECONDARY_DIAGONAL_RAYS[sq])
    } else {
        hyperbola_quintessence(occ, from, PRIMARY_STRAIGHT_RAYS[sq])
            | hyperbola_quintessence(occ, from, SECONDARY_STRAIGHT_RAYS[sq])
    }
}

pub struct SliderTable {
    magics: [Magic; 64],
    table: Vec<Bitboard>,
}

impl SliderTable {
    fn new(is_diagonal: bool, seed: u64) -> Self {
        let mut rng = Rng(seed);
        let mut magics = [Magic::default(); 64];
        let mut table = Vec::new();
        for sq in 0..64 {
            let (mut m, t) = find_magic(sq, is_diagonal, &mut rng);
            m.offset = table.len();
            table.extend_from_slice(&t);
            magics[sq] = m;
        }
        Self { magics, table }
    }

    fn from_magics(is_diagonal: bool, magic_numbers: &[u64; 64]) -> Self {
        let mut magics = [Magic::default(); 64];
        let mut table = Vec::new();

        for sq in 0..64 {
            let mask = relevant_mask(sq, is_diagonal);
            let shift = 64 - mask.count_ones();
            let size = 1usize << mask.count_ones();
            let magic = magic_numbers[sq];
            let offset = table.len();

            table.resize(offset + size, 0u64);

            // Carry-Rippler over all blocker subsets
            let mut subset: Bitboard = 0;
            loop {
                let idx = (subset.wrapping_mul(magic) >> shift) as usize;
                table[offset + idx] = slow_attacks(sq, subset, is_diagonal);
                subset = subset.wrapping_sub(mask) & mask;
                if subset == 0 {
                    break;
                }
            }

            magics[sq] = Magic { mask, magic, shift, offset };
        }
        Self { magics, table }
    }

    #[inline(always)]
    pub fn attacks(&self, sq: usize, occ: Bitboard) -> Bitboard {
        let m = &self.magics[sq];
        let idx = (occ & m.mask).wrapping_mul(m.magic) >> m.shift;
        // SAFETY-free version; use get_unchecked later if profiling shows bounds checks matter
        self.table[m.offset + idx as usize]
    }
}

pub struct Magics {
    pub rook: SliderTable,
    pub bishop: SliderTable,
}

pub const ROOK_MAGICS: [u64; 64] = [
    0x4180008840002014,
    0x0280094000d02000,
    0x1600042110410880,
    0x2080100004428800,
    0x1500034238000410,
    0x0100020081040008,
    0x0400004204408890,
    0x0200002081140042,
    0x0040800040062150,
    0x0001400010012001,
    0x000a000a2011c080,
    0x300300081000a004,
    0x4006000802200410,
    0x0060800200010400,
    0x2004000e00840c01,
    0x2002000044842106,
    0x000810800040002d,
    0xb040022001500804,
    0x8820004008001000,
    0x1040848010000801,
    0x0000020010082004,
    0x1000018004000a00,
    0x0000040001108082,
    0x2082010000408032,
    0x2800102280004008,
    0x0100100840042001,
    0x0020000880201000,
    0x0800082200041041,
    0x0018040300100800,
    0x4000084100024400,
    0x480104ac00021109,
    0x9000002200004419,
    0x1480002002400214,
    0x0040002004401004,
    0x0020001000400800,
    0x8000880004801002,
    0x1040102002000804,
    0x8800040001000208,
    0x0001080204003890,
    0x80000c0042000089,
    0x10061920c0028000,
    0x00e0006010014000,
    0x0004402480120008,
    0x2000a01000090006,
    0x0400040008018002,
    0x0010020004008001,
    0x920020a804420001,
    0x100c8410806a0001,
    0x2100f10020438600,
    0x0000801040210200,
    0x1004f00088a00480,
    0x002a04091020c200,
    0x6400040008000280,
    0x0000011004080200,
    0x38020031000a0880,
    0x2880004421008200,
    0xa21140821100200a,
    0x1a01420880201102,
    0x0000400806102001,
    0x2c0008044050a026,
    0x0000100800020401,
    0x4011648400080a01,
    0x0400122904480084,
    0x0000001401002082,
];
pub const BISHOP_MAGICS: [u64; 64] = [
    0x8810101001002020,
    0x0048011842084080,
    0x000401220a010c08,
    0x1042408100000000,
    0x0002021080000000,
    0x0a02080208080200,
    0x20090401a0080604,
    0x1002004044042000,
    0x0100101001084088,
    0x0000200a0050e430,
    0x4000100190809300,
    0x6000104040807010,
    0x4000040308000822,
    0x204802093008c850,
    0x4000030410822820,
    0x00c2002208024800,
    0x2089044042040439,
    0x0110000441080100,
    0x0004000204140006,
    0x00004008a0102200,
    0x404a000220210080,
    0x0010200130010840,
    0x4022000440410814,
    0x0e8301008c010110,
    0x00421001a1204200,
    0x1085410002331180,
    0x400030002800800f,
    0x1282206000410420,
    0x0020840000802011,
    0x8802002000101804,
    0x8001064804020a82,
    0x48008084402a0800,
    0x0062101000c18200,
    0x050080a680440844,
    0x0100280100080142,
    0x00008700c0040040,
    0x0015040220010010,
    0x0a10004880001000,
    0x2044909481010804,
    0x0020a48090010401,
    0xc082080540002500,
    0x0000240420000414,
    0x1002040201000804,
    0x8400001080800900,
    0x0010010200200902,
    0x0040060612100020,
    0x0501034802800442,
    0x8008421040402200,
    0x8001008210400006,
    0x1542804048040000,
    0x45a0004200900140,
    0x1001020814040000,
    0x0000080210440006,
    0x4008208470408014,
    0x1006500400844000,
    0x00020801040c80c6,
    0x4a02024500882040,
    0x0120c02228040400,
    0x2051001040441025,
    0x820d500241842400,
    0x002454042020410c,
    0x0054004010020389,
    0x0000682008008321,
    0x4022200202260020,
];

pub fn magics() -> &'static Magics {
    static MAGICS: OnceLock<Magics> = OnceLock::new();
    MAGICS.get_or_init(|| Magics {
        rook: SliderTable::from_magics(false, &ROOK_MAGICS),
        bishop: SliderTable::from_magics(true, &BISHOP_MAGICS),
    })
}


#[inline(always)]
pub fn rook_attacks(sq: usize, occ: Bitboard) -> Bitboard { magics().rook.attacks(sq, occ) }
#[inline(always)]
pub fn bishop_attacks(sq: usize, occ: Bitboard) -> Bitboard { magics().bishop.attacks(sq, occ) }
#[inline(always)]
pub fn queen_attacks(sq: usize, occ: Bitboard) -> Bitboard {
    rook_attacks(sq, occ) | bishop_attacks(sq, occ)
}

#[cfg(test)]
mod magic_tests {
    use super::*;

    fn print_array(name: &str, is_diagonal: bool, seed: u64) {
        let mut rng = Rng(seed);
        println!("pub const {name}: [u64; 64] = [");
        for sq in 0..64 {
            let (m, _table) = find_magic(sq, is_diagonal, &mut rng);
            println!("    {:#018x},", m.magic);
        }
        println!("];");
    }

    #[test]
    #[ignore] // only runs when asked for
    fn print_magics() {
        print_array("ROOK_MAGICS", false, 0x1234_5678_9ABC_DEF1);
        print_array("BISHOP_MAGICS", true, 0x0FED_CBA9_8765_4321);
    }
}