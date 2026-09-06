#[derive(Debug)]
pub struct Ram {
    memory: [i8; 256],
}

pub type Address = u8;

impl Ram {
    pub fn new() -> Self {
        Self { memory: [0; 256] }
    }

    pub fn get(&self, addr: Address) -> i8 {
        self.memory[addr as usize]
    }

    pub fn set(&mut self, addr: Address, value: i8) {
        self.memory[addr as usize] = value;
    }
}
