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

    pub fn clear(&mut self) {
        (0..MEM_MAX).for_each(|i| self.data[i] = 0);
    }

    pub fn get_byte(&self, idx: usize) -> Byte {
        self.data[idx]
    }

    pub fn insert_byte(&mut self, idx: usize, data: Byte) {
        self.data[idx] = data;
    }

    pub fn at(&mut self, idx: usize) -> &mut Byte {
        &mut self.data[idx]
    }

    pub fn write_word(&mut self, addr: usize, data: Word) {
        self.data[addr] = (data & 0xFF) as Byte;
        self.data[addr + 1] = (data >> 8) as Byte;
    }

}

