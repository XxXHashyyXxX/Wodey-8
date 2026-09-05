pub struct Register(i8);

impl Register {
    pub fn new() -> Self {
        Self(0)
    }

    pub fn get(&self) -> i8 {
        self.0
    }

    pub fn set(&mut self, value: i8) {
        self.0 = value;
    }
}

pub type RegisterCode = u8;
