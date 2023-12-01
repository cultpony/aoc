#[derive(
    Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, serde::Serialize, serde::Deserialize,
)]
pub struct Day(u8);

pub const fn day(d: i32) -> Day {
    Day::new(d as u8)
}

impl std::fmt::Display for Day {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:0>2}", self.0))
    }
}

impl PartialEq<u8> for Day {
    fn eq(&self, other: &u8) -> bool {
        self.0 == *other
    }
}

impl Day {
    pub fn as_u8(self) -> u8 {
        self.0
    }
    pub const fn new(d: u8) -> Self {
        Self(d)
    }
}

impl From<u8> for Day {
    fn from(val: u8) -> Self {
        Day(val)
    }
}
