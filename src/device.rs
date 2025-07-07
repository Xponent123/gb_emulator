use crate::cpu::CPU;
use crate::gbmode::GbMode;
use crate::keypad::KeypadKey;
use crate::mbc;
use crate::sound;
use crate::StrResult;

pub struct Device {
    cpu: CPU,
}

impl Device {
    pub fn new(
        romname: &str,
        _skip_checksum: bool,
        _save_state: Option<String>,
    ) -> StrResult<Device> {
        let cart = mbc::FileBackedMBC::new(romname.into(), false)?;
        CPU::new(Box::new(cart)).map(|cpu| Device { cpu })
    }

    pub fn new_cgb(
        romname: &str,
        _skip_checksum: bool,
        _save_state: Option<String>,
    ) -> StrResult<Device> {
        let cart = mbc::FileBackedMBC::new(romname.into(), false)?;
        CPU::new_cgb(Box::new(cart)).map(|cpu| Device { cpu })
    }

    pub fn do_cycle(&mut self) -> u32 {
        self.cpu.do_cycle()
    }

    pub fn check_and_reset_gpu_updated(&mut self) -> bool {
        let result = self.cpu.mmu.gpu.updated;
        self.cpu.mmu.gpu.updated = false;
        result
    }

    pub fn get_gpu_data(&self) -> &[u8] {
        &self.cpu.mmu.gpu.data
    }

    pub fn enable_audio(&mut self, player: Box<dyn sound::AudioPlayer>, is_on: bool) {
        match self.cpu.mmu.gbmode {
            GbMode::Classic => {
                self.cpu.mmu.sound = Some(sound::Sound::new_dmg(player));
            }
            GbMode::Color | GbMode::ColorAsClassic => {
                self.cpu.mmu.sound = Some(sound::Sound::new_cgb(player));
            }
        };
        if is_on {
            if let Some(sound) = self.cpu.mmu.sound.as_mut() {
                sound.set_on();
            }
        }
    }

    pub fn sync_audio(&mut self) {
        if let Some(ref mut sound) = self.cpu.mmu.sound {
            sound.sync();
        }
    }

    pub fn keyup(&mut self, key: KeypadKey) {
        self.cpu.mmu.keypad.keyup(key);
    }

    pub fn keydown(&mut self, key: KeypadKey) {
        self.cpu.mmu.keypad.keydown(key);
    }

    pub fn romname(&self) -> String {
        self.cpu.mmu.mbc.romname()
    }
}
