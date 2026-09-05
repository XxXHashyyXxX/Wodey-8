use crate::{
    cpu::Cpu,
    ram::{Address, Ram},
};

mod cpu;
mod instruction;
mod ram;
mod register;

fn main() {
    let mut ram = Ram::new();

    let mem = [0x80u8, 0x00, 0x06, 0xD0, 0x00, 0xE0, 0x63];
    for (i, byte) in mem.iter().enumerate() {
        ram.set(i as Address, *byte as i8);
    }

    let mut cpu = Cpu::new(&mut ram);

    cpu.cycle();
}
