use crate::utils::{bb_to_idx, print_bb};

pub const WALL_RIGHT: u64 = 0x8080808080808080;
pub const WALL_LEFT: u64 = 0x101010101010101;
pub const WALL_UP: u64 = 0xff00000000000000;
pub const WALL_DOWN: u64 = 0xff;
pub const DIAGONAL_DOWN: u64 = 0x102040810204080;
pub const DIAGONAL_UP: u64 = 0x8040201008040201;
pub const W_PAWN_PUSHES: [u64; 64] = {
    let mut moves = [0u64; 64];
    let mut i = 0;
    while i < 64 {
        let mut p: u64 = 1 << i;
        p = move_bb_vert(p, 1);
        moves[i] = p;
        i += 1;
    }
    moves
};
pub const B_PAWN_PUSHES: [u64; 64] = {
    let mut moves = [0u64; 64];
    let mut i = 0;
    while i < 64 {
        let mut p: u64 = 1 << i;
        p = move_bb_vert(p, -1);
        moves[i] = p;
        i += 1;
    }
    moves
};
pub const W_PAWN_DOUBLE_PUSHES: [u64; 64] = {
    let mut moves = [0u64; 64];
    let mut i = 0;
    while i < 64 {
        if i >= 8 && i <= 15 {
            let mut p: u64 = 1 << i;
            p = move_bb_vert(p, 2);
            moves[i] = p;
        }
        i += 1;
    }
    moves
};
pub const B_PAWN_DOUBLE_PUSHES: [u64; 64] = {
    let mut moves = [0u64; 64];
    let mut i = 0;
    while i < 64 {
        if i >= 48 && i <= 55 {
            let mut p: u64 = 1 << i;
            p = move_bb_vert(p, -2);
            moves[i] = p;
        }
        i += 1;
    }
    moves
};
pub const W_PAWN_CAPS: [u64; 64] = {
    let mut moves = [0u64; 64];
    let mut i = 0;
    while i < 64 {
        let mut p1: u64 = 1 << i;
        p1 = move_bb_vert(p1, 1);
        p1 = move_bb_hor(p1, 1);

        let mut p2: u64 = 1 << i;
        p2 = move_bb_vert(p2, 1);
        p2 = move_bb_hor(p2, -1);
        moves[i] = p2 | p1;
        i += 1;
    }
    moves
};

pub const B_PAWN_CAPS: [u64; 64] = {
    let mut moves = [0u64; 64];
    let mut i = 0;
    while i < 64 {
        let mut p1: u64 = 1 << i;
        p1 = move_bb_vert(p1, -1);
        p1 = move_bb_hor(p1, 1);

        let mut p2: u64 = 1 << i;
        p2 = move_bb_vert(p2, -1);
        p2 = move_bb_hor(p2, -1);
        moves[i] = p2 | p1;
        i += 1;
    }
    moves
};
static mut ROOK_POTENTIAL_MOVES: [u64; 64] = [0; 64];
static mut BISHOP_POTENTIAL_MOVES: [u64; 64] = [0; 64];
static mut ROOK_MAGICS: [u64; 64] = [
    0x80004001802010,
    0x1040100140022008,
    0x80182000809000,
    0x180180080251000,
    0x200020088042010,
    0xc00900220040008,
    0x2480010006000880,
    0xa00040200402081,
    0x2800020400088,
    0x6109004002802100,
    0xa00801000200080,
    0x803800802801000,
    0x800800240180,
    0x12800600800401,
    0x11000411002200,
    0x1092000844148102,
    0x2000808000400020,
    0x1081210040018108,
    0x430002004080062,
    0x40040a0020120040,
    0x4621808004010800,
    0x422008002040080,
    0x3020140012100108,
    0xa0021004084,
    0x400080022484,
    0x404802100400102,
    0x51050041002002b2,
    0x200100100200901,
    0x5280100041100,
    0x801001900040082,
    0xe0400887003,
    0x400018a20000c407,
    0x420082002500,
    0x4320822000804001,
    0x10200080805000,
    0x441000821001000,
    0xb140040080803800,
    0x4000440080804600,
    0x229004000108,
    0xb8040052000081,
    0x2200208040008011,
    0x820902420021,
    0x400410020090010,
    0x8010000800108080,
    0x3350028010010,
    0xc002000804420050,
    0x1081042040081,
    0x4082208449020004,
    0x800020c00080,
    0x840400221009100,
    0x10100c010200900,
    0x9070801000480080,
    0x8442801401280080,
    0x1000834000300,
    0x28060004980b0200,
    0x8043042180d200,
    0x1000410020108001,
    0x2104008802101,
    0x2000401120004901,
    0x80201000090005,
    0xc2010820101402,
    0xa0490088020c0005,
    0x3023000200018413,
    0x80000b8046210402,
];

static mut BISHOP_MAGICS: [u64; 64] = [
    0x6400248061520c0,
    0x2028100122112320,
    0x6110010a08201000,
    0x6008194108008100,
    0x219104000020000,
    0x144901008000a48,
    0x2804012818940800,
    0x10221011001a000,
    0x1108108202080624,
    0x4100202812208030,
    0x846100102102008,
    0x880845000000,
    0x800040421800148,
    0x2840a3004200222,
    0x800410410c001,
    0xd01106098141000,
    0x4c08007022089810,
    0x6008001010128aa0,
    0x10086905020890,
    0x4001840112000,
    0x1200885400a00500,
    0x110600008a012020,
    0x2080889104101228,
    0x2090042008400,
    0x40e1100005040816,
    0x284100420020180,
    0x2441001060a2040,
    0xa08080068220020,
    0x2140840002020201,
    0x10008101029084,
    0x11111486009000,
    0x1020063044100,
    0x1044090600840,
    0x1420120a0104214,
    0x822002a10340800,
    0x6074910800040240,
    0x1040004110110100,
    0x38500a1020020080,
    0x12020040040421,
    0x1210840040028208,
    0x1082012414c00,
    0x810c010708801081,
    0x12050041000802,
    0x2148106019000800,
    0x8000250122004400,
    0x2104200200200,
    0x8102900c80200,
    0x1118008900c84200,
    0x8080942420040800,
    0x26004208050802,
    0x886102209500020,
    0x2818700620981800,
    0x30002a020410100,
    0x120009200c042000,
    0x4008021002020112,
    0x6200a0081010000,
    0x8c340202022006,
    0x1028032401281801,
    0xa00044240400,
    0x1022680000608808,
    0x810200220c104402,
    0x5000004110044120,
    0x320309010048980,
    0x620200102002140,
];
static mut ROOK_MOVES: [[u64; 4096]; 64] = [[0; 4096]; 64];
static mut BISHOP_MOVES: [[u64; 512]; 64] = [[0; 512]; 64];

pub const KNIGHT_TABLE: [u64; 64] = {
    let bb_sq = 18;
    let knight_moves_bb = 0xa1100110a;
    let mut table = [0; 64];
    let mut i = 0;
    while i < table.len() as u64 {
        table[i as usize] = move_bb_slow(knight_moves_bb, bb_sq, i);
        i += 1;
    }
    table
};
pub const KING_TABLE: [u64; 64] = {
    let bb_sq = 9;
    let knight_moves_bb = 0x70507;
    let mut table = [0; 64];
    let mut i = 0;
    while i < table.len() as u64 {
        table[i as usize] = move_bb_slow(knight_moves_bb, bb_sq, i);
        i += 1;
    }
    table
};

const fn move_bb_vert(bb: u64, amount: i32) -> u64 {
    let mut amount = amount;
    let mut ret = bb;
    while amount > 0 {
        // shift Up
        ret &= !WALL_UP;
        ret <<= 8;

        amount -= 1;
    }
    while amount < 0 {
        // shift Down
        ret &= !WALL_DOWN;
        ret >>= 8;

        amount += 1;
    }
    ret
}
const fn move_bb_hor(bb: u64, amount: i32) -> u64 {
    let mut ret = bb;
    let mut amount = amount;
    while amount < 0 {
        // shift to the left
        ret &= !WALL_LEFT;
        ret >>= 1;
        amount += 1;
    }
    while amount > 0 {
        // shift to the right
        ret &= !WALL_RIGHT;
        ret <<= 1;

        amount -= 1;
    }
    ret
}

const fn move_bb_slow(bb: u64, center: u64, to: u64) -> u64 {
    let mut ret = bb;
    let col_start = center % 8;
    let col_end = to % 8;
    let row_start = center / 8;
    let row_end = to / 8;
    ret = move_bb_hor(ret, col_end as i32 - col_start as i32);
    ret = move_bb_vert(ret, row_end as i32 - row_start as i32);
    ret
}
pub fn count_bits(bb: u64) -> u32 {
    let mut count = 0;
    let mut bb = bb;
    while bb != 0 {
        let zeros = bb.trailing_zeros();
        count += 1;
        bb &= !(1 << zeros);
    }
    count
}
fn rand() -> u64 {
    /* initial seed must be nonzero, don't use a static variable for the state if multithreaded */
    static mut X: u64 = 123;
    unsafe {
        X ^= X >> 12;
        X ^= X << 25;
        X ^= X >> 27;
        X.wrapping_mul(0x2545F4914F6CDD1D)
    }
}

pub fn find_magic_rook(bb: u64) -> u64 {
    loop {
        let possible_magic = rand() & rand() & rand();
        if check_magic(bb, possible_magic, 12) {
            break possible_magic;
        }
    }
}
pub fn find_magic_bishop(bb: u64) -> u64 {
    loop {
        let possible_magic = rand() & rand() & rand();
        if check_magic(bb, possible_magic, 9) {
            break possible_magic;
        }
    }
}

fn check_magic(bb: u64, magic: u64, offset: u8) -> bool {
    let max_num = 2u64.pow(count_bits(bb));
    let table_size = 2u64.pow(offset.into());
    let mut table = vec![false; table_size as usize];
    for idx in 0..max_num {
        let variant = variants(bb, idx);
        let garbage = variant.wrapping_mul(magic);
        let magic_idx = garbage >> (64 - offset);
        if table[magic_idx as usize] {
            return false;
        }
        table[magic_idx as usize] = true;
    }
    true
}

pub fn fill_magic_table_rook(magic: u64, square: u64) {
    let bb;
    unsafe {
        bb = ROOK_POTENTIAL_MOVES[square as usize];
    }
    let bit_count = count_bits(bb);
    let max_num = 2u64.pow(bit_count);
    for idx in 0..max_num {
        let variant = variants(bb, idx);

        let mut move_mask = 0;

        // up
        let up_mask = move_bb_slow(WALL_LEFT, 0, square);
        let first_block = (variant & up_mask).trailing_zeros() as u64;

        let mut blocked_mask;
        blocked_mask = move_bb_slow(WALL_LEFT, 0, first_block);
        blocked_mask = !move_bb_vert(blocked_mask, 1);

        move_mask |= up_mask & blocked_mask;

        // down
        let down_mask = move_bb_slow(WALL_LEFT, 56, square);
        let first_block;
        if variant & down_mask != 0 {
            first_block = 63 - (variant & down_mask).leading_zeros() as u64;
        } else {
            first_block = 100;
        }

        let mut blocked_mask;
        blocked_mask = move_bb_slow(WALL_LEFT, 56, first_block);
        blocked_mask = !move_bb_vert(blocked_mask, -1);

        move_mask |= down_mask & blocked_mask;

        // left
        let left_mask = move_bb_slow(WALL_DOWN, 7, square);
        let first_block;
        if variant & left_mask != 0 {
            first_block = 63 - (variant & left_mask).leading_zeros() as u64;
        } else {
            first_block = 100;
        }

        let mut blocked_mask;
        blocked_mask = move_bb_slow(WALL_DOWN, 7, first_block);
        blocked_mask = !move_bb_hor(blocked_mask, -1);

        move_mask |= left_mask & blocked_mask;

        // right
        let right_mask = move_bb_slow(WALL_DOWN, 0, square);
        let first_block = (variant & right_mask).trailing_zeros() as u64;

        let mut blocked_mask;
        blocked_mask = move_bb_slow(WALL_DOWN, 0, first_block);
        blocked_mask = !move_bb_hor(blocked_mask, 1);

        move_mask |= right_mask & blocked_mask;

        move_mask &= !(1 << square);

        let garbage = variant.wrapping_mul(magic);
        // if square == 7 {
        //     println!("potential blockers");
        //     print_bb(bb);
        //     println!("variant");
        //     print_bb(variant);
        //     println!("Allowed moves");
        //     print_bb(move_mask);
        // }
        let magic_idx = garbage >> (64 - bit_count);
        unsafe {
            ROOK_MOVES[square as usize][magic_idx as usize] = move_mask;
        }
    }
}

pub fn fill_magic_table_bshop(magic: u64, square: u64) {
    let bb;
    unsafe {
        bb = BISHOP_POTENTIAL_MOVES[square as usize];
    }
    let bit_count = count_bits(bb);
    let max_num = 2u64.pow(bit_count);
    for idx in 0..max_num {
        let variant = variants(bb, idx);

        let mut move_mask = 0;

        // up - right
        let up_right_mask = move_bb_slow(DIAGONAL_UP, 0, square);
        let first_block = (variant & up_right_mask).trailing_zeros() as u64;

        let mut blocked_mask;
        blocked_mask = move_bb_slow(DIAGONAL_UP, 0, first_block);
        blocked_mask = move_bb_vert(blocked_mask, 1);
        blocked_mask = !move_bb_hor(blocked_mask, 1);

        move_mask |= up_right_mask & blocked_mask;

        // down - right
        let down_right_mask = move_bb_slow(DIAGONAL_DOWN, 56, square);
        let first_block;
        if variant & down_right_mask != 0 {
            first_block = 63 - (variant & down_right_mask).leading_zeros() as u64;
        } else {
            first_block = 100;
        }
        let mut blocked_mask;
        blocked_mask = move_bb_slow(DIAGONAL_DOWN, 56, first_block);
        blocked_mask = move_bb_vert(blocked_mask, -1);
        blocked_mask = !move_bb_hor(blocked_mask, 1);
        move_mask |= down_right_mask & blocked_mask;

        // up - left
        let up_left_mask = move_bb_slow(DIAGONAL_DOWN, 7, square);
        let first_block = (variant & up_left_mask).trailing_zeros() as u64;
        let mut blocked_mask;
        blocked_mask = move_bb_slow(DIAGONAL_DOWN, 7, first_block);
        blocked_mask = move_bb_vert(blocked_mask, 1);
        blocked_mask = !move_bb_hor(blocked_mask, -1);

        move_mask |= up_left_mask & blocked_mask;

        // down - left
        let down_left_mask = move_bb_slow(DIAGONAL_UP, 63, square);
        let first_block;
        if variant & down_left_mask != 0 {
            first_block = 63 - (variant & down_left_mask).leading_zeros() as u64;
        } else {
            first_block = 100;
        }

        let mut blocked_mask;
        blocked_mask = move_bb_slow(DIAGONAL_UP, 63, first_block);
        blocked_mask = move_bb_vert(blocked_mask, -1);
        blocked_mask = !move_bb_hor(blocked_mask, -1);

        move_mask |= down_left_mask & blocked_mask;

        move_mask &= !(1 << square);

        let garbage = variant.wrapping_mul(magic);
        let magic_idx = garbage >> (64 - bit_count);
        unsafe {
            BISHOP_MOVES[square as usize][magic_idx as usize] = move_mask;
        }
    }
}

fn variants(bb: u64, idx: u64) -> u64 {
    let mut bit = 0;
    let mut bb = bb;
    for i in 0..64 {
        if bb & (1 << i) != 0 {
            if idx & (1 << bit) == 0 {
                bb ^= 1 << i;
            }
            bit += 1;
        }
    }
    bb
}

pub fn gen_rook_potential_moves() {
    for i in 0..64 {
        let mut hor_line = WALL_DOWN;
        hor_line = move_bb_vert(hor_line, i / 8);
        hor_line &= !WALL_LEFT;
        hor_line &= !WALL_RIGHT;

        let mut vert_line = WALL_LEFT;
        vert_line = move_bb_hor(vert_line, i % 8);
        vert_line &= !WALL_UP;
        vert_line &= !WALL_DOWN;

        let mut cross = vert_line | hor_line;
        cross &= !(1 << i);

        unsafe { ROOK_POTENTIAL_MOVES[i as usize] = cross };
    }
}
pub fn gen_bishop_potential_moves() {
    for y in 0..8 {
        for x in 0..8 {
            let i = y * 8 + x;
            let down_diag = DIAGONAL_DOWN;
            let offset = x + y - 7;
            let down_diag = move_bb_hor(down_diag, offset);

            let up_diag = DIAGONAL_UP;
            let offset = x - y;
            let up_diag = move_bb_hor(up_diag, offset);

            let mut cross = down_diag ^ up_diag;
            cross &= !WALL_LEFT;
            cross &= !WALL_RIGHT;
            cross &= !WALL_UP;
            cross &= !WALL_DOWN;
            unsafe { BISHOP_POTENTIAL_MOVES[i as usize] = cross };
        }
    }
}

pub fn get_rook_moves(rook_bb: u64, pieces: u64) -> u64 {
    //TODO: Store offsets in the magics table
    let rook_idx = bb_to_idx(rook_bb);
    let magic = unsafe { ROOK_MAGICS[rook_idx] };
    let potential_blockers = unsafe { ROOK_POTENTIAL_MOVES[rook_idx] };
    let blockers = potential_blockers & pieces;
    let garbadge = magic.wrapping_mul(blockers);
    //let offset = 64 - count_bits(potential_blockers);
    let magic_idx = garbadge >> (64 - 12);
    unsafe { ROOK_MOVES[rook_idx][magic_idx as usize] }
}
pub fn get_bishop_moves(bishop_bb: u64, pieces: u64) -> u64 {
    //TODO: Store offsets in the magics table
    let bishop_idx = bb_to_idx(bishop_bb);
    let magic = unsafe { BISHOP_MAGICS[bishop_idx] };
    let potentinal_blockers = unsafe { BISHOP_POTENTIAL_MOVES[bishop_idx] };
    let blockers = potentinal_blockers & pieces;
    let garbadge = magic.wrapping_mul(blockers);
    //let offset = 64 - count_bits(potentinal_blockers);
    let magic_idx = garbadge >> (64 - 9);
    unsafe { BISHOP_MOVES[bishop_idx][magic_idx as usize] }
}
pub fn init_magics(gen_magics: bool) {
    gen_rook_potential_moves();
    gen_bishop_potential_moves();
    for i in 0..64 {
        let rook_magic;
        let bshop_magic;
        unsafe {
            if gen_magics {
                rook_magic = find_magic_rook(ROOK_POTENTIAL_MOVES[i]);
                eprintln!("Found the rook magic");
                ROOK_MAGICS[i] = rook_magic;
                print_bb(BISHOP_POTENTIAL_MOVES[i]);
                bshop_magic = find_magic_bishop(BISHOP_POTENTIAL_MOVES[i]);
                BISHOP_MAGICS[i] = bshop_magic;
                eprintln!("Found the bishop magic");
            } else {
                rook_magic = ROOK_MAGICS[i];
                bshop_magic = BISHOP_MAGICS[i];
            }
            fill_magic_table_rook(rook_magic, i as u64);
            fill_magic_table_bshop(bshop_magic, i as u64);
        }
    }
    if !gen_magics {
        return;
    }
    unsafe {
        println!("{{");
        for magic in ROOK_MAGICS {
            println!("  {magic:#x},");
        }
        println!("}}");
        println!("{{");
        for magic in BISHOP_MAGICS {
            println!("  {magic:#x},");
        }
        println!("}}");
    }
}
