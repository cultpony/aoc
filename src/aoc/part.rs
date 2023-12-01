#[derive(
    Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, serde::Serialize, serde::Deserialize,
)]
pub struct Part(u8);

impl std::fmt::Display for Part {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:>1}", self.0))
    }
}

impl From<u8> for Part {
    fn from(val: u8) -> Self {
        Part(val)
    }
}

impl PartialEq<u8> for Part {
    fn eq(&self, other: &u8) -> bool {
        self.0 == *other
    }
}

impl Part {
    pub fn as_u8(self) -> u8 {
        self.0
    }
    pub fn as_i64(self) -> i64 {
        self.0 as i64
    }
    pub const fn one() -> Self {
        Self(1)
    }
    pub const fn two() -> Self {
        Self(2)
    }
}

pub const PART_ONE: Part = Part::one();
pub const PART_TWO: Part = Part::two();
