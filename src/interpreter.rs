use std::path::Path;

pub struct Machine {
    pub rom_bytes: Vec<u8>,
    pub video_buffer: [[u8; 64]; 32],  // 64 pixels x 32 pixels === 64 bit x 32 bit, but there's no `u1`
    program_counter: u16,
    variable_register: [u8; 16],
    index_register: u16,
    memory: [u8; 4096],
}

impl Machine {
    pub fn new() -> Self {
        Machine {
            rom_bytes: vec![],
            video_buffer: [[0; 64]; 32],
            program_counter: 0,
            variable_register: [0; 16],
            index_register: 0,
            memory: [0; 4096],
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
            0x00E0 => self.clear_screen(),
            0x1000..=0x1FFF => {
                let address = opcode & 0x0FFF;
                self.jump(address);
            }
            0x6000..=0x6FFF => {
                let index = (opcode & 0x0F00) >> 8;
                let value = opcode & 0x00FF;
                self.set_vx(index, value as u8);
            }
            0x7000..=0x7FFF => {
                let index = (opcode & 0x0F00) >> 8;
                let value = opcode & 0x00FF;
                self.add_to_vx(index, value as u8);
            }
            0xA000..=0xAFFF => {
                let value = opcode & 0x0FFF;
                self.set_i(value);
            }
            0xD000..=0xDFFF => {
                let vx = (opcode & 0x0F00) >> 8;
                let x_position = self.variable_register[vx as usize];
                let vy = (opcode & 0x00F0) >> 4;
                let y_position = self.variable_register[vy as usize];
                let height = opcode & 0x000F;
                let address = self.index_register;
                self.draw(x_position, y_position, height as u8, address);
            }
            _ => todo!()
        }
    }

    fn clear_screen(&mut self) {
        self.video_buffer = [[0; 64]; 32]
    }

    fn jump(&mut self, address: u16) {
        self.program_counter = address;
    }

    fn set_vx(&mut self, x: u16, value: u8) {
        self.variable_register[x as usize] = value;
    }

    fn add_to_vx(&mut self, x: u16, value: u8) {
        self.variable_register[x as usize] = self.variable_register[x as usize].wrapping_add(value);
    }

    fn set_i(&mut self, value: u16) {
        self.index_register = value;
    }

    fn draw(&mut self, x: u8, y: u8, height: u8, address: u16) {
        let line = y as usize;
        let column = x as usize;
        for h in 0..height as usize {
            let image = self.memory[(address as usize) + h];
            let bytes = [
                image & 0b1000_0000,
                image & 0b0100_0000,
                image & 0b0010_0000,
                image & 0b0001_0000,
                image & 0b0000_1000,
                image & 0b0000_0100,
                image & 0b0000_0010,
                image & 0b0000_0001,
            ];

            for i in 0..8 {
                if bytes[i] == 0 {
                    self.video_buffer[line + h][column + i] = 0;
                } else {
                    self.video_buffer[line + h][column + i] = 1;
                }
            }
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
            0x00, 0xE0, 0xA2, 0x2A, 0x60, 0x0C, 0x61, 0x08, 0xD0, 0x1F, 0x70, 0x09, 0xA2, 0x39,
            0xD0, 0x1F, 0xA2, 0x48, 0x70, 0x08, 0xD0, 0x1F, 0x70, 0x04, 0xA2, 0x57, 0xD0, 0x1F,
            0x70, 0x08, 0xA2, 0x66, 0xD0, 0x1F, 0x70, 0x08, 0xA2, 0x75, 0xD0, 0x1F, 0x12, 0x28,
            0xFF, 0x00, 0xFF, 0x00, 0x3C, 0x00, 0x3C, 0x00, 0x3C, 0x00, 0x3C, 0x00, 0xFF, 0x00,
            0xFF, 0xFF, 0x00, 0xFF, 0x00, 0x38, 0x00, 0x3F, 0x00, 0x3F, 0x00, 0x38, 0x00, 0xFF,
            0x00, 0xFF, 0x80, 0x00, 0xE0, 0x00, 0xE0, 0x00, 0x80, 0x00, 0x80, 0x00, 0xE0, 0x00,
            0xE0, 0x00, 0x80, 0xF8, 0x00, 0xFC, 0x00, 0x3E, 0x00, 0x3F, 0x00, 0x3B, 0x00, 0x39,
            0x00, 0xF8, 0x00, 0xF8, 0x03, 0x00, 0x07, 0x00, 0x0F, 0x00, 0xBF, 0x00, 0xFB, 0x00,
            0xF3, 0x00, 0xE3, 0x00, 0x43, 0xE0, 0x00, 0xE0, 0x00, 0x80, 0x00, 0x80, 0x00, 0x80,
            0x00, 0x80, 0x00, 0xE0, 0x00, 0xE0,
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

    #[test]
    fn decodes_6xnn_as_set_vx() {
        let mut machine = Machine::new();

        machine.decode(0x6ACEu16);  // X = 0xA; NN = 0xCE

        assert_eq!(machine.variable_register[0xA], 0xCE);
    }

    #[test]
    fn decodes_7xnn_as_add_value_to_vx_no_overflow() {
        let mut machine = Machine::new();
        machine.variable_register[0xA] = 0x31;
        let initial_value = machine.variable_register[0xA];

        machine.decode(0x7ACEu16);  // X = 0xA; NN = 0xCE

        assert_eq!(machine.variable_register[0xA], initial_value + 0xCE);
    }

    #[test]
    fn decodes_7xnn_as_add_value_to_vx_with_overflow() {
        let mut machine = Machine::new();
        machine.variable_register[0xA] = 0xF1;
        let initial_value = machine.variable_register[0xA];

        machine.decode(0x7ACEu16);  // X = 0xA; NN = 0xCE

        assert_eq!(machine.variable_register[0xA], initial_value.wrapping_add(0xCE));
    }

    #[test]
    fn decodes_annn_as_set_index_register() {
        let mut machine = Machine::new();
        machine.index_register = 0xF1;

        machine.decode(0xAF23u16);  // NNN = 0xF23

        assert_eq!(machine.index_register, 0xF23);
    }

    #[test]
    fn decodes_dxyn_as_display_or_draw_1_pixel_tall_when_empty_buffer() {
        let mut machine = Machine::new();
        let i = 2048;
        machine.index_register = i;
        machine.memory[i as usize] = 0b1000_0000;
        machine.variable_register[0xF] = 0x1;
        machine.variable_register[0xE] = 0x1;
        // "Draw the sprite (1 pixel tall) at (x, y) = (1, 1) = (col, lin)"

        machine.decode(0xDFE1u16); // NNN = 0xFE1

        let mut expected_buffer = [[0; 64]; 32];
        expected_buffer[1][1] = 1;
        assert_eq!(machine.video_buffer, expected_buffer, "actual: {:#?}\nexpected: {expected_buffer:#?}", machine.video_buffer);
    }

    #[test]
    fn decodes_dxyn_as_display_or_draw_taller_sprite_when_empty_buffer() {
        let mut machine = Machine::new();
        let i: usize = 2048;
        machine.index_register = i as u16;
        machine.memory[i + 0] = 0b1100_0011;  //
        machine.memory[i + 1] = 0b0110_0110;  //
        machine.memory[i + 2] = 0b0011_1100;  // Kinda like an X
        machine.memory[i + 3] = 0b0110_0110;  //
        machine.memory[i + 4] = 0b1100_0011;  //
        machine.variable_register[0xF] = 0x7;  // X: column
        machine.variable_register[0x2] = 0xB;  // Y: line
        // "Draw the X sprite (5 bit tall) at (x, y) = (0x7, 0xB)"
        // Therefore it goes from (0x7, 0xB) to (0x7 + 0x7, 0xB + 0x4)

        machine.decode(0xDF25u16); // NNN = 0xF25

        let mut expected_buffer = [[0; 64]; 32];
        //
        expected_buffer[0xB][0x7 + 0] = 1;
        expected_buffer[0xB][0x7 + 1] = 1;
        expected_buffer[0xB][0x7 + 2] = 0;
        expected_buffer[0xB][0x7 + 3] = 0;
        expected_buffer[0xB][0x7 + 4] = 0;
        expected_buffer[0xB][0x7 + 5] = 0;
        expected_buffer[0xB][0x7 + 6] = 1;
        expected_buffer[0xB][0x7 + 7] = 1;
        //
        expected_buffer[0xB + 1][0x7 + 0] = 0;
        expected_buffer[0xB + 1][0x7 + 1] = 1;
        expected_buffer[0xB + 1][0x7 + 2] = 1;
        expected_buffer[0xB + 1][0x7 + 3] = 0;
        expected_buffer[0xB + 1][0x7 + 4] = 0;
        expected_buffer[0xB + 1][0x7 + 5] = 1;
        expected_buffer[0xB + 1][0x7 + 6] = 1;
        expected_buffer[0xB + 1][0x7 + 7] = 0;
        //
        expected_buffer[0xB + 2][0x7 + 0] = 0;
        expected_buffer[0xB + 2][0x7 + 1] = 0;
        expected_buffer[0xB + 2][0x7 + 2] = 1;
        expected_buffer[0xB + 2][0x7 + 3] = 1;
        expected_buffer[0xB + 2][0x7 + 4] = 1;
        expected_buffer[0xB + 2][0x7 + 5] = 1;
        expected_buffer[0xB + 2][0x7 + 6] = 0;
        expected_buffer[0xB + 2][0x7 + 7] = 0;
        //
        expected_buffer[0xB + 3][0x7 + 0] = 0;
        expected_buffer[0xB + 3][0x7 + 1] = 1;
        expected_buffer[0xB + 3][0x7 + 2] = 1;
        expected_buffer[0xB + 3][0x7 + 3] = 0;
        expected_buffer[0xB + 3][0x7 + 4] = 0;
        expected_buffer[0xB + 3][0x7 + 5] = 1;
        expected_buffer[0xB + 3][0x7 + 6] = 1;
        expected_buffer[0xB + 3][0x7 + 7] = 0;
        //
        expected_buffer[0xB + 4][0x7 + 0] = 1;
        expected_buffer[0xB + 4][0x7 + 1] = 1;
        expected_buffer[0xB + 4][0x7 + 2] = 0;
        expected_buffer[0xB + 4][0x7 + 3] = 0;
        expected_buffer[0xB + 4][0x7 + 4] = 0;
        expected_buffer[0xB + 4][0x7 + 5] = 0;
        expected_buffer[0xB + 4][0x7 + 6] = 1;
        expected_buffer[0xB + 4][0x7 + 7] = 1;
        assert_eq!(machine.video_buffer, expected_buffer);
    }
}
