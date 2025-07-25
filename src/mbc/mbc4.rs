use crate::mbc::{ram_banks, rom_banks, MBC};
use crate::StrResult;

pub struct MBC4 {
    rom: Vec<u8>,
    ram: Vec<u8>,
    rombank: usize,
    rambank: usize,
    ram_on: bool,
    ram_updated: bool,
    has_battery: bool,
    rombanks: usize,
    rambanks: usize,
}

impl MBC4 {
    pub fn new(data: Vec<u8>) -> StrResult<MBC4> {
        let subtype = data[0x147];
        let has_battery = matches!(subtype, 0x1B | 0x1E);
        let rambanks = match subtype {
            0x1A | 0x1B | 0x1D | 0x1E => ram_banks(data[0x149]),
            _ => 0,
        };
        let ramsize = 0x2000 * rambanks;
        let rombanks = rom_banks(data[0x148]);

        let res = MBC4 {
            rom: data,
            ram: vec![0u8; ramsize],
            rombank: 1,
            rambank: 0,
            ram_updated: false,
            ram_on: false,
            has_battery,
            rombanks,
            rambanks,
        };

        Ok(res)
    }
}

impl MBC for MBC4 {
    fn readrom(&self, a: u16) -> u8 {
        let idx = if a < 0x4000 {
            a as usize
        } else {
            self.rombank * 0x4000 | ((a as usize) & 0x3FFF)
        };
        *self.rom.get(idx).unwrap_or(&0)
    }

    fn readram(&self, a: u16) -> u8 {
        if !self.ram_on {
            return 0;
        }
        self.ram[self.rambank * 0x2000 | ((a as usize) & 0x1FFF)]
    }

    fn writerom(&mut self, a: u16, v: u8) {
        match a {
            0x0000..=0x1FFF => self.ram_on = v & 0x0F == 0x0A,
            0x2000..=0x2FFF => {
                self.rombank = ((self.rombank & 0x100) | (v as usize)) % self.rombanks
            }
            0x3000..=0x3FFF => {
                self.rombank = ((self.rombank & 0x0FF) | (((v & 0x1) as usize) << 8)) % self.rombanks
            }
            0x4000..=0x5FFF => self.rambank = ((v & 0x0F) as usize) % self.rambanks,
            0x6000..=0x7FFF => { /* unused */ }
            _ => panic!("Could not write to {:04X} (MBC4)", a),
        }
    }

    fn writeram(&mut self, a: u16, v: u8) {
        if !self.ram_on {
            return;
        }
        self.ram[self.rambank * 0x2000 | ((a as usize) & 0x1FFF)] = v;
        self.ram_updated = true;
    }

    fn is_battery_backed(&self) -> bool {
        self.has_battery
    }

    fn loadram(&mut self, ramdata: &[u8]) -> StrResult<()> {
        if ramdata.len() != self.ram.len() {
            return Err("Loaded RAM has incorrect length");
        }
        self.ram = ramdata.to_vec();
        Ok(())
    }

    fn dumpram(&self) -> Vec<u8> {
        self.ram.to_vec()
    }

    fn check_and_reset_ram_updated(&mut self) -> bool {
        let res = self.ram_updated;
        self.ram_updated = false;
        res
    }
}
