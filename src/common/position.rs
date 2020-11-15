#[derive(Debug, Clone)]
pub struct Position {
    x: u64,
    y: u64,
}

impl Position {
    pub fn init(x: u64, y: u64) -> Self {
        Self { x, y }
    }
}
