pub struct IDseed {
    seed: u32
}

impl IDseed {
    pub fn new() -> IDseed {
        IDseed {
            id: 0
        }
    }

    pub fn get_id(&mut self) -> u32 {
        self += 1;
        self.copy() - 1
    }
}
