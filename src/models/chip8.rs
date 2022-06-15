use rand::{prelude::ThreadRng, Rng};

use crate::{font::FONT, input::keyboard::Keyboard};

use super::{errors::ChipErrors, opcode::Opcode};

pub const CHIP8_WIDTH: usize = 64;
pub const CHIP8_HEIGHT: usize = 32;

pub struct Chip8 {
    pc: u16,
    opcode: u16,
    I: u16,
    sp: u16,
    memory: [u8; 4096],
    delay_timer: u8,
    sound_timer: u8,
    gfx: [[u8; CHIP8_WIDTH]; CHIP8_HEIGHT],
    stack: [u16; 16],
    V: [u8; 16],
    rnd: ThreadRng,
    keyboard_register: u8,
    keyboard_waiting: bool,
}

pub struct CycleResult<'a> {
    pub gfx: &'a [[u8; CHIP8_WIDTH]; CHIP8_HEIGHT],
    pub draw_update: bool,
}

impl Chip8 {
    pub fn new(program: Vec<u8>) -> Self {
        let mut memory = [0; 4096];
        for i in 0..FONT.len() {
            memory[i] = FONT[i];
        }

        for i in 0..program.len() {
            memory[512 + i] = program[i];
        }

        Chip8 {
            pc: 0x200,
            opcode: 0,
            I: 0,
            sp: 0,
            memory,
            delay_timer: 0,
            sound_timer: 0,
            gfx: [[0; 64]; 32],
            stack: [0; 16],
            V: [0; 16],
            rnd: rand::thread_rng(),
            keyboard_register: 0,
            keyboard_waiting: false,
        }
    }

    pub fn emulateCycle(&mut self, keyboard: &Keyboard) -> Result<CycleResult, ChipErrors> {
        if self.keyboard_waiting {
            println!("Waiting keyboard: {keyboard:?}");

            if let Some(key) = keyboard.get_pressed_key() {
                self.keyboard_waiting = false;
                self.V[self.keyboard_register as usize] = key;
            }

            return Ok(CycleResult {
                draw_update: false,
                gfx: &self.gfx,
            });
        }

        self.opcode = (self.memory[self.pc as usize] as u16) << 8
            | (self.memory[self.pc as usize + 1] as u16);
        println!("Executing code {:04x}", self.opcode);
        let operation = Opcode::parse(self.opcode)?;
        let mut draw_update = false;

        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }

        if self.sound_timer > 0 {
            self.sound_timer -= 1;

            if self.sound_timer == 0 {
                println!("BEEP");
            }
        }

        match operation {
            Opcode::SetI(value) => {
                println!("Set I to {:03x}", value);
                self.I = value;
                self.pc += 2;
            }
            Opcode::SetVConstant(x, n) => {
                self.V[x as usize] = n;
                self.pc += 2;
            }
            Opcode::SetV(x, y) => {
                self.V[x as usize] = self.V[y as usize];
                self.pc += 2;
            }
            Opcode::Or(x, y) => {
                self.V[x as usize] |= self.V[y as usize];
                self.pc += 2;
            }
            Opcode::And(x, y) => {
                self.V[x as usize] &= self.V[y as usize];
                self.pc += 2;
            }
            Opcode::Xor(x, y) => {
                self.V[x as usize] ^= self.V[y as usize];
                self.pc += 2;
            }
            Opcode::ShiftLeft(x) => {
                self.V[0xF] = (self.V[x as usize] & 0b10000000) >> 7;
                self.V[x as usize] = self.V[x as usize].wrapping_shl(1);
                self.pc += 2;
            }
            Opcode::ShiftRight(x) => {
                self.V[0xF] = self.V[x as usize] & 0b1;
                self.V[x as usize] = self.V[x as usize].wrapping_shr(1);
                self.pc += 2;
            }
            Opcode::SetDelayTimer(x) => {
                self.delay_timer = self.V[x as usize];
                self.pc += 2;
            }
            Opcode::Dump(x) => {
                for i in 0..=(x as usize) {
                    self.memory[self.I as usize + i] = self.V[i];
                }
                self.pc += 2;
            }
            Opcode::Load(x) => {
                for i in 0..=(x as usize) {
                    self.V[i] = self.memory[self.I as usize + i];
                }
                self.pc += 2;
            }
            Opcode::SpriteAddress(x) => {
                self.I = self.V[x as usize] as u16 * 5;
                self.pc += 2;
            }
            Opcode::GetKey(x) => {
                self.keyboard_waiting = true;
                self.keyboard_register = x;
                self.pc += 2;
            }
            Opcode::Subtract(x, y) => {
                self.V[0xF] = if self.V[x as usize] > self.V[y as usize] {
                    1
                } else {
                    0
                };
                self.V[x as usize] = self.V[x as usize].wrapping_sub(self.V[y as usize]);
                self.pc += 2;
            }
            Opcode::SubtractOpposite(x, y) => {
                self.V[0xF] = if self.V[x as usize] < self.V[y as usize] {
                    1
                } else {
                    0
                };
                self.V[x as usize] = self.V[y as usize].wrapping_sub(self.V[x as usize]);
                self.pc += 2;
            }
            Opcode::ClearScreen => {
                println!("Clearing screen");
                self.pc += 2;
                self.gfx = [[0; CHIP8_WIDTH]; CHIP8_HEIGHT];
                draw_update = true;
            }
            Opcode::ReturnFromSubroutine => {
                self.sp -= 1;
                self.pc = self.stack[self.sp as usize];
            }
            Opcode::CallSubroutine(addr) => {
                self.stack[self.sp as usize] = self.pc + 2;
                self.sp += 1;
                self.pc = addr;
            }
            Opcode::Add(x, y) => {
                let sum = self.V[x as usize] as u16 + self.V[y as usize] as u16;

                if sum > 0xFF {
                    self.V[0xF] = 1;
                } else {
                    self.V[0xF] = 0;
                }

                self.V[x as usize] = (sum & 0xFF) as u8;
                self.pc += 2;
            }
            Opcode::AddConstant(x, n) => {
                let total = self.V[x as usize] as u16 + n as u16;
                self.V[x as usize] = (total & 0xFF) as u8;
                self.pc += 2;
            }
            Opcode::RandAnd(x, n) => {
                self.V[x as usize] = self.rnd.gen::<u8>() & n;
                self.pc += 2;
            }
            Opcode::GetDelayTimer(x) => {
                self.V[x as usize] = self.delay_timer;
                self.pc += 2;
            }
            Opcode::AddMemory(x) => {
                self.I += self.V[x as usize] as u16;
                self.pc += 2;
            }
            Opcode::Jump(addr) => {
                self.pc = addr;
            }
            Opcode::JumpPlus(addr) => {
                self.pc = addr + self.V[0] as u16;
            }
            Opcode::BinaryCodedDecimal(x) => {
                let value = self.V[x as usize];
                self.memory[self.I as usize] = value / 100;
                self.memory[self.I as usize + 1] = value / 10 % 10;
                self.memory[self.I as usize + 2] = value % 10;
                self.pc += 2;
            }
            Opcode::SkipRegistersEqual(x, y) => {
                if self.V[x as usize] == self.V[y as usize] {
                    self.pc += 2;
                }

                self.pc += 2;
            }
            Opcode::SkipRegistersNonEqual(x, y) => {
                if self.V[x as usize] != self.V[y as usize] {
                    self.pc += 2;
                }

                self.pc += 2;
            }
            Opcode::SkipEqual(x, n) => {
                if self.V[x as usize] == n {
                    self.pc += 2;
                }

                self.pc += 2;
            }
            Opcode::SkipNonEqual(x, n) => {
                if self.V[x as usize] != n {
                    self.pc += 2;
                }

                self.pc += 2;
            }
            Opcode::Draw(x, y, n) => {
                self.V[0xF] = 0;
                for line in 0..n {
                    let pixel = self.memory[(self.I + line as u16) as usize];

                    for column in 0..8 {
                        if (pixel & (0x80 >> column)) != 0 {
                            let x = (self.V[x as usize] + column) as usize;
                            let y = (self.V[y as usize] + line) as usize;
                            if self.gfx[y][x] == 1 {
                                self.V[0xF] = 1;
                            }
                            self.gfx[y][x] ^= 1;
                        }
                    }
                }

                self.pc += 2;
                draw_update = true;
            }
        }

        Ok(CycleResult {
            draw_update,
            gfx: &self.gfx,
        })
    }
}
