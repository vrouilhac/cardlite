pub enum ReviewScore {
    ZERO,
    ONE,
    TWO,
    THREE,
    FOUR,
    FIVE,
}

impl ReviewScore {
    pub fn get_value(&self) -> u32 {
        match &self {
            ReviewScore::ZERO => 0,
            ReviewScore::ONE => 1,
            ReviewScore::TWO => 2,
            ReviewScore::THREE => 3,
            ReviewScore::FOUR => 4,
            ReviewScore::FIVE => 5,
        }
    }
}
