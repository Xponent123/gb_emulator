use crate::cpu::CPU;
use crate::register::CpuFlag::{C, H, N, Z};

pub fn call(cpu: &mut CPU) -> u32 {
    let opcode = cpu.fetchbyte();
    match opcode {
        0x00 => 1,
        0x01 => {
            let v = cpu.fetchword();
            cpu.reg.setbc(v);
            3
        }
        0x02 => {
            cpu.mmu.wb(cpu.reg.bc(), cpu.reg.a);
            2
        }
        0x03 => {
            cpu.reg.setbc(cpu.reg.bc().wrapping_add(1));
            2
        }
        0x04 => {
            cpu.reg.b = cpu.alu_inc(cpu.reg.b);
            1
        }
        0x05 => {
            cpu.reg.b = cpu.alu_dec(cpu.reg.b);
            1
        }
        0x06 => {
            cpu.reg.b = cpu.fetchbyte();
            2
        }
        0x07 => {
            cpu.reg.a = cpu.alu_rlc(cpu.reg.a);
            cpu.reg.flag(Z, false);
            1
        }
        0x08 => {
            let a = cpu.fetchword();
            cpu.mmu.ww(a, cpu.reg.sp);
            5
        }
        0x09 => {
            cpu.alu_add16(cpu.reg.bc());
            2
        }
        0x0A => {
            cpu.reg.a = cpu.mmu.rb(cpu.reg.bc());
            2
        }
        0x0B => {
            cpu.reg.setbc(cpu.reg.bc().wrapping_sub(1));
            2
        }
        0x0C => {
            cpu.reg.c = cpu.alu_inc(cpu.reg.c);
            1
        }
        0x0D => {
            cpu.reg.c = cpu.alu_dec(cpu.reg.c);
            1
        }
        0x0E => {
            cpu.reg.c = cpu.fetchbyte();
            2
        }
        0x0F => {
            cpu.reg.a = cpu.alu_rrc(cpu.reg.a);
            cpu.reg.flag(Z, false);
            1
        }
        0x10 => {
            cpu.mmu.switch_speed();
            1
        } // STOP
        0x11 => {
            let v = cpu.fetchword();
            cpu.reg.setde(v);
            3
        }
        0x12 => {
            cpu.mmu.wb(cpu.reg.de(), cpu.reg.a);
            2
        }
        0x13 => {
            cpu.reg.setde(cpu.reg.de().wrapping_add(1));
            2
        }
        0x14 => {
            cpu.reg.d = cpu.alu_inc(cpu.reg.d);
            1
        }
        0x15 => {
            cpu.reg.d = cpu.alu_dec(cpu.reg.d);
            1
        }
        0x16 => {
            cpu.reg.d = cpu.fetchbyte();
            2
        }
        0x17 => {
            cpu.reg.a = cpu.alu_rl(cpu.reg.a);
            cpu.reg.flag(Z, false);
            1
        }
        0x18 => {
            cpu.cpu_jr();
            3
        }
        0x19 => {
            cpu.alu_add16(cpu.reg.de());
            2
        }
        0x1A => {
            cpu.reg.a = cpu.mmu.rb(cpu.reg.de());
            2
        }
        0x1B => {
            cpu.reg.setde(cpu.reg.de().wrapping_sub(1));
            2
        }
        0x1C => {
            cpu.reg.e = cpu.alu_inc(cpu.reg.e);
            1
        }
        0x1D => {
            cpu.reg.e = cpu.alu_dec(cpu.reg.e);
            1
        }
        0x1E => {
            cpu.reg.e = cpu.fetchbyte();
            2
        }
        0x1F => {
            cpu.reg.a = cpu.alu_rr(cpu.reg.a);
            cpu.reg.flag(Z, false);
            1
        }
        0x20 => {
            if !cpu.reg.getflag(Z) {
                cpu.cpu_jr();
                3
            } else {
                cpu.reg.pc += 1;
                2
            }
        }
        0x21 => {
            let v = cpu.fetchword();
            cpu.reg.sethl(v);
            3
        }
        0x22 => {
            cpu.mmu.wb(cpu.reg.hli(), cpu.reg.a);
            2
        }
        0x23 => {
            let v = cpu.reg.hl().wrapping_add(1);
            cpu.reg.sethl(v);
            2
        }
        0x24 => {
            cpu.reg.h = cpu.alu_inc(cpu.reg.h);
            1
        }
        0x25 => {
            cpu.reg.h = cpu.alu_dec(cpu.reg.h);
            1
        }
        0x26 => {
            cpu.reg.h = cpu.fetchbyte();
            2
        }
        0x27 => {
            cpu.alu_daa();
            1
        }
        0x28 => {
            if cpu.reg.getflag(Z) {
                cpu.cpu_jr();
                3
            } else {
                cpu.reg.pc += 1;
                2
            }
        }
        0x29 => {
            let v = cpu.reg.hl();
            cpu.alu_add16(v);
            2
        }
        0x2A => {
            cpu.reg.a = cpu.mmu.rb(cpu.reg.hli());
            2
        }
        0x2B => {
            let v = cpu.reg.hl().wrapping_sub(1);
            cpu.reg.sethl(v);
            2
        }
        0x2C => {
            cpu.reg.l = cpu.alu_inc(cpu.reg.l);
            1
        }
        0x2D => {
            cpu.reg.l = cpu.alu_dec(cpu.reg.l);
            1
        }
        0x2E => {
            cpu.reg.l = cpu.fetchbyte();
            2
        }
        0x2F => {
            cpu.reg.a = !cpu.reg.a;
            cpu.reg.flag(H, true);
            cpu.reg.flag(N, true);
            1
        }
        0x30 => {
            if !cpu.reg.getflag(C) {
                cpu.cpu_jr();
                3
            } else {
                cpu.reg.pc += 1;
                2
            }
        }
        0x31 => {
            cpu.reg.sp = cpu.fetchword();
            3
        }
        0x32 => {
            cpu.mmu.wb(cpu.reg.hld(), cpu.reg.a);
            2
        }
        0x33 => {
            cpu.reg.sp = cpu.reg.sp.wrapping_add(1);
            2
        }
        0x34 => {
            let a = cpu.reg.hl();
            let v = cpu.mmu.rb(a);
            let v2 = cpu.alu_inc(v);
            cpu.mmu.wb(a, v2);
            3
        }
        0x35 => {
            let a = cpu.reg.hl();
            let v = cpu.mmu.rb(a);
            let v2 = cpu.alu_dec(v);
            cpu.mmu.wb(a, v2);
            3
        }
        0x36 => {
            let v = cpu.fetchbyte();
            cpu.mmu.wb(cpu.reg.hl(), v);
            3
        }
        0x37 => {
            cpu.reg.flag(C, true);
            cpu.reg.flag(H, false);
            cpu.reg.flag(N, false);
            1
        }
        0x38 => {
            if cpu.reg.getflag(C) {
                cpu.cpu_jr();
                3
            } else {
                cpu.reg.pc += 1;
                2
            }
        }
        0x39 => {
            cpu.alu_add16(cpu.reg.sp);
            2
        }
        0x3A => {
            cpu.reg.a = cpu.mmu.rb(cpu.reg.hld());
            2
        }
        0x3B => {
            cpu.reg.sp = cpu.reg.sp.wrapping_sub(1);
            2
        }
        0x3C => {
            cpu.reg.a = cpu.alu_inc(cpu.reg.a);
            1
        }
        0x3D => {
            cpu.reg.a = cpu.alu_dec(cpu.reg.a);
            1
        }
        0x3E => {
            cpu.reg.a = cpu.fetchbyte();
            2
        }
        0x3F => {
            let v = !cpu.reg.getflag(C);
            cpu.reg.flag(C, v);
            cpu.reg.flag(H, false);
            cpu.reg.flag(N, false);
            1
        }
        0x40 => 1,
        0x41 => {
            cpu.reg.b = cpu.reg.c;
            1
        }
        0x42 => {
            cpu.reg.b = cpu.reg.d;
            1
        }
        0x43 => {
            cpu.reg.b = cpu.reg.e;
            1
        }
        0x44 => {
            cpu.reg.b = cpu.reg.h;
            1
        }
        0x45 => {
            cpu.reg.b = cpu.reg.l;
            1
        }
        0x46 => {
            cpu.reg.b = cpu.mmu.rb(cpu.reg.hl());
            2
        }
        0x47 => {
            cpu.reg.b = cpu.reg.a;
            1
        }
        0x48 => {
            cpu.reg.c = cpu.reg.b;
            1
        }
        0x49 => 1,
        0x4A => {
            cpu.reg.c = cpu.reg.d;
            1
        }
        0x4B => {
            cpu.reg.c = cpu.reg.e;
            1
        }
        0x4C => {
            cpu.reg.c = cpu.reg.h;
            1
        }
        0x4D => {
            cpu.reg.c = cpu.reg.l;
            1
        }
        0x4E => {
            cpu.reg.c = cpu.mmu.rb(cpu.reg.hl());
            2
        }
        0x4F => {
            cpu.reg.c = cpu.reg.a;
            1
        }
        0x50 => {
            cpu.reg.d = cpu.reg.b;
            1
        }
        0x51 => {
            cpu.reg.d = cpu.reg.c;
            1
        }
        0x52 => 1,
        0x53 => {
            cpu.reg.d = cpu.reg.e;
            1
        }
        0x54 => {
            cpu.reg.d = cpu.reg.h;
            1
        }
        0x55 => {
            cpu.reg.d = cpu.reg.l;
            1
        }
        0x56 => {
            cpu.reg.d = cpu.mmu.rb(cpu.reg.hl());
            2
        }
        0x57 => {
            cpu.reg.d = cpu.reg.a;
            1
        }
        0x58 => {
            cpu.reg.e = cpu.reg.b;
            1
        }
        0x59 => {
            cpu.reg.e = cpu.reg.c;
            1
        }
        0x5A => {
            cpu.reg.e = cpu.reg.d;
            1
        }
        0x5B => 1,
        0x5C => {
            cpu.reg.e = cpu.reg.h;
            1
        }
        0x5D => {
            cpu.reg.e = cpu.reg.l;
            1
        }
        0x5E => {
            cpu.reg.e = cpu.mmu.rb(cpu.reg.hl());
            2
        }
        0x5F => {
            cpu.reg.e = cpu.reg.a;
            1
        }
        0x60 => {
            cpu.reg.h = cpu.reg.b;
            1
        }
        0x61 => {
            cpu.reg.h = cpu.reg.c;
            1
        }
        0x62 => {
            cpu.reg.h = cpu.reg.d;
            1
        }
        0x63 => {
            cpu.reg.h = cpu.reg.e;
            1
        }
        0x64 => 1,
        0x65 => {
            cpu.reg.h = cpu.reg.l;
            1
        }
        0x66 => {
            cpu.reg.h = cpu.mmu.rb(cpu.reg.hl());
            2
        }
        0x67 => {
            cpu.reg.h = cpu.reg.a;
            1
        }
        0x68 => {
            cpu.reg.l = cpu.reg.b;
            1
        }
        0x69 => {
            cpu.reg.l = cpu.reg.c;
            1
        }
        0x6A => {
            cpu.reg.l = cpu.reg.d;
            1
        }
        0x6B => {
            cpu.reg.l = cpu.reg.e;
            1
        }
        0x6C => {
            cpu.reg.l = cpu.reg.h;
            1
        }
        0x6D => 1,
        0x6E => {
            cpu.reg.l = cpu.mmu.rb(cpu.reg.hl());
            2
        }
        0x6F => {
            cpu.reg.l = cpu.reg.a;
            1
        }
        0x70 => {
            cpu.mmu.wb(cpu.reg.hl(), cpu.reg.b);
            2
        }
        0x71 => {
            cpu.mmu.wb(cpu.reg.hl(), cpu.reg.c);
            2
        }
        0x72 => {
            cpu.mmu.wb(cpu.reg.hl(), cpu.reg.d);
            2
        }
        0x73 => {
            cpu.mmu.wb(cpu.reg.hl(), cpu.reg.e);
            2
        }
        0x74 => {
            cpu.mmu.wb(cpu.reg.hl(), cpu.reg.h);
            2
        }
        0x75 => {
            cpu.mmu.wb(cpu.reg.hl(), cpu.reg.l);
            2
        }
        0x76 => {
            cpu.halted = true;
            cpu.halt_bug = cpu.mmu.intf & cpu.mmu.inte & 0x1F != 0;
            1
        }
        0x77 => {
            cpu.mmu.wb(cpu.reg.hl(), cpu.reg.a);
            2
        }
        0x78 => {
            cpu.reg.a = cpu.reg.b;
            1
        }
        0x79 => {
            cpu.reg.a = cpu.reg.c;
            1
        }
        0x7A => {
            cpu.reg.a = cpu.reg.d;
            1
        }
        0x7B => {
            cpu.reg.a = cpu.reg.e;
            1
        }
        0x7C => {
            cpu.reg.a = cpu.reg.h;
            1
        }
        0x7D => {
            cpu.reg.a = cpu.reg.l;
            1
        }
        0x7E => {
            cpu.reg.a = cpu.mmu.rb(cpu.reg.hl());
            2
        }
        0x7F => 1,
        0x80 => {
            cpu.alu_add(cpu.reg.b, false);
            1
        }
        0x81 => {
            cpu.alu_add(cpu.reg.c, false);
            1
        }
        0x82 => {
            cpu.alu_add(cpu.reg.d, false);
            1
        }
        0x83 => {
            cpu.alu_add(cpu.reg.e, false);
            1
        }
        0x84 => {
            cpu.alu_add(cpu.reg.h, false);
            1
        }
        0x85 => {
            cpu.alu_add(cpu.reg.l, false);
            1
        }
        0x86 => {
            let v = cpu.mmu.rb(cpu.reg.hl());
            cpu.alu_add(v, false);
            2
        }
        0x87 => {
            cpu.alu_add(cpu.reg.a, false);
            1
        }
        0x88 => {
            cpu.alu_add(cpu.reg.b, true);
            1
        }
        0x89 => {
            cpu.alu_add(cpu.reg.c, true);
            1
        }
        0x8A => {
            cpu.alu_add(cpu.reg.d, true);
            1
        }
        0x8B => {
            cpu.alu_add(cpu.reg.e, true);
            1
        }
        0x8C => {
            cpu.alu_add(cpu.reg.h, true);
            1
        }
        0x8D => {
            cpu.alu_add(cpu.reg.l, true);
            1
        }
        0x8E => {
            let v = cpu.mmu.rb(cpu.reg.hl());
            cpu.alu_add(v, true);
            2
        }
        0x8F => {
            cpu.alu_add(cpu.reg.a, true);
            1
        }
        0x90 => {
            cpu.alu_sub(cpu.reg.b, false);
            1
        }
        0x91 => {
            cpu.alu_sub(cpu.reg.c, false);
            1
        }
        0x92 => {
            cpu.alu_sub(cpu.reg.d, false);
            1
        }
        0x93 => {
            cpu.alu_sub(cpu.reg.e, false);
            1
        }
        0x94 => {
            cpu.alu_sub(cpu.reg.h, false);
            1
        }
        0x95 => {
            cpu.alu_sub(cpu.reg.l, false);
            1
        }
        0x96 => {
            let v = cpu.mmu.rb(cpu.reg.hl());
            cpu.alu_sub(v, false);
            2
        }
        0x97 => {
            cpu.alu_sub(cpu.reg.a, false);
            1
        }
        0x98 => {
            cpu.alu_sub(cpu.reg.b, true);
            1
        }
        0x99 => {
            cpu.alu_sub(cpu.reg.c, true);
            1
        }
        0x9A => {
            cpu.alu_sub(cpu.reg.d, true);
            1
        }
        0x9B => {
            cpu.alu_sub(cpu.reg.e, true);
            1
        }
        0x9C => {
            cpu.alu_sub(cpu.reg.h, true);
            1
        }
        0x9D => {
            cpu.alu_sub(cpu.reg.l, true);
            1
        }
        0x9E => {
            let v = cpu.mmu.rb(cpu.reg.hl());
            cpu.alu_sub(v, true);
            2
        }
        0x9F => {
            cpu.alu_sub(cpu.reg.a, true);
            1
        }
        0xA0 => {
            cpu.alu_and(cpu.reg.b);
            1
        }
        0xA1 => {
            cpu.alu_and(cpu.reg.c);
            1
        }
        0xA2 => {
            cpu.alu_and(cpu.reg.d);
            1
        }
        0xA3 => {
            cpu.alu_and(cpu.reg.e);
            1
        }
        0xA4 => {
            cpu.alu_and(cpu.reg.h);
            1
        }
        0xA5 => {
            cpu.alu_and(cpu.reg.l);
            1
        }
        0xA6 => {
            let v = cpu.mmu.rb(cpu.reg.hl());
            cpu.alu_and(v);
            2
        }
        0xA7 => {
            cpu.alu_and(cpu.reg.a);
            1
        }
        0xA8 => {
            cpu.alu_xor(cpu.reg.b);
            1
        }
        0xA9 => {
            cpu.alu_xor(cpu.reg.c);
            1
        }
        0xAA => {
            cpu.alu_xor(cpu.reg.d);
            1
        }
        0xAB => {
            cpu.alu_xor(cpu.reg.e);
            1
        }
        0xAC => {
            cpu.alu_xor(cpu.reg.h);
            1
        }
        0xAD => {
            cpu.alu_xor(cpu.reg.l);
            1
        }
        0xAE => {
            let v = cpu.mmu.rb(cpu.reg.hl());
            cpu.alu_xor(v);
            2
        }
        0xAF => {
            cpu.alu_xor(cpu.reg.a);
            1
        }
        0xB0 => {
            cpu.alu_or(cpu.reg.b);
            1
        }
        0xB1 => {
            cpu.alu_or(cpu.reg.c);
            1
        }
        0xB2 => {
            cpu.alu_or(cpu.reg.d);
            1
        }
        0xB3 => {
            cpu.alu_or(cpu.reg.e);
            1
        }
        0xB4 => {
            cpu.alu_or(cpu.reg.h);
            1
        }
        0xB5 => {
            cpu.alu_or(cpu.reg.l);
            1
        }
        0xB6 => {
            let v = cpu.mmu.rb(cpu.reg.hl());
            cpu.alu_or(v);
            2
        }
        0xB7 => {
            cpu.alu_or(cpu.reg.a);
            1
        }
        0xB8 => {
            cpu.alu_cp(cpu.reg.b);
            1
        }
        0xB9 => {
            cpu.alu_cp(cpu.reg.c);
            1
        }
        0xBA => {
            cpu.alu_cp(cpu.reg.d);
            1
        }
        0xBB => {
            cpu.alu_cp(cpu.reg.e);
            1
        }
        0xBC => {
            cpu.alu_cp(cpu.reg.h);
            1
        }
        0xBD => {
            cpu.alu_cp(cpu.reg.l);
            1
        }
        0xBE => {
            let v = cpu.mmu.rb(cpu.reg.hl());
            cpu.alu_cp(v);
            2
        }
        0xBF => {
            cpu.alu_cp(cpu.reg.a);
            1
        }
        0xC0 => {
            if !cpu.reg.getflag(Z) {
                cpu.reg.pc = cpu.popstack();
                5
            } else {
                2
            }
        }
        0xC1 => {
            let v = cpu.popstack();
            cpu.reg.setbc(v);
            3
        }
        0xC2 => {
            if !cpu.reg.getflag(Z) {
                cpu.reg.pc = cpu.fetchword();
                4
            } else {
                cpu.reg.pc += 2;
                3
            }
        }
        0xC3 => {
            cpu.reg.pc = cpu.fetchword();
            4
        }
        0xC4 => {
            if !cpu.reg.getflag(Z) {
                cpu.pushstack(cpu.reg.pc + 2);
                cpu.reg.pc = cpu.fetchword();
                6
            } else {
                cpu.reg.pc += 2;
                3
            }
        }
        0xC5 => {
            cpu.pushstack(cpu.reg.bc());
            4
        }
        0xC6 => {
            let v = cpu.fetchbyte();
            cpu.alu_add(v, false);
            2
        }
        0xC7 => {
            cpu.pushstack(cpu.reg.pc);
            cpu.reg.pc = 0x00;
            4
        }
        0xC8 => {
            if cpu.reg.getflag(Z) {
                cpu.reg.pc = cpu.popstack();
                5
            } else {
                2
            }
        }
        0xC9 => {
            cpu.reg.pc = cpu.popstack();
            4
        }
        0xCA => {
            if cpu.reg.getflag(Z) {
                cpu.reg.pc = cpu.fetchword();
                4
            } else {
                cpu.reg.pc += 2;
                3
            }
        }
        0xCB => cpu.call_cb(),
        0xCC => {
            if cpu.reg.getflag(Z) {
                cpu.pushstack(cpu.reg.pc + 2);
                cpu.reg.pc = cpu.fetchword();
                6
            } else {
                cpu.reg.pc += 2;
                3
            }
        }
        0xCD => {
            cpu.pushstack(cpu.reg.pc + 2);
            cpu.reg.pc = cpu.fetchword();
            6
        }
        0xCE => {
            let v = cpu.fetchbyte();
            cpu.alu_add(v, true);
            2
        }
        0xCF => {
            cpu.pushstack(cpu.reg.pc);
            cpu.reg.pc = 0x08;
            4
        }
        0xD0 => {
            if !cpu.reg.getflag(C) {
                cpu.reg.pc = cpu.popstack();
                5
            } else {
                2
            }
        }
        0xD1 => {
            let v = cpu.popstack();
            cpu.reg.setde(v);
            3
        }
        0xD2 => {
            if !cpu.reg.getflag(C) {
                cpu.reg.pc = cpu.fetchword();
                4
            } else {
                cpu.reg.pc += 2;
                3
            }
        }
        0xD4 => {
            if !cpu.reg.getflag(C) {
                cpu.pushstack(cpu.reg.pc + 2);
                cpu.reg.pc = cpu.fetchword();
                6
            } else {
                cpu.reg.pc += 2;
                3
            }
        }
        0xD5 => {
            cpu.pushstack(cpu.reg.de());
            4
        }
        0xD6 => {
            let v = cpu.fetchbyte();
            cpu.alu_sub(v, false);
            2
        }
        0xD7 => {
            cpu.pushstack(cpu.reg.pc);
            cpu.reg.pc = 0x10;
            4
        }
        0xD8 => {
            if cpu.reg.getflag(C) {
                cpu.reg.pc = cpu.popstack();
                5
            } else {
                2
            }
        }
        0xD9 => {
            cpu.reg.pc = cpu.popstack();
            cpu.setei = 1;
            4
        }
        0xDA => {
            if cpu.reg.getflag(C) {
                cpu.reg.pc = cpu.fetchword();
                4
            } else {
                cpu.reg.pc += 2;
                3
            }
        }
        0xDC => {
            if cpu.reg.getflag(C) {
                cpu.pushstack(cpu.reg.pc + 2);
                cpu.reg.pc = cpu.fetchword();
                6
            } else {
                cpu.reg.pc += 2;
                3
            }
        }
        0xDE => {
            let v = cpu.fetchbyte();
            cpu.alu_sub(v, true);
            2
        }
        0xDF => {
            cpu.pushstack(cpu.reg.pc);
            cpu.reg.pc = 0x18;
            4
        }
        0xE0 => {
            let a = 0xFF00 | cpu.fetchbyte() as u16;
            cpu.mmu.wb(a, cpu.reg.a);
            3
        }
        0xE1 => {
            let v = cpu.popstack();
            cpu.reg.sethl(v);
            3
        }
        0xE2 => {
            cpu.mmu.wb(0xFF00 | cpu.reg.c as u16, cpu.reg.a);
            2
        }
        0xE5 => {
            cpu.pushstack(cpu.reg.hl());
            4
        }
        0xE6 => {
            let v = cpu.fetchbyte();
            cpu.alu_and(v);
            2
        }
        0xE7 => {
            cpu.pushstack(cpu.reg.pc);
            cpu.reg.pc = 0x20;
            4
        }
        0xE8 => {
            cpu.reg.sp = cpu.alu_add16imm(cpu.reg.sp);
            4
        }
        0xE9 => {
            cpu.reg.pc = cpu.reg.hl();
            1
        }
        0xEA => {
            let a = cpu.fetchword();
            cpu.mmu.wb(a, cpu.reg.a);
            4
        }
        0xEE => {
            let v = cpu.fetchbyte();
            cpu.alu_xor(v);
            2
        }
        0xEF => {
            cpu.pushstack(cpu.reg.pc);
            cpu.reg.pc = 0x28;
            4
        }
        0xF0 => {
            let a = 0xFF00 | cpu.fetchbyte() as u16;
            cpu.reg.a = cpu.mmu.rb(a);
            3
        }
        0xF1 => {
            let v = cpu.popstack() & 0xFFF0;
            cpu.reg.setaf(v);
            3
        }
        0xF2 => {
            cpu.reg.a = cpu.mmu.rb(0xFF00 | cpu.reg.c as u16);
            2
        }
        0xF3 => {
            cpu.setdi = 2;
            1
        }
        0xF5 => {
            cpu.pushstack(cpu.reg.af());
            4
        }
        0xF6 => {
            let v = cpu.fetchbyte();
            cpu.alu_or(v);
            2
        }
        0xF7 => {
            cpu.pushstack(cpu.reg.pc);
            cpu.reg.pc = 0x30;
            4
        }
        0xF8 => {
            let r = cpu.alu_add16imm(cpu.reg.sp);
            cpu.reg.sethl(r);
            3
        }
        0xF9 => {
            cpu.reg.sp = cpu.reg.hl();
            2
        }
        0xFA => {
            let a = cpu.fetchword();
            cpu.reg.a = cpu.mmu.rb(a);
            4
        }
        0xFB => {
            cpu.setei = 2;
            1
        }
        0xFE => {
            let v = cpu.fetchbyte();
            cpu.alu_cp(v);
            2
        }
        0xFF => {
            cpu.pushstack(cpu.reg.pc);
            cpu.reg.pc = 0x38;
            4
        }
        other => panic!("Instruction {:2X} is not implemented", other),
    }
}

pub fn call_cb(cpu: &mut CPU) -> u32 {
    let opcode = cpu.fetchbyte();
    match opcode {
        0x00 => {
            cpu.reg.b = cpu.alu_rlc(cpu.reg.b);
            2
        }
        0x01 => {
            cpu.reg.c = cpu.alu_rlc(cpu.reg.c);
            2
        }
        0x02 => {
            cpu.reg.d = cpu.alu_rlc(cpu.reg.d);
            2
        }
        0x03 => {
            cpu.reg.e = cpu.alu_rlc(cpu.reg.e);
            2
        }
        0x04 => {
            cpu.reg.h = cpu.alu_rlc(cpu.reg.h);
            2
        }
        0x05 => {
            cpu.reg.l = cpu.alu_rlc(cpu.reg.l);
            2
        }
        0x06 => {
            let a = cpu.reg.hl();
            let v = cpu.mmu.rb(a);
            let v2 = cpu.alu_rlc(v);
            cpu.mmu.wb(a, v2);
            4
        }
        0x07 => {
            cpu.reg.a = cpu.alu_rlc(cpu.reg.a);
            2
        }
        0x08 => {
            cpu.reg.b = cpu.alu_rrc(cpu.reg.b);
            2
        }
        0x09 => {
            cpu.reg.c = cpu.alu_rrc(cpu.reg.c);
            2
        }
        0x0A => {
            cpu.reg.d = cpu.alu_rrc(cpu.reg.d);
            2
        }
        0x0B => {
            cpu.reg.e = cpu.alu_rrc(cpu.reg.e);
            2
        }
        0x0C => {
            cpu.reg.h = cpu.alu_rrc(cpu.reg.h);
            2
        }
        0x0D => {
            cpu.reg.l = cpu.alu_rrc(cpu.reg.l);
            2
        }
        0x0E => {
            let a = cpu.reg.hl();
            let v = cpu.mmu.rb(a);
            let v2 = cpu.alu_rrc(v);
            cpu.mmu.wb(a, v2);
            4
        }
        0x0F => {
            cpu.reg.a = cpu.alu_rrc(cpu.reg.a);
            2
        }
        0x10 => {
            cpu.reg.b = cpu.alu_rl(cpu.reg.b);
            2
        }
        0x11 => {
            cpu.reg.c = cpu.alu_rl(cpu.reg.c);
            2
        }
        0x12 => {
            cpu.reg.d = cpu.alu_rl(cpu.reg.d);
            2
        }
        0x13 => {
            cpu.reg.e = cpu.alu_rl(cpu.reg.e);
            2
        }
        0x14 => {
            cpu.reg.h = cpu.alu_rl(cpu.reg.h);
            2
        }
        0x15 => {
            cpu.reg.l = cpu.alu_rl(cpu.reg.l);
            2
        }
        0x16 => {
            let a = cpu.reg.hl();
            let v = cpu.mmu.rb(a);
            let v2 = cpu.alu_rl(v);
            cpu.mmu.wb(a, v2);
            4
        }
        0x17 => {
            cpu.reg.a = cpu.alu_rl(cpu.reg.a);
            2
        }
        0x18 => {
            cpu.reg.b = cpu.alu_rr(cpu.reg.b);
            2
        }
        0x19 => {
            cpu.reg.c = cpu.alu_rr(cpu.reg.c);
            2
        }
        0x1A => {
            cpu.reg.d = cpu.alu_rr(cpu.reg.d);
            2
        }
        0x1B => {
            cpu.reg.e = cpu.alu_rr(cpu.reg.e);
            2
        }
        0x1C => {
            cpu.reg.h = cpu.alu_rr(cpu.reg.h);
            2
        }
        0x1D => {
            cpu.reg.l = cpu.alu_rr(cpu.reg.l);
            2
        }
        0x1E => {
            let a = cpu.reg.hl();
            let v = cpu.mmu.rb(a);
            let v2 = cpu.alu_rr(v);
            cpu.mmu.wb(a, v2);
            4
        }
        0x1F => {
            cpu.reg.a = cpu.alu_rr(cpu.reg.a);
            2
        }
        0x20 => {
            cpu.reg.b = cpu.alu_sla(cpu.reg.b);
            2
        }
        0x21 => {
            cpu.reg.c = cpu.alu_sla(cpu.reg.c);
            2
        }
        0x22 => {
            cpu.reg.d = cpu.alu_sla(cpu.reg.d);
            2
        }
        0x23 => {
            cpu.reg.e = cpu.alu_sla(cpu.reg.e);
            2
        }
        0x24 => {
            cpu.reg.h = cpu.alu_sla(cpu.reg.h);
            2
        }
        0x25 => {
            cpu.reg.l = cpu.alu_sla(cpu.reg.l);
            2
        }
        0x26 => {
            let a = cpu.reg.hl();
            let v = cpu.mmu.rb(a);
            let v2 = cpu.alu_sla(v);
            cpu.mmu.wb(a, v2);
            4
        }
        0x27 => {
            cpu.reg.a = cpu.alu_sla(cpu.reg.a);
            2
        }
        0x28 => {
            cpu.reg.b = cpu.alu_sra(cpu.reg.b);
            2
        }
        0x29 => {
            cpu.reg.c = cpu.alu_sra(cpu.reg.c);
            2
        }
        0x2A => {
            cpu.reg.d = cpu.alu_sra(cpu.reg.d);
            2
        }
        0x2B => {
            cpu.reg.e = cpu.alu_sra(cpu.reg.e);
            2
        }
        0x2C => {
            cpu.reg.h = cpu.alu_sra(cpu.reg.h);
            2
        }
        0x2D => {
            cpu.reg.l = cpu.alu_sra(cpu.reg.l);
            2
        }
        0x2E => {
            let a = cpu.reg.hl();
            let v = cpu.mmu.rb(a);
            let v2 = cpu.alu_sra(v);
            cpu.mmu.wb(a, v2);
            4
        }
        0x2F => {
            cpu.reg.a = cpu.alu_sra(cpu.reg.a);
            2
        }
        0x30 => {
            cpu.reg.b = cpu.alu_swap(cpu.reg.b);
            2
        }
        0x31 => {
            cpu.reg.c = cpu.alu_swap(cpu.reg.c);
            2
        }
        0x32 => {
            cpu.reg.d = cpu.alu_swap(cpu.reg.d);
            2
        }
        0x33 => {
            cpu.reg.e = cpu.alu_swap(cpu.reg.e);
            2
        }
        0x34 => {
            cpu.reg.h = cpu.alu_swap(cpu.reg.h);
            2
        }
        0x35 => {
            cpu.reg.l = cpu.alu_swap(cpu.reg.l);
            2
        }
        0x36 => {
            let a = cpu.reg.hl();
            let v = cpu.mmu.rb(a);
            let v2 = cpu.alu_swap(v);
            cpu.mmu.wb(a, v2);
            4
        }
        0x37 => {
            cpu.reg.a = cpu.alu_swap(cpu.reg.a);
            2
        }
        0x38 => {
            cpu.reg.b = cpu.alu_srl(cpu.reg.b);
            2
        }
        0x39 => {
            cpu.reg.c = cpu.alu_srl(cpu.reg.c);
            2
        }
        0x3A => {
            cpu.reg.d = cpu.alu_srl(cpu.reg.d);
            2
        }
        0x3B => {
            cpu.reg.e = cpu.alu_srl(cpu.reg.e);
            2
        }
        0x3C => {
            cpu.reg.h = cpu.alu_srl(cpu.reg.h);
            2
        }
        0x3D => {
            cpu.reg.l = cpu.alu_srl(cpu.reg.l);
            2
        }
        0x3E => {
            let a = cpu.reg.hl();
            let v = cpu.mmu.rb(a);
            let v2 = cpu.alu_srl(v);
            cpu.mmu.wb(a, v2);
            4
        }
        0x3F => {
            cpu.reg.a = cpu.alu_srl(cpu.reg.a);
            2
        }
        0x40 => {
            cpu.alu_bit(cpu.reg.b, 0);
            2
        }
        0x41 => {
            cpu.alu_bit(cpu.reg.c, 0);
            2
        }
        0x42 => {
            cpu.alu_bit(cpu.reg.d, 0);
            2
        }
        0x43 => {
            cpu.alu_bit(cpu.reg.e, 0);
            2
        }
        0x44 => {
            cpu.alu_bit(cpu.reg.h, 0);
            2
        }
        0x45 => {
            cpu.alu_bit(cpu.reg.l, 0);
            2
        }
        0x46 => {
            let v = cpu.mmu.rb(cpu.reg.hl());
            cpu.alu_bit(v, 0);
            3
        }
        0x47 => {
            cpu.alu_bit(cpu.reg.a, 0);
            2
        }
        0x48 => {
            cpu.alu_bit(cpu.reg.b, 1);
            2
        }
        0x49 => {
            cpu.alu_bit(cpu.reg.c, 1);
            2
        }
        0x4A => {
            cpu.alu_bit(cpu.reg.d, 1);
            2
        }
        0x4B => {
            cpu.alu_bit(cpu.reg.e, 1);
            2
        }
        0x4C => {
            cpu.alu_bit(cpu.reg.h, 1);
            2
        }
        0x4D => {
            cpu.alu_bit(cpu.reg.l, 1);
            2
        }
        0x4E => {
            let v = cpu.mmu.rb(cpu.reg.hl());
            cpu.alu_bit(v, 1);
            3
        }
        0x4F => {
            cpu.alu_bit(cpu.reg.a, 1);
            2
        }
        0x50 => {
            cpu.alu_bit(cpu.reg.b, 2);
            2
        }
        0x51 => {
            cpu.alu_bit(cpu.reg.c, 2);
            2
        }
        0x52 => {
            cpu.alu_bit(cpu.reg.d, 2);
            2
        }
        0x53 => {
            cpu.alu_bit(cpu.reg.e, 2);
            2
        }
        0x54 => {
            cpu.alu_bit(cpu.reg.h, 2);
            2
        }
        0x55 => {
            cpu.alu_bit(cpu.reg.l, 2);
            2
        }
        0x56 => {
            let v = cpu.mmu.rb(cpu.reg.hl());
            cpu.alu_bit(v, 2);
            3
        }
        0x57 => {
            cpu.alu_bit(cpu.reg.a, 2);
            2
        }
        0x58 => {
            cpu.alu_bit(cpu.reg.b, 3);
            2
        }
        0x59 => {
            cpu.alu_bit(cpu.reg.c, 3);
            2
        }
        0x5A => {
            cpu.alu_bit(cpu.reg.d, 3);
            2
        }
        0x5B => {
            cpu.alu_bit(cpu.reg.e, 3);
            2
        }
        0x5C => {
            cpu.alu_bit(cpu.reg.h, 3);
            2
        }
        0x5D => {
            cpu.alu_bit(cpu.reg.l, 3);
            2
        }
        0x5E => {
            let v = cpu.mmu.rb(cpu.reg.hl());
            cpu.alu_bit(v, 3);
            3
        }
        0x5F => {
            cpu.alu_bit(cpu.reg.a, 3);
            2
        }
        0x60 => {
            cpu.alu_bit(cpu.reg.b, 4);
            2
        }
        0x61 => {
            cpu.alu_bit(cpu.reg.c, 4);
            2
        }
        0x62 => {
            cpu.alu_bit(cpu.reg.d, 4);
            2
        }
        0x63 => {
            cpu.alu_bit(cpu.reg.e, 4);
            2
        }
        0x64 => {
            cpu.alu_bit(cpu.reg.h, 4);
            2
        }
        0x65 => {
            cpu.alu_bit(cpu.reg.l, 4);
            2
        }
        0x66 => {
            let v = cpu.mmu.rb(cpu.reg.hl());
            cpu.alu_bit(v, 4);
            3
        }
        0x67 => {
            cpu.alu_bit(cpu.reg.a, 4);
            2
        }
        0x68 => {
            cpu.alu_bit(cpu.reg.b, 5);
            2
        }
        0x69 => {
            cpu.alu_bit(cpu.reg.c, 5);
            2
        }
        0x6A => {
            cpu.alu_bit(cpu.reg.d, 5);
            2
        }
        0x6B => {
            cpu.alu_bit(cpu.reg.e, 5);
            2
        }
        0x6C => {
            cpu.alu_bit(cpu.reg.h, 5);
            2
        }
        0x6D => {
            cpu.alu_bit(cpu.reg.l, 5);
            2
        }
        0x6E => {
            let v = cpu.mmu.rb(cpu.reg.hl());
            cpu.alu_bit(v, 5);
            3
        }
        0x6F => {
            cpu.alu_bit(cpu.reg.a, 5);
            2
        }
        0x70 => {
            cpu.alu_bit(cpu.reg.b, 6);
            2
        }
        0x71 => {
            cpu.alu_bit(cpu.reg.c, 6);
            2
        }
        0x72 => {
            cpu.alu_bit(cpu.reg.d, 6);
            2
        }
        0x73 => {
            cpu.alu_bit(cpu.reg.e, 6);
            2
        }
        0x74 => {
            cpu.alu_bit(cpu.reg.h, 6);
            2
        }
        0x75 => {
            cpu.alu_bit(cpu.reg.l, 6);
            2
        }
        0x76 => {
            let v = cpu.mmu.rb(cpu.reg.hl());
            cpu.alu_bit(v, 6);
            3
        }
        0x77 => {
            cpu.alu_bit(cpu.reg.a, 6);
            2
        }
        0x78 => {
            cpu.alu_bit(cpu.reg.b, 7);
            2
        }
        0x79 => {
            cpu.alu_bit(cpu.reg.c, 7);
            2
        }
        0x7A => {
            cpu.alu_bit(cpu.reg.d, 7);
            2
        }
        0x7B => {
            cpu.alu_bit(cpu.reg.e, 7);
            2
        }
        0x7C => {
            cpu.alu_bit(cpu.reg.h, 7);
            2
        }
        0x7D => {
            cpu.alu_bit(cpu.reg.l, 7);
            2
        }
        0x7E => {
            let v = cpu.mmu.rb(cpu.reg.hl());
            cpu.alu_bit(v, 7);
            3
        }
        0x7F => {
            cpu.alu_bit(cpu.reg.a, 7);
            2
        }
        0x80 => {
            cpu.reg.b = cpu.reg.b & !(1 << 0);
            2
        }
        0x81 => {
            cpu.reg.c = cpu.reg.c & !(1 << 0);
            2
        }
        0x82 => {
            cpu.reg.d = cpu.reg.d & !(1 << 0);
            2
        }
        0x83 => {
            cpu.reg.e = cpu.reg.e & !(1 << 0);
            2
        }
        0x84 => {
            cpu.reg.h = cpu.reg.h & !(1 << 0);
            2
        }
        0x85 => {
            cpu.reg.l = cpu.reg.l & !(1 << 0);
            2
        }
        0x86 => {
            let a = cpu.reg.hl();
            let v = cpu.mmu.rb(a) & !(1 << 0);
            cpu.mmu.wb(a, v);
            4
        }
        0x87 => {
            cpu.reg.a = cpu.reg.a & !(1 << 0);
            2
        }
        0x88 => {
            cpu.reg.b = cpu.reg.b & !(1 << 1);
            2
        }
        0x89 => {
            cpu.reg.c = cpu.reg.c & !(1 << 1);
            2
        }
        0x8A => {
            cpu.reg.d = cpu.reg.d & !(1 << 1);
            2
        }
        0x8B => {
            cpu.reg.e = cpu.reg.e & !(1 << 1);
            2
        }
        0x8C => {
            cpu.reg.h = cpu.reg.h & !(1 << 1);
            2
        }
        0x8D => {
            cpu.reg.l = cpu.reg.l & !(1 << 1);
            2
        }
        0x8E => {
            let a = cpu.reg.hl();
            let v = cpu.mmu.rb(a) & !(1 << 1);
            cpu.mmu.wb(a, v);
            4
        }
        0x8F => {
            cpu.reg.a = cpu.reg.a & !(1 << 1);
            2
        }
        0x90 => {
            cpu.reg.b = cpu.reg.b & !(1 << 2);
            2
        }
        0x91 => {
            cpu.reg.c = cpu.reg.c & !(1 << 2);
            2
        }
        0x92 => {
            cpu.reg.d = cpu.reg.d & !(1 << 2);
            2
        }
        0x93 => {
            cpu.reg.e = cpu.reg.e & !(1 << 2);
            2
        }
        0x94 => {
            cpu.reg.h = cpu.reg.h & !(1 << 2);
            2
        }
        0x95 => {
            cpu.reg.l = cpu.reg.l & !(1 << 2);
            2
        }
        0x96 => {
            let a = cpu.reg.hl();
            let v = cpu.mmu.rb(a) & !(1 << 2);
            cpu.mmu.wb(a, v);
            4
        }
        0x97 => {
            cpu.reg.a = cpu.reg.a & !(1 << 2);
            2
        }
        0x98 => {
            cpu.reg.b = cpu.reg.b & !(1 << 3);
            2
        }
        0x99 => {
            cpu.reg.c = cpu.reg.c & !(1 << 3);
            2
        }
        0x9A => {
            cpu.reg.d = cpu.reg.d & !(1 << 3);
            2
        }
        0x9B => {
            cpu.reg.e = cpu.reg.e & !(1 << 3);
            2
        }
        0x9C => {
            cpu.reg.h = cpu.reg.h & !(1 << 3);
            2
        }
        0x9D => {
            cpu.reg.l = cpu.reg.l & !(1 << 3);
            2
        }
        0x9E => {
            let a = cpu.reg.hl();
            let v = cpu.mmu.rb(a) & !(1 << 3);
            cpu.mmu.wb(a, v);
            4
        }
        0x9F => {
            cpu.reg.a = cpu.reg.a & !(1 << 3);
            2
        }
        0xA0 => {
            cpu.reg.b = cpu.reg.b & !(1 << 4);
            2
        }
        0xA1 => {
            cpu.reg.c = cpu.reg.c & !(1 << 4);
            2
        }
        0xA2 => {
            cpu.reg.d = cpu.reg.d & !(1 << 4);
            2
        }
        0xA3 => {
            cpu.reg.e = cpu.reg.e & !(1 << 4);
            2
        }
        0xA4 => {
            cpu.reg.h = cpu.reg.h & !(1 << 4);
            2
        }
        0xA5 => {
            cpu.reg.l = cpu.reg.l & !(1 << 4);
            2
        }
        0xA6 => {
            let a = cpu.reg.hl();
            let v = cpu.mmu.rb(a) & !(1 << 4);
            cpu.mmu.wb(a, v);
            4
        }
        0xA7 => {
            cpu.reg.a = cpu.reg.a & !(1 << 4);
            2
        }
        0xA8 => {
            cpu.reg.b = cpu.reg.b & !(1 << 5);
            2
        }
        0xA9 => {
            cpu.reg.c = cpu.reg.c & !(1 << 5);
            2
        }
        0xAA => {
            cpu.reg.d = cpu.reg.d & !(1 << 5);
            2
        }
        0xAB => {
            cpu.reg.e = cpu.reg.e & !(1 << 5);
            2
        }
        0xAC => {
            cpu.reg.h = cpu.reg.h & !(1 << 5);
            2
        }
        0xAD => {
            cpu.reg.l = cpu.reg.l & !(1 << 5);
            2
        }
        0xAE => {
            let a = cpu.reg.hl();
            let v = cpu.mmu.rb(a) & !(1 << 5);
            cpu.mmu.wb(a, v);
            4
        }
        0xAF => {
            cpu.reg.a = cpu.reg.a & !(1 << 5);
            2
        }
        0xB0 => {
            cpu.reg.b = cpu.reg.b & !(1 << 6);
            2
        }
        0xB1 => {
            cpu.reg.c = cpu.reg.c & !(1 << 6);
            2
        }
        0xB2 => {
            cpu.reg.d = cpu.reg.d & !(1 << 6);
            2
        }
        0xB3 => {
            cpu.reg.e = cpu.reg.e & !(1 << 6);
            2
        }
        0xB4 => {
            cpu.reg.h = cpu.reg.h & !(1 << 6);
            2
        }
        0xB5 => {
            cpu.reg.l = cpu.reg.l & !(1 << 6);
            2
        }
        0xB6 => {
            let a = cpu.reg.hl();
            let v = cpu.mmu.rb(a) & !(1 << 6);
            cpu.mmu.wb(a, v);
            4
        }
        0xB7 => {
            cpu.reg.a = cpu.reg.a & !(1 << 6);
            2
        }
        0xB8 => {
            cpu.reg.b = cpu.reg.b & !(1 << 7);
            2
        }
        0xB9 => {
            cpu.reg.c = cpu.reg.c & !(1 << 7);
            2
        }
        0xBA => {
            cpu.reg.d = cpu.reg.d & !(1 << 7);
            2
        }
        0xBB => {
            cpu.reg.e = cpu.reg.e & !(1 << 7);
            2
        }
        0xBC => {
            cpu.reg.h = cpu.reg.h & !(1 << 7);
            2
        }
        0xBD => {
            cpu.reg.l = cpu.reg.l & !(1 << 7);
            2
        }
        0xBE => {
            let a = cpu.reg.hl();
            let v = cpu.mmu.rb(a) & !(1 << 7);
            cpu.mmu.wb(a, v);
            4
        }
        0xBF => {
            cpu.reg.a = cpu.reg.a & !(1 << 7);
            2
        }
        0xC0 => {
            cpu.reg.b = cpu.reg.b | (1 << 0);
            2
        }
        0xC1 => {
            cpu.reg.c = cpu.reg.c | (1 << 0);
            2
        }
        0xC2 => {
            cpu.reg.d = cpu.reg.d | (1 << 0);
            2
        }
        0xC3 => {
            cpu.reg.e = cpu.reg.e | (1 << 0);
            2
        }
        0xC4 => {
            cpu.reg.h = cpu.reg.h | (1 << 0);
            2
        }
        0xC5 => {
            cpu.reg.l = cpu.reg.l | (1 << 0);
            2
        }
        0xC6 => {
            let a = cpu.reg.hl();
            let v = cpu.mmu.rb(a) | (1 << 0);
            cpu.mmu.wb(a, v);
            4
        }
        0xC7 => {
            cpu.reg.a = cpu.reg.a | (1 << 0);
            2
        }
        0xC8 => {
            cpu.reg.b = cpu.reg.b | (1 << 1);
            2
        }
        0xC9 => {
            cpu.reg.c = cpu.reg.c | (1 << 1);
            2
        }
        0xCA => {
            cpu.reg.d = cpu.reg.d | (1 << 1);
            2
        }
        0xCB => {
            cpu.reg.e = cpu.reg.e | (1 << 1);
            2
        }
        0xCC => {
            cpu.reg.h = cpu.reg.h | (1 << 1);
            2
        }
        0xCD => {
            cpu.reg.l = cpu.reg.l | (1 << 1);
            2
        }
        0xCE => {
            let a = cpu.reg.hl();
            let v = cpu.mmu.rb(a) | (1 << 1);
            cpu.mmu.wb(a, v);
            4
        }
        0xCF => {
            cpu.reg.a = cpu.reg.a | (1 << 1);
            2
        }
        0xD0 => {
            cpu.reg.b = cpu.reg.b | (1 << 2);
            2
        }
        0xD1 => {
            cpu.reg.c = cpu.reg.c | (1 << 2);
            2
        }
        0xD2 => {
            cpu.reg.d = cpu.reg.d | (1 << 2);
            2
        }
        0xD3 => {
            cpu.reg.e = cpu.reg.e | (1 << 2);
            2
        }
        0xD4 => {
            cpu.reg.h = cpu.reg.h | (1 << 2);
            2
        }
        0xD5 => {
            cpu.reg.l = cpu.reg.l | (1 << 2);
            2
        }
        0xD6 => {
            let a = cpu.reg.hl();
            let v = cpu.mmu.rb(a) | (1 << 2);
            cpu.mmu.wb(a, v);
            4
        }
        0xD7 => {
            cpu.reg.a = cpu.reg.a | (1 << 2);
            2
        }
        0xD8 => {
            cpu.reg.b = cpu.reg.b | (1 << 3);
            2
        }
        0xD9 => {
            cpu.reg.c = cpu.reg.c | (1 << 3);
            2
        }
        0xDA => {
            cpu.reg.d = cpu.reg.d | (1 << 3);
            2
        }
        0xDB => {
            cpu.reg.e = cpu.reg.e | (1 << 3);
            2
        }
        0xDC => {
            cpu.reg.h = cpu.reg.h | (1 << 3);
            2
        }
        0xDD => {
            cpu.reg.l = cpu.reg.l | (1 << 3);
            2
        }
        0xDE => {
            let a = cpu.reg.hl();
            let v = cpu.mmu.rb(a) | (1 << 3);
            cpu.mmu.wb(a, v);
            4
        }
        0xDF => {
            cpu.reg.a = cpu.reg.a | (1 << 3);
            2
        }
        0xE0 => {
            cpu.reg.b = cpu.reg.b | (1 << 4);
            2
        }
        0xE1 => {
            cpu.reg.c = cpu.reg.c | (1 << 4);
            2
        }
        0xE2 => {
            cpu.reg.d = cpu.reg.d | (1 << 4);
            2
        }
        0xE3 => {
            cpu.reg.e = cpu.reg.e | (1 << 4);
            2
        }
        0xE4 => {
            cpu.reg.h = cpu.reg.h | (1 << 4);
            2
        }
        0xE5 => {
            cpu.reg.l = cpu.reg.l | (1 << 4);
            2
        }
        0xE6 => {
            let a = cpu.reg.hl();
            let v = cpu.mmu.rb(a) | (1 << 4);
            cpu.mmu.wb(a, v);
            4
        }
        0xE7 => {
            cpu.reg.a = cpu.reg.a | (1 << 4);
            2
        }
        0xE8 => {
            cpu.reg.b = cpu.reg.b | (1 << 5);
            2
        }
        0xE9 => {
            cpu.reg.c = cpu.reg.c | (1 << 5);
            2
        }
        0xEA => {
            cpu.reg.d = cpu.reg.d | (1 << 5);
            2
        }
        0xEB => {
            cpu.reg.e = cpu.reg.e | (1 << 5);
            2
        }
        0xEC => {
            cpu.reg.h = cpu.reg.h | (1 << 5);
            2
        }
        0xED => {
            cpu.reg.l = cpu.reg.l | (1 << 5);
            2
        }
        0xEE => {
            let a = cpu.reg.hl();
            let v = cpu.mmu.rb(a) | (1 << 5);
            cpu.mmu.wb(a, v);
            4
        }
        0xEF => {
            cpu.reg.a = cpu.reg.a | (1 << 5);
            2
        }
        0xF0 => {
            cpu.reg.b = cpu.reg.b | (1 << 6);
            2
        }
        0xF1 => {
            cpu.reg.c = cpu.reg.c | (1 << 6);
            2
        }
        0xF2 => {
            cpu.reg.d = cpu.reg.d | (1 << 6);
            2
        }
        0xF3 => {
            cpu.reg.e = cpu.reg.e | (1 << 6);
            2
        }
        0xF4 => {
            cpu.reg.h = cpu.reg.h | (1 << 6);
            2
        }
        0xF5 => {
            cpu.reg.l = cpu.reg.l | (1 << 6);
            2
        }
        0xF6 => {
            let a = cpu.reg.hl();
            let v = cpu.mmu.rb(a) | (1 << 6);
            cpu.mmu.wb(a, v);
            4
        }
        0xF7 => {
            cpu.reg.a = cpu.reg.a | (1 << 6);
            2
        }
        0xF8 => {
            cpu.reg.b = cpu.reg.b | (1 << 7);
            2
        }
        0xF9 => {
            cpu.reg.c = cpu.reg.c | (1 << 7);
            2
        }
        0xFA => {
            cpu.reg.d = cpu.reg.d | (1 << 7);
            2
        }
        0xFB => {
            cpu.reg.e = cpu.reg.e | (1 << 7);
            2
        }
        0xFC => {
            cpu.reg.h = cpu.reg.h | (1 << 7);
            2
        }
        0xFD => {
            cpu.reg.l = cpu.reg.l | (1 << 7);
            2
        }
        0xFE => {
            let a = cpu.reg.hl();
            let v = cpu.mmu.rb(a) | (1 << 7);
            cpu.mmu.wb(a, v);
            4
        }
        0xFF => {
            cpu.reg.a = cpu.reg.a | (1 << 7);
            2
        }
    }
}
