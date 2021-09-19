use std::cmp::Ordering;
use std::fmt::{self, Display};

#[derive(Clone)]
pub(crate) struct Crate(String);

impl Crate {
    pub(crate) fn new(string: String) -> Self {
        Crate(string)
    }
}

impl Display for Crate {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(&self.0, formatter)
    }
}

impl Ord for Crate {
    fn cmp(&self, rhs: &Self) -> Ordering {
        self.0
            .bytes()
            .map(CaseInsensitiveByte)
            .cmp(rhs.0.bytes().map(CaseInsensitiveByte))
    }
}

impl PartialOrd for Crate {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        Some(self.cmp(rhs))
    }
}

impl Eq for Crate {}

impl PartialEq for Crate {
    fn eq(&self, rhs: &Self) -> bool {
        self.0
            .bytes()
            .map(CaseInsensitiveByte)
            .eq(rhs.0.bytes().map(CaseInsensitiveByte))
    }
}

struct CaseInsensitiveByte(u8);

impl Ord for CaseInsensitiveByte {
    fn cmp(&self, rhs: &Self) -> Ordering {
        let lhs = if self.0 == b'_' {
            b'-'
        } else {
            self.0.to_ascii_lowercase()
        };
        let rhs = if rhs.0 == b'_' {
            b'-'
        } else {
            rhs.0.to_ascii_lowercase()
        };
        lhs.cmp(&rhs)
    }
}

impl PartialOrd for CaseInsensitiveByte {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        Some(self.cmp(rhs))
    }
}

impl Eq for CaseInsensitiveByte {}

impl PartialEq for CaseInsensitiveByte {
    fn eq(&self, rhs: &Self) -> bool {
        self.cmp(rhs) == Ordering::Equal
    }
}
