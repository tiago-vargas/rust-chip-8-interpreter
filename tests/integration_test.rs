use std::path::Path;

use rust_chip_8_interpreter as chip_8;
use chip_8::interpreter::Machine;

#[test]
fn given_ibm_logo_rom_then_check_video_buffer() {
    let mut machine = Machine::new();

    machine.load_rom(Path::new("./assets/IBM Logo.ch8"));
	machine.run();

	let expected_buffer = [
        [0; 64],
        [0; 64],
        [0; 64],
        [0; 64],
        [0; 64],
        [0; 64],
        [0; 64],
        [0; 64],  //                        ████████████████████████   ███████████████████████████         ███████████████                           ███████████████
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0; 64],  //                        ████████████████████████   █████████████████████████████████   ██████████████████                     ██████████████████
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0; 64],  //                              ████████████               █████████         █████████         ███████████████               ███████████████
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0; 64],  //                              ████████████               ███████████████████████████         █████████████████████   █████████████████████
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0; 64],  //                              ████████████               ███████████████████████████         █████████   █████████████████████   █████████
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0; 64],  //                              ████████████               █████████         █████████         █████████      ███████████████      █████████
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 1, 1, 1, 0, 0, 0, 1, 1, 1, 0, 0, 1, 1, 1, 1, 1, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0; 64],  //                        ████████████████████████   █████████████████████████████████   ██████████████████      █████████      ██████████████████
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 0, 0, 1, 1, 1, 0, 0, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0; 64],  //                        ████████████████████████   ███████████████████████████         ██████████████████         ███         ██████████████████
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 0, 0, 0, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0; 64],
        [0; 64],
        [0; 64],
        [0; 64],
        [0; 64],
        [0; 64],
        [0; 64],
        [0; 64],
        [0; 64],
    ];
	assert_eq!(machine.video_buffer, expected_buffer);
}

// #[test]
// fn given_ibm_logo_rom_then_check_video_buffer_alt() {
//     let mut machine = Machine::new();

//     machine.load_rom(Path::new("./assets/IBM Logo.ch8"));
// 	machine.run();

// 	let expected_buffer: [[u8; 8]; 32] = [
//         [0; 8],
//         [0; 8],
//         [0; 8],
//         [0; 8],
//         [0; 8],
//         [0; 8],
//         [0; 8],
//         [0; 8],  //     ░░░░ ████    ████ ░███    ████ ██░░    ░███ ██░░    ░░░░ ░░░█    ████ ░░░░
//         [0b0000_0000, 0b0000_1111, 0b1111_0111, 0b1111_1100, 0b0111_1100, 0b0000_0001, 0b1111_0000, 0b0000_0000],
//         [0; 8],  //     ░░░░ ████    ████ ░███    ████ ████    ░███ ███░    ░░░░ ░░██    ████ ░░░░
//         [0b0000_0000, 0b0000_1111, 0b1111_0111, 0b1111_1111, 0b0111_1110, 0b0000_0011, 0b1111_0000, 0b0000_0000],
//         [0; 8],  //     ░░░░ ░░██    ██░░ ░░░█    ██░░ ░███    ░░░█ ████    ░░░░ ░███    ██░░ ░░░░
//         [0b0000_0000, 0b0000_0011, 0b1100_0001, 0b1100_0111, 0b0001_1111, 0b0000_0111, 0b1100_0000, 0b0000_0000],
//         [0; 8],  //     ░░░░ ░░██    ██░░ ░░░█    ████ ████    ░░░█ ████    ██░█ ████    ██░░ ░░░░
//         [0b0000_0000, 0b0000_0011, 0b1100_0001, 0b1111_1111, 0b0001_1111, 0b1101_1111, 0b1100_0000, 0b0000_0000],
//         [0; 8],  //     ░░░░ ░░██    ██░░ ░░░█    ████ ████    ░░░█ ██░█    ████ ██░█    ██░░ ░░░░
//         [0b0000_0000, 0b0000_0011, 0b1100_0001, 0b1111_1111, 0b0001_1101, 0b1111_1101, 0b1100_0000, 0b0000_0000],
//         [0; 8],  //     ░░░░ ░░██    ██░░ ░░░█    ██░░ ░███    ░░░█ ██░░    ████ █░░█    ██░░ ░░░░
//         [0b0000_0000, 0b0000_0011, 0b1100_0001, 0b1100_0111, 0b0001_1100, 0b1111_1001, 0b1100_0000, 0b0000_0000],
//         [0; 8],  //     ░░░░ ████    ████ ░███    ████ ████    ░███ ███░    ░███ ░░██    ████ ░░░░
//         [0b0000_0000, 0b0000_1111, 0b1111_0111, 0b1111_1111, 0b0111_1110, 0b0111_0011, 0b1111_0000, 0b0000_0000],
//         [0; 8],  //     ░░░░ ████    ████ ░███    ████ ██░░    ░███ ███░    ░░█░ ░░██    ████ ░░░░
//         [0b0000_0000, 0b0000_1111, 0b1111_0111, 0b1111_1100, 0b0111_1110, 0b0010_0011, 0b1111_0000, 0b0000_0000],
//         [0; 8],
//         [0; 8],
//         [0; 8],
//         [0; 8],
//         [0; 8],
//         [0; 8],
//         [0; 8],
//         [0; 8],
//         [0; 8],
//     ];
// 	assert_eq!(machine.video_buffer, expected_buffer);
// }
