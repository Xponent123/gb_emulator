use crate::instructions;
use crate::mbc;
use crate::mmu::MMU;
use crate::register::CpuFlag::{C, H, N, Z};
use crate::register::Registers;
use crate::StrResult;

pub struct CPU {
    pub(crate) reg: Registers,
    pub mmu: MMU,
    pub(crate) halted: bool,
    pub(crate) halt_bug: bool,
    pub(crate) ime: bool,
    pub(crate) setdi: u32,
    pub(crate) setei: u32,
}

impl CPU {
    pub fn new(cart: Box<dyn mbc::MBC + 'static>) -> StrResult<CPU> {
        let cpu_mmu = MMU::new(cart)?;
        let registers = Registers::new(cpu_mmu.gbmode);
        Ok(CPU {
            reg: registers,
            halted: false,
            halt_bug: false,
            ime: true,
            setdi: 0,
            setei: 0,
            mmu: cpu_mmu,
        })
    }

    pub fn new_cgb(cart: Box<dyn mbc::MBC + 'static>) -> StrResult<CPU> {
        let cpu_mmu = MMU::new_cgb(cart)?;
        let registers = Registers::new(cpu_mmu.gbmode);
        Ok(CPU {
            reg: registers,
            halted: false,
            halt_bug: false,
            ime: true,
            setdi: 0,
            setei: 0,
            mmu: cpu_mmu,
        })
    }

    pub fn do_cycle(&mut self) -> u32 {
        let ticks = self.docycle() * 4;
        return self.mmu.do_cycle(ticks);
    }

    fn docycle(&mut self) -> u32 {
        self.updateime();
        match self.handleinterrupt() {
            0 => {}
            n => return n,
        };

        if self.halted {
            // Emulate a noop instruction
            1
        } else {
            self.call()
        }
    }

    pub(crate) fn fetchbyte(&mut self) -> u8 {
        let b = self.mmu.rb(self.reg.pc);
        if self.halt_bug {
            self.halt_bug = false;
        } else {
            self.reg.pc = self.reg.pc.wrapping_add(1);
        }
        // Return the fetched byte from memory at the current PC
        b 
    }

    pub(crate) fn fetchword(&mut self) -> u16 {
        let w = self.mmu.rw(self.reg.pc);
        self.reg.pc += 2;
        w
    }

    fn updateime(&mut self) {
        self.setdi = match self.setdi {
            2 => 1,
            1 => {
                self.ime = false;
                0
            }
            _ => 0,
        };
        self.setei = match self.setei {
            2 => 1,
            1 => {
                self.ime = true;
                0
            }
            _ => 0,
        };
    }

    fn handleinterrupt(&mut self) -> u32 {
        if self.ime == false && self.halted == false {
            return 0;
        }

        let triggered = self.mmu.inte & self.mmu.intf & 0x1F;
        if triggered == 0 {
            return 0;
        }

        self.halted = false;
        if self.ime == false {
            return 0;
        }
        self.ime = false;

        let n = triggered.trailing_zeros();
        if n >= 5 {
            panic!("Invalid interrupt triggered");
        }
        self.mmu.intf &= !(1 << n);
        let pc = self.reg.pc;
        self.pushstack(pc);
        self.reg.pc = 0x0040 | ((n as u16) << 3);

        4
    }

    pub(crate) fn pushstack(&mut self, value: u16) {
        self.reg.sp = self.reg.sp.wrapping_sub(2);
        self.mmu.ww(self.reg.sp, value);
    }

    pub(crate) fn popstack(&mut self) -> u16 {
        let res = self.mmu.rw(self.reg.sp);
        self.reg.sp += 2;
        res
    }

    pub(crate) fn alu_add(&mut self, b: u8, usec: bool) {
        let c = if usec && self.reg.getflag(C) { 1 } else { 0 };
        let a = self.reg.a;
        let r = a.wrapping_add(b).wrapping_add(c);
        self.reg.flag(Z, r == 0);
        self.reg.flag(H, (a & 0xF) + (b & 0xF) + c > 0xF);
        self.reg.flag(N, false);
        self.reg
            .flag(C, (a as u16) + (b as u16) + (c as u16) > 0xFF);
        self.reg.a = r;
    }

    pub(crate) fn alu_sub(&mut self, b: u8, usec: bool) {
        let c = if usec && self.reg.getflag(C) { 1 } else { 0 };
        let a = self.reg.a;
        let r = a.wrapping_sub(b).wrapping_sub(c);
        self.reg.flag(Z, r == 0);
        self.reg.flag(H, (a & 0x0F) < (b & 0x0F) + c);
        self.reg.flag(N, true);
        self.reg.flag(C, (a as u16) < (b as u16) + (c as u16));
        self.reg.a = r;
    }

    pub(crate) fn alu_and(&mut self, b: u8) {
        let r = self.reg.a & b;
        self.reg.flag(Z, r == 0);
        self.reg.flag(H, true);
        self.reg.flag(C, false);
        self.reg.flag(N, false);
        self.reg.a = r;
    }

    pub(crate) fn alu_or(&mut self, b: u8) {
        let r = self.reg.a | b;
        self.reg.flag(Z, r == 0);
        self.reg.flag(C, false);
        self.reg.flag(H, false);
        self.reg.flag(N, false);
        self.reg.a = r;
    }

    pub(crate) fn alu_xor(&mut self, b: u8) {
        let r = self.reg.a ^ b;
        self.reg.flag(Z, r == 0);
        self.reg.flag(C, false);
        self.reg.flag(H, false);
        self.reg.flag(N, false);
        self.reg.a = r;
    }

    pub(crate) fn alu_cp(&mut self, b: u8) {
        let r = self.reg.a;
        self.alu_sub(b, false);
        self.reg.a = r;
    }

    pub(crate) fn alu_inc(&mut self, a: u8) -> u8 {
        let r = a.wrapping_add(1);
        self.reg.flag(Z, r == 0);
        self.reg.flag(H, (a & 0x0F) + 1 > 0x0F);
        self.reg.flag(N, false);
        return r;
    }

    pub(crate) fn alu_dec(&mut self, a: u8) -> u8 {
        let r = a.wrapping_sub(1);
        self.reg.flag(Z, r == 0);
        self.reg.flag(H, (a & 0x0F) == 0);
        self.reg.flag(N, true);
        return r;
    }

    pub(crate) fn alu_add16(&mut self, b: u16) {
        let a = self.reg.hl();
        let r = a.wrapping_add(b);
        self.reg.flag(H, (a & 0x0FFF) + (b & 0x0FFF) > 0x0FFF);
        self.reg.flag(N, false);
        self.reg.flag(C, a > 0xFFFF - b);
        self.reg.sethl(r);
    }

    pub(crate) fn alu_add16imm(&mut self, a: u16) -> u16 {
        let b = self.fetchbyte() as i8 as i16 as u16;
        self.reg.flag(N, false);
        self.reg.flag(Z, false);
        self.reg.flag(H, (a & 0x000F) + (b & 0x000F) > 0x000F);
        self.reg.flag(C, (a & 0x00FF) + (b & 0x00FF) > 0x00FF);
        return a.wrapping_add(b);
    }

    pub(crate) fn alu_swap(&mut self, a: u8) -> u8 {
        self.reg.flag(Z, a == 0);
        self.reg.flag(C, false);
        self.reg.flag(H, false);
        self.reg.flag(N, false);
        (a >> 4) | (a << 4)
    }

    pub(crate) fn alu_srflagupdate(&mut self, r: u8, c: bool) {
        self.reg.flag(H, false);
        self.reg.flag(N, false);
        self.reg.flag(Z, r == 0);
        self.reg.flag(C, c);
    }

    pub(crate) fn alu_rlc(&mut self, a: u8) -> u8 {
        let c = a & 0x80 == 0x80;
        let r = (a << 1) | (if c { 1 } else { 0 });
        self.alu_srflagupdate(r, c);
        return r;
    }

    pub(crate) fn alu_rl(&mut self, a: u8) -> u8 {
        let c = a & 0x80 == 0x80;
        let r = (a << 1) | (if self.reg.getflag(C) { 1 } else { 0 });
        self.alu_srflagupdate(r, c);
        return r;
    }

    pub(crate) fn alu_rrc(&mut self, a: u8) -> u8 {
        let c = a & 0x01 == 0x01;
        let r = (a >> 1) | (if c { 0x80 } else { 0 });
        self.alu_srflagupdate(r, c);
        return r;
    }

    pub(crate) fn alu_rr(&mut self, a: u8) -> u8 {
        let c = a & 0x01 == 0x01;
        let r = (a >> 1) | (if self.reg.getflag(C) { 0x80 } else { 0 });
        self.alu_srflagupdate(r, c);
        return r;
    }

    pub(crate) fn alu_sla(&mut self, a: u8) -> u8 {
        let c = a & 0x80 == 0x80;
        let r = a << 1;
        self.alu_srflagupdate(r, c);
        return r;
    }

    pub(crate) fn alu_sra(&mut self, a: u8) -> u8 {
        let c = a & 0x01 == 0x01;
        let r = (a >> 1) | (a & 0x80);
        self.alu_srflagupdate(r, c);
        return r;
    }

    pub(crate) fn alu_srl(&mut self, a: u8) -> u8 {
        let c = a & 0x01 == 0x01;
        let r = a >> 1;
        self.alu_srflagupdate(r, c);
        return r;
    }

    pub(crate) fn alu_bit(&mut self, a: u8, b: u8) {
        let r = a & (1 << (b as u32)) == 0;
        self.reg.flag(N, false);
        self.reg.flag(H, true);
        self.reg.flag(Z, r);
    }

    pub(crate) fn alu_daa(&mut self) {
        let mut a = self.reg.a;
        let mut adjust = if self.reg.getflag(C) { 0x60 } else { 0x00 };
        if self.reg.getflag(H) {
            adjust |= 0x06;
        };
        if !self.reg.getflag(N) {
            if a & 0x0F > 0x09 {
                adjust |= 0x06;
            };
            if a > 0x99 {
                adjust |= 0x60;
            };
            a = a.wrapping_add(adjust);
        } else {
            a = a.wrapping_sub(adjust);
        }

        self.reg.flag(C, adjust >= 0x60);
        self.reg.flag(H, false);
        self.reg.flag(Z, a == 0);
        self.reg.a = a;
    }

    pub(crate) fn cpu_jr(&mut self) {
        let n = self.fetchbyte() as i8;
        self.reg.pc = ((self.reg.pc as u32 as i32) + (n as i32)) as u16;
    }

    // replace the old big match in CPU::call()
    pub(crate) fn call(&mut self) -> u32 {
        instructions::call(self)
    }

    // replace the old big match in CPU::call_cb()
    pub(crate) fn call_cb(&mut self) -> u32 {
        instructions::call_cb(self)
    }
}
