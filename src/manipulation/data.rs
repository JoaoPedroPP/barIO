

#[derive(Debug, Clone)]
pub struct Hist {
    pub id: u8,
    pub value: u32,
}

impl Hist {
    pub fn default() -> Hist {
        Hist { id: 0, value: 0 }
    }
}