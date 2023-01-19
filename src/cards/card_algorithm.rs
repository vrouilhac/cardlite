pub trait CardAlgorithm {
    fn new() -> Self;
}

pub struct CardAlgorithmSM2 {}

impl CardAlgorithm for CardAlgorithmSM2 {
    fn new() -> Self {
        CardAlgorithmSM2 {}
    }
}
