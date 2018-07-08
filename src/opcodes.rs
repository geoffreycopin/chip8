#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Op {
    Cls,
    Ret,
    Jp(u16),
    Call(u16),
    Se(u8, u8),
    Sne(u8, u8),
    SeReg(u8, u8),
    Ld(u8, u8),
    Add(u8, u8),
    LdReg(u8, u8),
    Or(u8, u8),
    And(u8, u8),
    Xor(u8, u8),
    AddReg(u8, u8),
    Sub(u8, u8),
    Shr(u8),
    Subn(u8, u8),
    Shl(u8),
    SneReg(u8, u8),
    LdI(u16),
    JpRegI(u16),
    Rnd(u8, u8),
    Drw(u8, u8, u8),
    Skp(u8),
    Sknp(u8),
    LdDT(u8),
    LdKb(u8),
    SetDT(u8),
    SetST(u8),
    AddToI(u8),
    LdChr(u8),
    LdBCD(u8),
    LdRegs(u8),
    RdMem(u8),
}

impl From<u16> for Op {
    fn from(op: u16) -> Self {
        decode(op)
    }
}

pub fn decode(opcode: u16) -> Op {
    let components = opcode_components(opcode);
    let addr = opcode & 0x0FFF;
    let byte = (opcode & 0x00FF) as u8;

    match components {
        (0x0, 0x0, 0xE, 0x0) => Op::Cls,
        (0x0, 0x0, 0xE, 0xE) => Op::Ret,
        (0x1, _, _, _) => Op::Jp(addr),
        (0x2, _, _, _) => Op::Call(addr),
        (0x3, x, _, _) => Op::Se(x, byte),
        (0x4, x, _, _) => Op::Sne(x, byte),
        (0x5, x, y, 0x0) => Op::SeReg(x, y),
        (0x6, x, _, _) => Op::Ld(x, byte),
        (0x7, x, _, _) => Op::Add(x, byte),
        (0x8, x, y, 0x0) => Op::LdReg(x, y),
        (0x8, x, y, 0x1) => Op::Or(x, y),
        (0x8, x, y, 0x2) => Op::And(x, y),
        (0x8, x, y, 0x3) => Op::Xor(x, y),
        (0x8, x, y, 0x4) => Op::AddReg(x, y),
        (0x8, x, y, 0x5) => Op::Sub(x, y),
        (0x8, x, _, 0x6) => Op::Shr(x),
        (0x8, x, y, 0x7) => Op::Subn(x, y),
        (0x8, x, _, 0xE) => Op::Shl(x),
        (0x9, x, y, 0x0) => Op::SneReg(x, y),
        (0xA, _, _, _) => Op::LdI(addr),
        (0xB, _, _, _) => Op::JpRegI(addr),
        (0xC, x, _, _) => Op::Rnd(x, byte),
        (0xD, x, y, n) => Op::Drw(x, y, n),
        (0xE, x, 0x9, 0xE) => Op::Skp(x),
        (0xE, x, 0xA, 0x1) => Op::Sknp(x),
        (0xF, x, 0x0, 0x7) => Op::LdDT(x),
        (0xF, x, 0x0, 0xA) => Op::LdKb(x),
        (0xF, x, 0x1, 0x5) => Op::SetDT(x),
        (0xF, x, 0x1, 0x8) => Op::SetST(x),
        (0xF, x, 0x1, 0xE) => Op::AddToI(x),
        (0xF, x, 0x2, 0x9) => Op::LdChr(x),
        (0xF, x, 0x3, 0x3) => Op::LdBCD(x),
        (0xF, x, 0x5, 0x5) => Op::LdRegs(x),
        (0xF, x, 0x6, 0x5) => Op::RdMem(x),
        _ => panic!("Invalid opcode: {:X?}", opcode),
    }
}

fn opcode_components(opcode: u16) -> (u8, u8, u8, u8) {
    (
        ((opcode & 0xF000) >> 12) as u8,
        ((opcode & 0x0F00) >> 8) as u8,
        ((opcode & 0x00F0) >> 4) as u8,
        (opcode & 0x000F) as u8,
    )
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn cls() {
        assert_eq!(Op::Cls, decode(0x00E0))
    }

    #[test]
    fn ret() {
        assert_eq!(Op::Ret, decode(0x00EE))
    }

    #[test]
    fn jp() {
        assert_eq!(Op::Jp(0x0234), decode(0x1234))
    }

    #[test]
    fn call() {
        assert_eq!(Op::Call(0x0345), decode(0x2345))
    }

    #[test]
    fn se() {
        assert_eq!(Op::Se(0x04, 0x56), decode(0x3456))
    }

    #[test]
    fn sne() {
        assert_eq!(Op::Sne(0x05, 0x67), decode(0x4567))
    }

    #[test]
    fn se_reg() {
        assert_eq!(Op::SeReg(0x06, 0x07), decode(0x5670))
    }

    #[test]
    fn ld() {
        assert_eq!(Op::Ld(0x07, 0x89), decode(0x6789))
    }

    #[test]
    fn add() {
        assert_eq!(Op::Add(0x08, 0x90), decode(0x7890))
    }

    #[test]
    fn ld_reg() {
        assert_eq!(Op::LdReg(0x09, 0x01), decode(0x8910))
    }

    #[test]
    fn or() {
        assert_eq!(Op::Or(0x09, 0x01), decode(0x8911))
    }

    #[test]
    fn and() {
        assert_eq!(Op::And(0x09, 0x07), decode(0x8972))
    }

    #[test]
    fn xor() {
        assert_eq!(Op::Xor(0x09, 0x07), decode(0x8973))
    }

    #[test]
    fn add_reg() {
        assert_eq!(Op::AddReg(0x09, 0x07), decode(0x8974))
    }

    #[test]
    fn sub() {
        assert_eq!(Op::Sub(0x09, 0x07), decode(0x8975))
    }

    #[test]
    fn shr() {
        assert_eq!(Op::Shr(0x09), decode(0x8906))
    }

    #[test]
    fn subn() {
        assert_eq!(Op::Subn(0x09, 0x07), decode(0x8977))
    }

    #[test]
    fn shl() {
        assert_eq!(Op::Shl(0x09), decode(0x897E))
    }

    #[test]
    fn sne_reg() {
        assert_eq!(Op::SneReg(0x01, 0x02), decode(0x9120))
    }

    #[test]
    fn ldi() {
        assert_eq!(Op::LdI(0x0123), decode(0xA123))
    }

    #[test]
    fn jp_reg_i() {
        assert_eq!(Op::JpRegI(0x0123), decode(0xB123))
    }

    #[test]
    fn rand() {
        assert_eq!(Op::Rnd(0x0F, 0x12), decode(0xCF12))
    }

    #[test]
    fn drw() {
        assert_eq!(Op::Drw(0x05, 0x06, 0x07), decode(0xD567))
    }

    #[test]
    fn skp() {
        assert_eq!(Op::Skp(0x3), decode(0xE39E))
    }

    #[test]
    fn sknp() {
        assert_eq!(Op::Sknp(0x03), decode(0xE3A1))
    }

    #[test]
    fn ld_dt() {
        assert_eq!(Op::LdDT(0x09), decode(0xF907))
    }

    #[test]
    fn ld_kb() {
        assert_eq!(Op::LdKb(0x09), decode(0xF90A))
    }

    #[test]
    fn set_dt() {
        assert_eq!(Op::SetDT(0x09), decode(0xF915))
    }

    #[test]
    fn set_st() {
        assert_eq!(Op::SetST(0x09), decode(0xF918))
    }

    #[test]
    fn add_to_i() {
        assert_eq!(Op::AddToI(0x09), decode(0xF91E))
    }

    #[test]
    fn ld_chr() {
        assert_eq!(Op::LdChr(0x09), decode(0xF929))
    }

    #[test]
    fn ld_bcd() {
        assert_eq!(Op::LdBCD(0x09), decode(0xF933))
    }

    #[test]
    fn ld_regs() {
        assert_eq!(Op::LdRegs(0x09), decode(0xF955))
    }

    #[test]
    fn rd_mem() {
        assert_eq!(Op::RdMem(0x09), decode(0xF965))
    }
}
