use std::path::Path;

pub(crate) struct Machine {
    pub(crate) rom_bytes: Vec<u8>,
}

impl Machine {
    pub(crate) fn new() -> Self {
        Machine { rom_bytes: vec![] }
    }

    pub(crate) fn load_rom<P: AsRef<Path>>(&mut self, rom_path: P) {
        let rom = std::fs::read(rom_path);
        self.rom_bytes = rom.unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::Machine;

    use std::path::Path;

    #[test]
    pub(crate) fn loads_ibm_rom() {
        let mut machine = Machine::new();

        machine.load_rom(Path::new("./assets/IBM Logo.ch8"));

        let expected_bytes: [u8; 132] = [
            0, 224, 162, 42, 96, 12, 97, 8, 208, 31, 112, 9, 162, 57, 208, 31, 162, 72, 112, 8,
            208, 31, 112, 4, 162, 87, 208, 31, 112, 8, 162, 102, 208, 31, 112, 8, 162, 117, 208,
            31, 18, 40, 255, 0, 255, 0, 60, 0, 60, 0, 60, 0, 60, 0, 255, 0, 255, 255, 0, 255, 0,
            56, 0, 63, 0, 63, 0, 56, 0, 255, 0, 255, 128, 0, 224, 0, 224, 0, 128, 0, 128, 0, 224,
            0, 224, 0, 128, 248, 0, 252, 0, 62, 0, 63, 0, 59, 0, 57, 0, 248, 0, 248, 3, 0, 7, 0,
            15, 0, 191, 0, 251, 0, 243, 0, 227, 0, 67, 224, 0, 224, 0, 128, 0, 128, 0, 128, 0, 128,
            0, 224, 0, 224,
        ];
        assert_eq!(machine.rom_bytes, expected_bytes);
    }
}
