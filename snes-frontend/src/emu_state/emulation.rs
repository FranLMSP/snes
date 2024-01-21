pub struct EmulationState {
    pub is_paused: bool,
    pub one_tick_per_frame: bool,
}

impl EmulationState {
    pub fn new() -> Self {
        Self {
            is_paused: true,
            one_tick_per_frame: false,
        }
    }
}