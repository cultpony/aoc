#[derive(
    Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, serde::Serialize, serde::Deserialize,
)]
pub struct Year(u16);

pub const fn year(d: i32) -> Year {
    Year(d as u16)
}

impl std::fmt::Display for Year {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{: >4}", self.0))
    }
}

impl From<u16> for Year {
    fn from(val: u16) -> Self {
        Year(val)
    }
}

impl PartialEq<u16> for Year {
    fn eq(&self, other: &u16) -> bool {
        self.0 == *other
    }
}

impl Year {
    pub fn as_u16(self) -> u16 {
        self.0
    }
}
