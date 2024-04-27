use std::usize;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Color {
    White = 0,
    Black = 1,
}
impl Color {
    pub fn opposite(&self) -> Self {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}

impl From<Color> for usize {
    fn from(value: Color) -> Self {
        value as usize
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Piece {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct BB(pub u64);

impl BB {
    pub fn empty(&self) -> bool {
        self.0 == 0
    }
    pub fn to_idx(&self) -> SquareIdx {
        SquareIdx(self.0.trailing_zeros() as u8)
    }
}

#[derive(Clone, Copy)]
pub struct SquareIdx(pub u8);

impl From<BB> for SquareIdx {
    fn from(sq: BB) -> SquareIdx {
        SquareIdx(sq.0.trailing_zeros() as u8)
    }
}
impl From<SquareIdx> for usize {
    fn from(sq: SquareIdx) -> usize {
        sq.0 as usize
    }
}
impl SquareIdx {
    pub fn new() -> Self {
        Self(255)
    }
    pub fn valid(&self) -> bool {
        self.0 != 255
    }
}
impl Default for SquareIdx {
    fn default() -> Self {
        Self(255)
    }
}
impl From<SquareIdx> for BB {
    fn from(value: SquareIdx) -> BB {
        BB(1 << value.0)
    }
}

impl Iterator for BB {
    type Item = BB;

    fn next(&mut self) -> Option<Self::Item> {
        fn pop_lsb(bb: &mut u64) -> u64 {
            let bit = 1 << bb.trailing_zeros();
            *bb &= !bit;
            bit
        }

        if self.0 == 0 {
            return None;
        }
        Some(BB(pop_lsb(&mut self.0)))
    }
}

impl std::ops::Not for BB {
    type Output = BB;

    fn not(self) -> Self::Output {
        BB(!self.0)
    }
}
impl std::ops::BitAnd for BB {
    type Output = BB;

    fn bitand(self, rhs: Self) -> Self::Output {
        BB(self.0 & rhs.0)
    }
}

impl std::ops::BitOr for BB {
    type Output = BB;

    fn bitor(self, rhs: Self) -> Self::Output {
        BB(self.0 | rhs.0)
    }
}
impl std::ops::BitXor for BB {
    type Output = BB;

    fn bitxor(self, rhs: Self) -> Self::Output {
        BB(self.0 ^ rhs.0)
    }
}
impl std::ops::BitAndAssign for BB {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 = self.0 & rhs.0;
    }
}
impl std::ops::BitOrAssign for BB {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 = self.0 | rhs.0;
    }
}
impl std::ops::BitXorAssign for BB {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 = self.0 ^ rhs.0;
    }
}
impl Into<BB> for u64 {
    fn into(self) -> BB {
        BB(self)
    }
}
