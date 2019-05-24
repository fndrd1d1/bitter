pub fn conver_to_u32(input: String) -> u32 {
    let str_val = input.trim().to_lowercase();
    let val:u32;
    if str_val.starts_with("0x") {
        val = u32::from_str_radix(&str_val[2..], 16)
            .expect("invalid input");
    }
    else
    {
        val = u32::from_str_radix(str_val.as_str(), 10)
            .expect("invalid input");
    }
    return val;
}