pub enum State {
    SearchingInit,
    SearchingEnd,
}

pub struct PNG {
    position: usize,
    match_index: usize,
    png_bytes: Vec<u8>,
    state: State,
}

impl PNG {
    const MAGIC_INIT: [u8; 8] = [0x89,0x50,0x4E,0x47,0x0D,0x0A,0x1A,0x0A];
    const MAGIC_END: [u8; 4] = [0x49,0x45,0x4E,0x44];

    pub fn new() -> PNG {
        PNG {
            position: 0,
            match_index: 0,
            png_bytes: Vec::new(),
            state: State::SearchingInit
        }
    }

    pub fn step(&mut self, byte: u8) -> Option<Vec<u8>> {
        match self.state {
            State::SearchingInit => {
                let found = self.check_init(byte);
                if found {
                    self.state = State::SearchingEnd;
                }
            }

            State::SearchingEnd => {
                self.png_bytes.push(byte);
                let found = self.check_end(byte);
                if found {
                    self.state = State::SearchingInit;
                    let mut complete_png_bytes: Vec<u8> = Vec::new();
                    complete_png_bytes.extend_from_slice(&Self::MAGIC_INIT);
                    complete_png_bytes.extend_from_slice(&self.png_bytes);

                    self.png_bytes = Vec::new();

                    return Some(complete_png_bytes);
                }
            }
        }

        self.position += 1;
        None
    }

    fn check_init(&mut self, byte: u8) -> bool {
        if self.match_index > PNG::MAGIC_INIT.len() - 1 {
            self.match_index = 0;
        }

        if Self::MAGIC_INIT[self.match_index] == byte {
            self.match_index += 1;
        } else {
            self.match_index = 0;
        }

        return self.match_index == PNG::MAGIC_INIT.len();
    }

    fn check_end(&mut self, byte: u8) -> bool {
        if self.match_index > PNG::MAGIC_END.len() - 1 {
            self.match_index = 0;
        }

        if Self::MAGIC_END[self.match_index] == byte {
            self.match_index += 1;
        } else {
            self.match_index = 0;
        }

        return self.match_index == PNG::MAGIC_END.len();
    }
}
