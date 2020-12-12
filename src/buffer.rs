const BUFFER_SIZE: usize = 512;

pub struct BytesBuffer {
    index: usize,
    buffer: [u8; BUFFER_SIZE],
}

impl BytesBuffer {
    /// new return a new BytesBuffer
    pub fn new() -> Self {
        Self {
            index: 0,
            buffer: [0; BUFFER_SIZE],
        }
    }

    pub fn push(&mut self, data: u8) -> Result<(), ()> {
        if self.index < BUFFER_SIZE {
            self.buffer[self.index] = data;
            self.index += 1;
            return Ok(());
        }
        Err(())
    }

    pub fn read(&mut self) -> Option<Self> {
        if self.index > 0 {
            let tmp = self.index;
            self.index = 0;
            Some(Self {
                index: tmp,
                buffer: self.buffer,
            })
        } else {
            None
        }
    }

    pub fn clear(&mut self) {
        self.index = 0;
    }

    pub fn check_end(&mut self) -> Option<([u8; BUFFER_SIZE], usize)> {
        if self.buffer[self.index - 1] == 0x0A {
            if self.buffer[self.index - 2] == 0x0D {
                return Some((self.buffer, self.index));
            }
        }
        None
    }
}
