pub const MEM_MAX: usize = 1024 * 64;

type Byte = u8;
type Word = u16;

pub struct Memory {
    data: [u8; MEM_MAX],
}

#[allow(dead_code)]
impl Memory {
    pub fn new() -> Self {
        Self { data: [0; MEM_MAX] }
    }

    pub fn get_byte(&self, idx: usize) -> Byte {
        self.data[idx]
    }

    pub fn at(&mut self, idx: usize) -> &mut Byte {
        &mut self.data[idx]
    }

}

