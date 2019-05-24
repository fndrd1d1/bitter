pub fn convert_to_number(input: String) -> u64 {
    let str_val = input.trim().to_lowercase().replace(" ", "");
    let val:u64;
    if str_val.starts_with("0x") {
        let len = str_val[2..].len();
        if len > 16 {
            panic!("integer size > 64 is not supported")
        }
        val = u64::from_str_radix(&str_val[2..], 16)
            .expect("invalid input");
    }
    else
    {
        val = u64::from_str_radix(str_val.as_str(), 10)
            .expect("invalid input");
    }
    return val;
}