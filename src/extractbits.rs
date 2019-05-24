use std::vec;

pub fn extract_active_bits(value: u32) -> vec::Vec<u8> {
    let mut bit_vec: vec::Vec<u8> = Vec::new();

    for i in 0..32 {
        if (value & (1 << i)) > 0 {
            bit_vec.push(i);
        }
    }

    return bit_vec;
}