use super::{opcodes::Op, screen::Screen};

use std::u16;

use rand::prelude::*;

const MEM_SIZE: usize = 4096;

const DIGIT_SPRITES: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

pub struct Cpu {
    v: [u8; 16],
    i: u16,
    sound_timer: u16,
    delay_timer: u16,
    stack: [u16; 16],
    pc: usize,
    sp: usize,
    memory: [u8; MEM_SIZE],
    screen: Screen,
}

impl Cpu {
    pub fn new() -> Cpu {
        let mut cpu = Cpu {
            v: [0; 16],
            i: 0x200,
            sound_timer: 0,
            delay_timer: 0,
            stack: [0; 16],
            pc: 0x200,
            sp: 0,
            memory: [0; MEM_SIZE],
            screen: Screen::new(),
        };

        cpu.memory[0..80].copy_from_slice(&DIGIT_SPRITES);

        cpu
    }


    fn compute_op(&mut self, op: Op) {
        match op {
            Op::Cls => self.screen.clear(),
            Op::Ret => self.return_from_subroutine(),
            Op::Jp(n) => self.pc = n as usize,
            Op::Call(n) => self.call(n),
            Op::Se(reg, val) => self.skip_equals(reg, val),
            Op::Sne(reg, val) => self.skip_not_equals(reg, val),
            Op::SeReg(r1, r2) => self.skip_reg_equals(r1, r2),
            Op::SneReg(r1, r2) => self.skip_reg_not_equals(r1, r2),
            Op::Ld(reg, val) => self.load(reg, val),
            Op::Add(reg, val) => self.add(reg, val),
            Op::LdReg(r1, r2) => self.load_reg(r1, r2),
            Op::Or(r1, r2) => self.or(r1, r2),
            Op::And(r1, r2) => self.and(r1, r2),
            Op::Xor(r1, r2) => self.xor(r1, r2),
            Op::AddReg(r1, r2) => self.add_reg(r1, r2),
            Op::Sub(r1, r2) => self.sub(r1, r2),
            Op::Shr(reg) => self.shr(reg),
            Op::Subn(r1, r2) => self.sub(r2, r1),
            Op::Shl(reg) => self.shl(reg),
            Op::LdI(val) => self.i = val,
            Op::JpRegI(addr) => self.jp_reg_i(addr),
            Op::Rnd(reg, mask) => self.rnd(reg, mask),
            _ => panic!("Not supported !")
        }
        self.update_pc(op);
    }

    fn update_pc(&mut self, previous_op: Op) {
        match previous_op {
            Op::Ret | Op::Jp(..) | Op::Call(..) | Op::Se(..) | Op::Sne(..) | Op::SeReg(..)
            | Op::SneReg(..) | Op::JpRegI(..) => (),
            _ => self.pc += 2
        }
    }

    fn return_from_subroutine(&mut self) {
        if self.sp == 0 {
            panic!("Invalid state: sp = 0 !");
        }
        self.pc = self.stack[self.sp] as usize;
        self.sp -= 1;
    }

    fn call(&mut self, address: u16) {
        if self.pc > MEM_SIZE as usize {
            panic!("Invald pc value: {}", self.pc);
        }
        if address >= MEM_SIZE as u16 {
            panic!("Invalid jump address: {}", address)
        }
        self.sp += 1;
        self.stack[self.sp] = self.pc as u16;
        self.pc = address as usize;
    }

    fn skip_equals(&mut self, reg: u8, val: u8) {
        self.pc += if self.reg(reg) as u8 == val { 4 } else { 2 };
    }

    fn skip_not_equals(&mut self, reg: u8, val: u8) {
        self.pc += if self.reg(reg) as u8 != val { 4 } else { 2 };
    }

    fn skip_reg_equals(&mut self, r1: u8, r2: u8) {
        self.pc += if self.reg(r1) as u8 == self.reg(r2) { 4 } else { 2 };
    }

    fn skip_reg_not_equals(&mut self, r1: u8, r2: u8) {
        self.pc += if self.reg(r1) as u8 != self.reg(r2) { 4 } else { 2 };
    }

    fn load(&mut self, register: u8, val: u8) {
        self.v[register as usize] = val;
    }

    fn add(&mut self, register: u8, val: u8) {
        self.v[register as usize] += val;
    }

    fn load_reg(&mut self, r1: u8, r2: u8) {
        self.v[r1 as usize] = self.reg(r2);
    }

    fn or(&mut self, r1: u8, r2: u8) {
        self.v[r1 as usize] = self.reg(r1) | self.reg(r2);
    }

    fn and(&mut self, r1: u8, r2: u8) {
        self.v[r1 as usize] = self.reg(r1) & self.reg(r2);
    }

    fn xor(&mut self, r1: u8, r2: u8) {
        self.v[r1 as usize] = self.reg(r1) ^ self.reg(r2);
    }

    fn add_reg(&mut self, r1: u8, r2: u8) {
        let result = self.reg(r1) as u16 + self.reg(r2) as u16;
        self.v[r1 as usize] = result as u8;
        self.v[0xF] = (result > 255) as u8;
    }

    fn sub(&mut self, r1: u8, r2: u8) {
        self.v[0xF] = (self.reg(r1) > self.reg(r2)) as u8;
        self.v[r1 as usize] = (self.reg(r1) as i8 - self.reg(r2) as i8) as u8;
    }

    fn shr(&mut self, register: u8) {
        self.v[0xF] = self.reg(register) & 1;
        self.v[register as usize] = self.reg(register) >> 1;
    }

    fn shl(&mut self, register: u8) {
        self.v[0xF] = if (self.reg(register) & 0b10000000) != 0 { 1 } else { 0 } ;
        self.v[register as usize] = self.reg(register) << 1;
    }

    fn jp_reg_i(&mut self, addr: u16) {
        let address = self.v[0] as u16 + addr;
        self.pc = address as usize;
    }

    fn rnd(&mut self, reg: u8, mask: u8) {
        let random_val = random::<u8>() & mask;
        self.v[reg as usize] = random_val;
    }

    fn draw(&mut self, x: u8, y: u8, size: u8) {
        let address = self.i as usize;
        let sprite = &self.memory[address.. address + size as usize];
        self.v[0xF] = 0;

        for (line, byte) in sprite.iter().enumerate() {
            for offset in 0..8 {
                let screen_x = x as usize + offset as usize;
                let screen_y = y as usize + line;
                let on = (byte & (128 >> offset)) == 1;
                if self.screen.set_pixel_value(screen_x, screen_y, on) {
                    self.v[0xF] = 1;
                }
            }
        }
    }

    pub fn load_program(&mut self, program: &[u8]) -> Result<(), String> {
        if program.len() > MEM_SIZE - 0x200 {
            return Err("Program is too big !".to_string());
        }
        self.memory[0x200..].copy_from_slice(program);
        Ok(())
    }

    fn update_timers(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
        if self.sound_timer > 0 {
            self.sound_timer -= 1;
        }
    }

    fn reg<T: Into<usize>>(&self, register: T) -> u8 {
        self.v[register.into()]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new() {
        let cpu = Cpu::new();
        assert_eq!([0; 16], cpu.v);
        assert_eq!(0x200, cpu.i);
        assert_eq!(0, cpu.sound_timer);
        assert_eq!(0, cpu.delay_timer);
        assert_eq!([0; 16], cpu.stack);
        assert_eq!(0x200, cpu.pc);
        assert_eq!(0, cpu.sp);

        for i in 80..MEM_SIZE {
            if i < 80 {
                assert_eq!(DIGIT_SPRITES[i], cpu.memory[i])
            } else {
                assert_eq!(0, cpu.memory[i]);
            }
        }
    }

    #[test]
    fn update_timers() {
        let mut cpu = Cpu::new();
        cpu.sound_timer = 11;
        cpu.delay_timer = 5;
        cpu.update_timers();
        assert_eq!(10, cpu.sound_timer);
        assert_eq!(4, cpu.delay_timer);
    }

    #[test]
    fn update_timers_when_zero() {
        let mut cpu = Cpu::new();
        cpu.update_timers();
        assert_eq!(0, cpu.sound_timer);
        assert_eq!(0, cpu.delay_timer);
    }

    #[test]
    fn ret() {
        let mut cpu = Cpu::new();
        cpu.stack[0] = 5;
        cpu.stack[1] = 6;
        cpu.sp = 1;
        cpu.compute_op(Op::Ret);
        assert_eq!(6, cpu.pc);
        assert_eq!(0, cpu.sp);
    }

    #[test]
    #[should_panic]
    fn ret_when_sp_is_0() {
        let mut cpu = Cpu::new();
        cpu.stack[0] = 5;
        cpu.sp = 0;
        cpu.compute_op(Op::Ret);
    }

    #[test]
    fn jp() {
        let mut cpu = Cpu::new();
        cpu.compute_op(Op::Jp(5));
        assert_eq!(5, cpu.pc)
    }

    #[test]
    fn call() {
        let mut cpu = Cpu::new();
        cpu.pc = 55;
        cpu.compute_op(Op::Call(75));
        assert_eq!(75, cpu.pc);
        assert_eq!(1, cpu.sp);
        assert_eq!(55, cpu.stack[1]);
    }

    #[test]
    #[should_panic]
    fn call_invalid_pc() {
        let mut cpu = Cpu::new();
        cpu.pc = 5000;
        cpu.compute_op(Op::Call(0));
    }

    #[test]
    #[should_panic]
    fn call_invalid_address() {
        let mut cpu = Cpu::new();
        cpu.pc = 0x200;
        cpu.compute_op(Op::Call(MEM_SIZE as u16));
    }

    #[test]
    fn se() {
        let mut cpu = Cpu::new();
        cpu.v[1] = 5;
        cpu.compute_op(Op::Se(1, 5));
        assert_eq!(0x204, cpu.pc);
    }

    #[test]
    fn se_not_equals() {
        let mut cpu = Cpu::new();
        cpu.v[1] = 5;
        cpu.compute_op(Op::Se(1, 6));
        assert_eq!(0x202, cpu.pc);
    }

    #[test]
    fn sne() {
        let mut cpu = Cpu::new();
        cpu.v[1] = 5;
        cpu.compute_op(Op::Sne(1, 6));
        assert_eq!(0x204, cpu.pc);
    }

    #[test]
    fn sne_equals() {
        let mut cpu = Cpu::new();
        cpu.v[1] = 5;
        cpu.compute_op(Op::Sne(1, 5));
        assert_eq!(0x202, cpu.pc);
    }

    #[test]
    fn se_reg() {
        let mut cpu = Cpu::new();
        cpu.v[0] = 5;
        cpu.v[5] = 5;
        cpu.compute_op(Op::SeReg(0, 5));
        assert_eq!(0x204, cpu.pc);
    }

    #[test]
    fn se_reg_not_equals() {
        let mut cpu = Cpu::new();
        cpu.v[0] = 5;
        cpu.v[5] = 6;
        cpu.compute_op(Op::SeReg(0, 5));
        assert_eq!(0x202, cpu.pc);
    }

    #[test]
    fn sne_reg() {
        let mut cpu = Cpu::new();
        cpu.v[0] = 5;
        cpu.v[5] = 6;
        cpu.compute_op(Op::SneReg(0, 5));
        assert_eq!(0x204, cpu.pc);
    }

    #[test]
    fn sne_reg_equals() {
        let mut cpu = Cpu::new();
        cpu.v[0] = 5;
        cpu.v[5] = 5;
        cpu.compute_op(Op::SneReg(0, 5));
        assert_eq!(0x202, cpu.pc);
    }

    #[test]
    fn ld() {
        let mut cpu = Cpu::new();
        cpu.compute_op(Op::Ld(6, 124));
        assert_eq!(124, cpu.v[6]);
        assert_eq!(0x202, cpu.pc);
    }

    #[test]
    fn add() {
        let mut cpu = Cpu::new();
        cpu.v[9] = 10;
        cpu.compute_op(Op::Add(9, 10));
        assert_eq!(20, cpu.v[9]);
        assert_eq!(0x202, cpu.pc);
    }

    #[test]
    fn load_reg() {
        let mut cpu = Cpu::new();
        cpu.v[5] = 11;
        cpu.compute_op(Op::LdReg(1, 5));
        assert_eq!(11, cpu.v[1]);
        assert_eq!(0x202, cpu.pc);
    }

    #[test]
    fn or() {
        let mut cpu = Cpu::new();
        cpu.v[0xA] = 0b10100;
        cpu.v[0xB] = 0b01010;
        cpu.compute_op(Op::Or(0xA, 0xB));
        assert_eq!(0b11110, cpu.v[0xA]);
        assert_eq!(0x202, cpu.pc);
    }

    #[test]
    fn and() {
        let mut cpu = Cpu::new();
        cpu.v[3] = 0b10011;
        cpu.v[4] = 0b01110;
        cpu.compute_op(Op::And(3, 4));
        assert_eq!(0b00010, cpu.v[3]);
        assert_eq!(0x202, cpu.pc);
    }

    #[test]
    fn xor() {
        let mut cpu = Cpu::new();
        cpu.v[0] = 0b110;
        cpu.v[1] = 0b101;
        cpu.compute_op(Op::Xor(0, 1));
        assert_eq!(0b011, cpu.v[0]);
        assert_eq!(0x202, cpu.pc);
    }

    #[test]
    fn add_reg() {
        let mut cpu = Cpu::new();
        cpu.v[1] = 5;
        cpu.v[2] = 6;
        cpu.compute_op(Op::AddReg(1, 2));
        assert_eq!(11, cpu.v[1]);
        assert_eq!(0, cpu.v[0xF]);
        assert_eq!(0x202, cpu.pc);
    }

    #[test]
    fn add_reg_overflow() {
        let mut cpu = Cpu::new();
        cpu.v[1] = 128;
        cpu.v[2] = 128;
        cpu.compute_op(Op::AddReg(1, 2));
        assert_eq!(0, cpu.v[1]);
        assert_eq!(1, cpu.v[0xF]);
        assert_eq!(0x202, cpu.pc);
    }

    #[test]
    fn sub() {
        let mut cpu = Cpu::new();
        cpu.v[1] = 5;
        cpu.v[2] = 6;
        cpu.compute_op(Op::Sub(2, 1));
        assert_eq!(1, cpu.v[2]);
        assert_eq!(1, cpu.v[0xF]);
        assert_eq!(0x202, cpu.pc);
    }

    #[test]
    fn sub_overflow() {
        let mut cpu = Cpu::new();
        cpu.v[1] = 5;
        cpu.v[2] = 6;
        cpu.compute_op(Op::Sub(1, 2));
        assert_eq!(6, cpu.v[2]);
        assert_eq!(0, cpu.v[0xF]);
        assert_eq!(0x202, cpu.pc);
    }

    #[test]
    fn shr() {
        let mut cpu = Cpu::new();
        cpu.v[6] = 0b110;
        cpu.compute_op(Op::Shr(6));
        assert_eq!(0b011, cpu.v[6]);
        assert_eq!(0, cpu.v[0xF]);
        assert_eq!(0x202, cpu.pc);
    }

    #[test]
    fn shr_carry() {
        let mut cpu = Cpu::new();
        cpu.v[6] = 0b101;
        cpu.compute_op(Op::Shr(6));
        assert_eq!(0b010, cpu.v[6]);
        assert_eq!(1, cpu.v[0xF]);
        assert_eq!(0x202, cpu.pc);
    }

    #[test]
    fn subn() {
        let mut cpu = Cpu::new();
        cpu.v[1] = 5;
        cpu.v[2] = 6;
        cpu.compute_op(Op::Subn(1, 2));
        assert_eq!(1, cpu.v[2]);
        assert_eq!(1, cpu.v[0xF]);
        assert_eq!(0x202, cpu.pc);
    }

    #[test]
    fn subn_overflow() {
        let mut cpu = Cpu::new();
        cpu.v[1] = 5;
        cpu.v[2] = 6;
        cpu.compute_op(Op::Subn(2, 1));
        assert_eq!(6, cpu.v[2]);
        assert_eq!(0, cpu.v[0xF]);
        assert_eq!(0x202, cpu.pc);
    }

    #[test]
    fn shl() {
        let mut cpu = Cpu::new();
        cpu.v[1] = 0b01111111;
        cpu.compute_op(Op::Shl(1));
        assert_eq!(0b11111110, cpu.v[1]);
        assert_eq!(0, cpu.v[0xF]);
    }

    #[test]
    fn shl_carry() {
        let mut cpu = Cpu::new();
        cpu.v[1] = 0b11111111;
        cpu.compute_op(Op::Shl(1));
        assert_eq!(0b11111110, cpu.v[1]);
        assert_eq!(1, cpu.v[0xF]);
    }

    #[test]
    fn ldi() {
        let mut cpu = Cpu::new();
        cpu.compute_op(Op::LdI(123));
        assert_eq!(123, cpu.i);
    }

    #[test]
    fn jp_reg_i() {
        let mut cpu = Cpu::new();
        cpu.v[0] = 4;
        cpu.compute_op(Op::JpRegI(5));
        assert_eq!(9, cpu.pc);
    }
}
