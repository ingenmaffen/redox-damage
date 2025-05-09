use super::instructions::utils;
use super::memory::Memory;

pub fn boot_sequence(memory: &mut Memory) {
    unpack_and_load_logo(memory);
    setup_tile_data(memory);
}

fn setup_tile_data(memory: &mut Memory) {
    let mut tile: u8 = 0x01;
    for i in 0x9904..=0x990F {
        utils::write_byte_to_memory(memory, i, tile);
        tile += 1;
    }
    for i in 0x9924..=0x992F {
        utils::write_byte_to_memory(memory, i, tile);
        tile += 1;
    }
}

fn unpack_and_load_logo(memory: &mut Memory) {
    let mut memory_index: usize = 0x8010;
    for i in 0x0104..=0x0133 {
        let byte = utils::read_byte_from_memory(memory, i);
        let upper = (byte & 0b11110000) >> 4;
        let lower = byte & 0b00001111;
        load_to_vram(memory, upper, memory_index);
        memory_index += 4;
        load_to_vram(memory, lower, memory_index);
        memory_index += 4;
    }
}

fn load_to_vram(memory: &mut Memory, value: u8, index: usize) {
    let unpacked = get_current_value_unpacked(value);
    let mut memory_index = index;
    for _ in 0..2 {
        utils::write_byte_to_memory(memory, memory_index, unpacked);
        memory_index += 1;
        utils::write_byte_to_memory(memory, memory_index, 0x00);
        memory_index += 1;
    }
}

fn get_current_value_unpacked(byte: u8) -> u8 {
    let mut value: u8 = 0x00;
    if byte & 0b1000 > 0 {
        value = value | 0b11000000;
    }
    if byte & 0b0100 > 0 {
        value = value | 0b00110000;
    }
    if byte & 0b0010 > 0 {
        value = value | 0b00001100;
    }
    if byte & 0b0001 > 0 {
        value = value | 0b00000011;
    }
    return value;
}
