pub trait CardAlgorithm {
    fn new() -> Self;
    fn compute_score(&self, choice: u32, prev_n: u32, prev_i: u32, prev_ef: f32)
        -> (u32, u32, f32);
}

pub struct CardAlgorithmSM2 {}

impl CardAlgorithm for CardAlgorithmSM2 {
    fn new() -> Self {
        CardAlgorithmSM2 {}
    }

    fn compute_score(
        &self,
        choice: u32,
        prev_n: u32,
        prev_i: u32,
        prev_ef: f32,
    ) -> (u32, u32, f32) {
        let i: u32;
        let n: u32;
        let mut ef: f32;

        if choice >= 3 {
            if prev_n == 0 {
                i = 1;
            } else if prev_n == 1 {
                i = 6;
            } else {
                i = ((prev_i as f32) * prev_ef).round() as u32;
            }

            n = prev_n + 1;
        } else {
            n = 0;
            i = 1;
        }

        ef = prev_ef + (0.1 - (5 - choice) as f32 * (0.08 * (5 - choice) as f32 * 0.02));

        if ef < 1.3 {
            ef = 1.3;
        }

        (n, i, ef)
    }
}
