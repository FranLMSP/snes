pub struct EmulationState {
    pub is_paused: bool,
}

impl EmulationState {
    pub fn new() -> Self {
        Self {
            is_paused: true,
        }
    }
}