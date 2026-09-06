use std::io::{self, Read, Write};

use crate::{
    instruction::Instruction,
    ram::{Address, Ram},
    register::{Register, RegisterCode},
};

#[derive(Debug)]
pub struct Cpu<'ram> {
    r1: Register,
    r2: Register,
    r3: Register,
    r4: Register,
    r5: Register,
    r6: Register,
    r7: Register,
    r8: Register,

    pc: Register,
    ir: Register,

    flag_z: bool,
    flag_n: bool,
    flag_of: bool,

    ram: &'ram mut Ram,
}

impl<'cpu> Cpu<'cpu> {
    pub fn new(ram: &'cpu mut Ram) -> Self {
        Self {
            r1: Register::new(),
            r2: Register::new(),
            r3: Register::new(),
            r4: Register::new(),
            r5: Register::new(),
            r6: Register::new(),
            r7: Register::new(),
            r8: Register::new(),

            pc: Register::new(),
            ir: Register::new(),

            flag_z: false,
            flag_n: false,
            flag_of: false,

            ram,
        }
    }

    fn get_register(&self, code: RegisterCode) -> Option<&Register> {
        match code {
            0x00 => Some(&self.r1),
            0x01 => Some(&self.r2),
            0x02 => Some(&self.r3),
            0x03 => Some(&self.r4),
            0x04 => Some(&self.r5),
            0x05 => Some(&self.r6),
            0x06 => Some(&self.r7),
            0x07 => Some(&self.r8),

            0x80 => Some(&self.pc),
            0x81 => Some(&self.ir),

            _ => None,
        }
    }

    fn get_register_mut(&mut self, code: RegisterCode) -> Option<&mut Register> {
        match code {
            0x00 => Some(&mut self.r1),
            0x01 => Some(&mut self.r2),
            0x02 => Some(&mut self.r3),
            0x03 => Some(&mut self.r4),
            0x04 => Some(&mut self.r5),
            0x05 => Some(&mut self.r6),
            0x06 => Some(&mut self.r7),
            0x07 => Some(&mut self.r8),

            0x80 => Some(&mut self.pc),
            0x81 => Some(&mut self.ir),

            _ => None,
        }
    }

    fn fetch(&mut self) {
        let counter = self.pc.get();
        self.pc.set(counter + 1);

        let instruciton_code = self.ram.get(counter as Address);

        self.ir.set(instruciton_code);
    }

    fn decode(&mut self) -> Option<Instruction> {
        let instruction_code = self.ir.get() as u8;

        match instruction_code {
            0x00 => {
                let pc = self.pc.get();
                self.pc.set(pc + 3);

                let addr = pc as Address;

                let dest = self.ram.get(addr) as RegisterCode;
                let source = self.ram.get(addr + 1) as RegisterCode;
                let other = self.ram.get(addr + 2) as RegisterCode;

                Some(Instruction::Add {
                    dest,
                    source,
                    other,
                })
            }
            0x01 => {
                let pc = self.pc.get();
                self.pc.set(pc + 3);

                let addr = pc as Address;

                let dest = self.ram.get(addr) as RegisterCode;
                let source = self.ram.get(addr + 1) as RegisterCode;
                let other = self.ram.get(addr + 2) as RegisterCode;

                Some(Instruction::Sub {
                    dest,
                    source,
                    other,
                })
            }
            0x02 => {
                let pc = self.pc.get();
                self.pc.set(pc + 3);

                let addr = pc as Address;

                let dest = self.ram.get(addr) as RegisterCode;
                let source = self.ram.get(addr + 1) as RegisterCode;
                let other = self.ram.get(addr + 2) as RegisterCode;

                Some(Instruction::Mul {
                    dest,
                    source,
                    other,
                })
            }
            0x03 => {
                let pc = self.pc.get();
                self.pc.set(pc + 3);

                let addr = pc as Address;

                let dest = self.ram.get(addr) as RegisterCode;
                let source = self.ram.get(addr + 1) as RegisterCode;
                let other = self.ram.get(addr + 2) as RegisterCode;

                Some(Instruction::Div {
                    dest,
                    source,
                    other,
                })
            }

            0x40 => {
                let pc = self.pc.get();
                self.pc.set(pc + 3);

                let addr = pc as Address;

                let dest = self.ram.get(addr) as RegisterCode;
                let source = self.ram.get(addr + 1) as RegisterCode;
                let other = self.ram.get(addr + 2) as RegisterCode;

                Some(Instruction::And {
                    dest,
                    source,
                    other,
                })
            }
            0x41 => {
                let pc = self.pc.get();
                self.pc.set(pc + 3);

                let addr = pc as Address;

                let dest = self.ram.get(addr) as RegisterCode;
                let source = self.ram.get(addr + 1) as RegisterCode;
                let other = self.ram.get(addr + 2) as RegisterCode;

                Some(Instruction::Or {
                    dest,
                    source,
                    other,
                })
            }
            0x42 => {
                let pc = self.pc.get();
                self.pc.set(pc + 3);

                let addr = pc as Address;

                let dest = self.ram.get(addr) as RegisterCode;
                let source = self.ram.get(addr + 1) as RegisterCode;
                let other = self.ram.get(addr + 2) as RegisterCode;

                Some(Instruction::Xor {
                    dest,
                    source,
                    other,
                })
            }
            0x43 => {
                let pc = self.pc.get();
                self.pc.set(pc + 2);

                let addr = pc as Address;

                let dest = self.ram.get(addr) as RegisterCode;
                let source = self.ram.get(addr + 1) as RegisterCode;

                Some(Instruction::Not { dest, source })
            }
            0x44 => {
                let pc = self.pc.get();
                self.pc.set(pc + 3);

                let addr = pc as Address;

                let dest = self.ram.get(addr) as RegisterCode;
                let source = self.ram.get(addr + 1) as RegisterCode;
                let other = self.ram.get(addr + 2) as RegisterCode;

                Some(Instruction::Shl {
                    dest,
                    source,
                    other,
                })
            }
            0x45 => {
                let pc = self.pc.get();
                self.pc.set(pc + 3);

                let addr = pc as Address;

                let dest = self.ram.get(addr) as RegisterCode;
                let source = self.ram.get(addr + 1) as RegisterCode;
                let other = self.ram.get(addr + 2) as RegisterCode;

                Some(Instruction::Shr {
                    dest,
                    source,
                    other,
                })
            }

            0x04 => {
                let pc = self.pc.get();
                self.pc.set(pc + 1);

                let addr = pc as Address;

                let value = self.ram.get(addr) as RegisterCode;

                Some(Instruction::Cmp { value })
            }

            0x80 => {
                let pc = self.pc.get();
                self.pc.set(pc + 2);

                let addr = pc as Address;

                let dest = self.ram.get(addr) as RegisterCode;
                let addr = self.ram.get(addr + 1) as Address;

                Some(Instruction::Load { dest, addr })
            }
            0x81 => {
                let pc = self.pc.get();
                self.pc.set(pc + 2);

                let addr = pc as Address;

                let dest = self.ram.get(addr) as RegisterCode;
                let source = self.ram.get(addr + 1) as RegisterCode;

                Some(Instruction::Loadr { dest, source })
            }
            0x82 => {
                let pc = self.pc.get();
                self.pc.set(pc + 2);

                let addr = pc as Address;

                let source = self.ram.get(addr) as RegisterCode;
                let addr = self.ram.get(addr + 1) as Address;

                Some(Instruction::Store { source, addr })
            }
            0x83 => {
                let pc = self.pc.get();
                self.pc.set(pc + 2);

                let addr = pc as Address;

                let source = self.ram.get(addr) as RegisterCode;
                let dest = self.ram.get(addr + 1) as RegisterCode;

                Some(Instruction::Storer { source, dest })
            }

            0x90 => {
                let pc = self.pc.get();
                self.pc.set(pc + 2);

                let addr = pc as Address;

                let dest = self.ram.get(addr) as RegisterCode;
                let source = self.ram.get(addr + 1) as RegisterCode;

                Some(Instruction::Move { dest, source })
            }
            0x91 => {
                let pc = self.pc.get();
                self.pc.set(pc + 2);

                let addr = pc as Address;

                let dest = self.ram.get(addr) as RegisterCode;
                let value = self.ram.get(addr + 1);

                Some(Instruction::MoveIm { dest, value })
            }

            0xB1 => {
                let pc = self.pc.get();
                self.pc.set(pc + 2);

                let addr = pc as Address;

                let dest = self.ram.get(addr) as RegisterCode;
                let source = self.ram.get(addr + 1) as RegisterCode;

                Some(Instruction::Cmovz { dest, source })
            }
            0xB2 => {
                let pc = self.pc.get();
                self.pc.set(pc + 2);

                let addr = pc as Address;

                let dest = self.ram.get(addr) as RegisterCode;
                let source = self.ram.get(addr + 1) as RegisterCode;

                Some(Instruction::Cmovn { dest, source })
            }
            0xB3 => {
                let pc = self.pc.get();
                self.pc.set(pc + 2);

                let addr = pc as Address;

                let dest = self.ram.get(addr) as RegisterCode;
                let source = self.ram.get(addr + 1) as RegisterCode;

                Some(Instruction::Cmovof { dest, source })
            }

            0xC0 => {
                let pc = self.pc.get();
                self.pc.set(pc + 1);

                let addr = pc as Address;

                let port = self.ram.get(addr) as u8;

                Some(Instruction::In { port })
            }
            0xD0 => {
                let pc = self.pc.get();
                self.pc.set(pc + 1);

                let addr = pc as Address;

                let port = self.ram.get(addr) as u8;

                Some(Instruction::Out { port })
            }

            0xE0 => Some(Instruction::Halt),
            0xF0 => {
                let pc = self.pc.get();
                self.pc.set(pc + 1);

                let addr = self.ram.get(pc as Address) as Address;

                Some(Instruction::Jmp { addr })
            }
            0xF8 => {
                let pc = self.pc.get();
                self.pc.set(pc + 1);

                let addr = self.ram.get(pc as Address) as Address;

                Some(Instruction::Jz { addr })
            }
            0xFC => {
                let pc = self.pc.get();
                self.pc.set(pc + 1);

                let addr = self.ram.get(pc as Address) as Address;

                Some(Instruction::Jn { addr })
            }
            0xFE => {
                let pc = self.pc.get();
                self.pc.set(pc + 1);

                let addr = self.ram.get(pc as Address) as Address;

                Some(Instruction::Jof { addr })
            }

            _ => None,
        }
    }

    fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Add {
                dest,
                source,
                other,
            } => {
                let lhs = self
                    .get_register(source)
                    .expect("Invalid register code")
                    .get();
                let rhs = self
                    .get_register(other)
                    .expect("Invalid register code")
                    .get();

                let (value, overflown) = lhs.overflowing_add(rhs);
                self.flag_of = overflown;
                self.flag_n = value < 0;
                self.flag_z = value == 0;

                self.get_register_mut(dest)
                    .expect("Invalid register code")
                    .set(value);
            }
            Instruction::Sub {
                dest,
                source,
                other,
            } => {
                let lhs = self
                    .get_register(source)
                    .expect("Invalid register code")
                    .get();
                let rhs = self
                    .get_register(other)
                    .expect("Invalid register code")
                    .get();

                let (value, overflow) = lhs.overflowing_sub(rhs);
                self.flag_of = overflow;
                self.flag_n = value < 0;
                self.flag_z = value == 0;

                self.get_register_mut(dest)
                    .expect("Invalid register code")
                    .set(value);
            }
            Instruction::Mul {
                dest,
                source,
                other,
            } => {
                let lhs = self
                    .get_register(source)
                    .expect("Invalid register code")
                    .get();
                let rhs = self
                    .get_register(other)
                    .expect("Invalid register code")
                    .get();

                let (value, overflow) = lhs.overflowing_mul(rhs);
                self.flag_of = overflow;
                self.flag_n = value < 0;
                self.flag_z = value == 0;

                self.get_register_mut(dest)
                    .expect("Invalid register code")
                    .set(value);
            }
            Instruction::Div {
                dest,
                source,
                other,
            } => {
                let lhs = self
                    .get_register(source)
                    .expect("Invalid register code")
                    .get();
                let rhs = self
                    .get_register(other)
                    .expect("Invalid register code")
                    .get();

                if rhs == 0 {
                    panic!("Division by zero");
                }

                let (value, overflow) = lhs.overflowing_div(rhs);
                self.flag_of = overflow;
                self.flag_n = value < 0;
                self.flag_z = value == 0;

                self.get_register_mut(dest)
                    .expect("Invalid register code")
                    .set(value);
            }

            Instruction::And {
                dest,
                source,
                other,
            } => {
                let lhs = self
                    .get_register(source)
                    .expect("Invalid register code")
                    .get();
                let rhs = self
                    .get_register(other)
                    .expect("Invalid register code")
                    .get();

                let value = lhs & rhs;
                self.flag_n = value < 0;
                self.flag_z = value == 0;

                self.get_register_mut(dest)
                    .expect("Invalid register code")
                    .set(value);
            }
            Instruction::Or {
                dest,
                source,
                other,
            } => {
                let lhs = self
                    .get_register(source)
                    .expect("Invalid register code")
                    .get();
                let rhs = self
                    .get_register(other)
                    .expect("Invalid register code")
                    .get();

                let value = lhs | rhs;
                self.flag_n = value < 0;
                self.flag_z = value == 0;

                self.get_register_mut(dest)
                    .expect("Invalid register code")
                    .set(value);
            }
            Instruction::Xor {
                dest,
                source,
                other,
            } => {
                let lhs = self
                    .get_register(source)
                    .expect("Invalid register code")
                    .get();
                let rhs = self
                    .get_register(other)
                    .expect("Invalid register code")
                    .get();

                let value = lhs ^ rhs;
                self.flag_n = value < 0;
                self.flag_z = value == 0;

                self.get_register_mut(dest)
                    .expect("Invalid register code")
                    .set(value);
            }
            Instruction::Not { dest, source } => {
                let val = self
                    .get_register(source)
                    .expect("Invalid register code")
                    .get();

                let value = !val;
                self.flag_n = value < 0;
                self.flag_z = value == 0;

                self.get_register_mut(dest)
                    .expect("Invalid register code")
                    .set(value);
            }
            Instruction::Shl {
                dest,
                source,
                other,
            } => {
                let lhs = self
                    .get_register(source)
                    .expect("Invalid register code")
                    .get();
                let rhs = self
                    .get_register(other)
                    .expect("Invalid register code")
                    .get();

                let (value, overflow) = lhs.overflowing_shl(rhs as u8 as u32);
                self.flag_of = overflow;
                self.flag_n = value < 0;
                self.flag_z = value == 0;

                self.get_register_mut(dest)
                    .expect("Invalid register code")
                    .set(value);
            }
            Instruction::Shr {
                dest,
                source,
                other,
            } => {
                let lhs = self
                    .get_register(source)
                    .expect("Invalid register code")
                    .get();
                let rhs = self
                    .get_register(other)
                    .expect("Invalid register code")
                    .get();

                let (value, overflow) = lhs.overflowing_shr(rhs as u8 as u32);
                self.flag_of = overflow;
                self.flag_n = value < 0;
                self.flag_z = value == 0;

                self.get_register_mut(dest)
                    .expect("Invalid register code")
                    .set(value);
            }

            Instruction::Cmp { value } => {
                let val = self
                    .get_register(value)
                    .expect("Invalid register code")
                    .get();

                self.flag_z = val == 0;
                self.flag_n = val < 0;
            }

            Instruction::Load { dest, addr } => {
                let value = self.ram.get(addr);

                self.flag_n = value < 0;
                self.flag_z = value == 0;

                self.get_register_mut(dest)
                    .expect("Invalid register code")
                    .set(value);
            }
            Instruction::Loadr { dest, source } => {
                let addr = self
                    .get_register(source)
                    .expect("Invalid register code")
                    .get() as Address;
                let value = self.ram.get(addr);
                self.flag_n = value < 0;
                self.flag_z = value == 0;
                self.get_register_mut(dest)
                    .expect("Invalid register code")
                    .set(value);
            }
            Instruction::Store { source, addr } => {
                let value = self
                    .get_register(source)
                    .expect("Invalid register code")
                    .get();
                self.flag_n = value < 0;
                self.flag_z = value == 0;
                self.ram.set(addr, value);
            }
            Instruction::Storer { source, dest } => {
                let value = self
                    .get_register(source)
                    .expect("Invalid register code")
                    .get();
                self.flag_n = value < 0;
                self.flag_z = value == 0;
                let addr = self
                    .get_register(dest)
                    .expect("Invalid register code")
                    .get() as Address;
                self.ram.set(addr, value);
            }

            Instruction::Move { dest, source } => {
                let value = self
                    .get_register(source)
                    .expect("Invalid register code")
                    .get();
                self.flag_n = value < 0;
                self.flag_z = value == 0;
                self.get_register_mut(dest)
                    .expect("Invalid register code")
                    .set(value);
            }
            Instruction::MoveIm { dest, value } => {
                self.flag_n = value < 0;
                self.flag_z = value == 0;
                self.get_register_mut(dest)
                    .expect("Invalid register code")
                    .set(value);
            }

            Instruction::Cmovz { dest, source } => {
                if self.flag_z {
                    let value = self
                        .get_register(source)
                        .expect("Invalid register code")
                        .get();
                    self.flag_n = value < 0;
                    self.flag_z = value == 0;
                    self.get_register_mut(dest)
                        .expect("Invalid register code")
                        .set(value);
                }
            }
            Instruction::Cmovn { dest, source } => {
                if self.flag_n {
                    let value = self
                        .get_register(source)
                        .expect("Invalid register code")
                        .get();
                    self.flag_n = value < 0;
                    self.flag_z = value == 0;
                    self.get_register_mut(dest)
                        .expect("Invalid register code")
                        .set(value);
                }
            }
            Instruction::Cmovof { dest, source } => {
                if self.flag_of {
                    let value = self
                        .get_register(source)
                        .expect("Invalid register code")
                        .get();
                    self.flag_n = value < 0;
                    self.flag_z = value == 0;
                    self.get_register_mut(dest)
                        .expect("Invalid register code")
                        .set(value);
                }
            }

            #[allow(clippy::single_match)]
            Instruction::In { port } => match port {
                0 => {
                    let mut buf = [0u8; 1];
                    io::stdin().read_exact(&mut buf).unwrap_or(());
                    let value = buf[0] as i8;
                    self.flag_n = value < 0;
                    self.flag_z = value == 0;

                    self.r1.set(value);
                }
                _ => (),
            },

            #[allow(clippy::single_match)]
            Instruction::Out { port } => match port {
                0 => {
                    let value = self.r1.get() as u8 as char;
                    self.flag_n = (value as u8 as i8) < 0;
                    self.flag_z = (value as u8 as i8) == 0;
                    print!("{value}");
                    let _ = io::stdout().flush();
                }
                _ => (),
            },

            #[allow(clippy::empty_loop)]
            Instruction::Halt => loop {},
            Instruction::Jmp { addr } => {
                self.pc.set(addr as i8);
            }
            Instruction::Jz { addr } => {
                if self.flag_z {
                    self.pc.set(addr as i8);
                }
            }
            Instruction::Jn { addr } => {
                if self.flag_n {
                    self.pc.set(addr as i8);
                }
            }
            Instruction::Jof { addr } => {
                if self.flag_of {
                    self.pc.set(addr as i8);
                }
            }
        }
    }

    pub fn cycle(&mut self) {
        loop {
            // self.debug();

            self.fetch();
            let instruction = self.decode().expect("Invalid instruction");
            self.execute(instruction);
        }
    }

    #[allow(unused)]
    fn debug(&self) {
        println!("CPU State:");

        println!("r1: {}", self.r1.get());
        println!("r2: {}", self.r2.get());
        println!("r3: {}", self.r3.get());
        println!("r4: {}", self.r4.get());
        println!("r5: {}", self.r5.get());
        println!("r6: {}", self.r6.get());
        println!("r7: {}", self.r7.get());
        println!("r8: {}", self.r8.get());
        println!("program counter: {}", self.pc.get());
        println!("instruction register: {}", self.ir.get());
    }
}
