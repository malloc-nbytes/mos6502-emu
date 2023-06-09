pub const MEM_MAX: usize = 1024 * 64;

type Byte = u8;

pub struct Memory {
    data: [u8; MEM_MAX],
}

impl Memory {
    pub fn new() -> Self {
        Self { data: [0; MEM_MAX] }
    }

    pub fn get_byte(&mut self, idx: usize) -> &mut Byte {
        &mut self.data[idx]
    }

}

