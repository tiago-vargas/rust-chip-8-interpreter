use std::path::Path;

pub struct Machine {
    pub rom_bytes: Vec<u8>,
    pub video_buffer: [[u8; 64]; 32],
    program_counter: u16,
}

impl Machine {
    pub fn new() -> Self {
        Machine {
            rom_bytes: vec![],
            video_buffer: [[0; 64]; 32],
            program_counter: 0,
        }
    }

    pub fn load_rom<P: AsRef<Path>>(&mut self, rom_path: P) {
        let rom = std::fs::read(rom_path);
        self.rom_bytes = rom.unwrap();
    }

    pub fn run(&self) {
        todo!()
    }

    fn decode(&mut self, opcode: u16) {
        match opcode {
            0x00E0 => self.video_buffer = [[0; 64]; 32],
            0x1000..=0x1FFF => self.program_counter = opcode - 0x1000,
            _ => todo!()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Machine;

    use std::path::Path;

    #[test]
    fn loads_ibm_rom() {
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

    #[test]
    fn decodes_00e0_as_clear_screen() {
        let mut machine = Machine::new();
        let buffer_of_ones = [[1; 64]; 32];
        machine.video_buffer = buffer_of_ones;

        machine.decode(0x00E0u16);

        let buffer_of_zeros = [[0; 64]; 32];
        assert_eq!(machine.video_buffer, buffer_of_zeros);
    }

    #[test]
    fn decodes_1nnn_as_jump() {
        let mut machine = Machine::new();

        machine.decode(0x1ACEu16); // NNN = 0xACE

        assert_eq!(machine.program_counter, 0x0ACE);
    }
}
