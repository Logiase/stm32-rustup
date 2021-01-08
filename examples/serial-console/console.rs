use crate::command::Commands;

pub const BUFFER_SIZE: usize = 256;

const CR: u8 = '\r' as u8;
const LF: u8 = '\n' as u8;

pub struct Console {
    buf: [u8; BUFFER_SIZE],
    pos: usize,
}

impl Console {
    pub fn new() -> Console {
        Console {
            buf: [0; BUFFER_SIZE],
            pos: 0,
        }
    }

    pub fn push(&mut self, b: u8) -> Option<Commands> {
        if self.pos == BUFFER_SIZE {
            return None;
        }
        if b == CR || b == LF {
            if self.pos == 0 {
                return None;
            }
            let result = &self.buf[0..self.pos];
            match result {
                b"command1" => {
                    return Commands::Command1.into();
                }
                b"command2" => {
                    return Commands::Command2.into();
                }
                _ => {
                    return Commands::NotFound.into();
                }
            }
        }
        self.buf[self.pos] = b;
        self.pos += 1;
        None
    }
}
