use regex::Regex;

pub fn sanitize_input(input: &mut String, is_page: bool) {
    let re = Regex::new(r"^(0x)?[0-9a-fA-F]{2,4}$").unwrap();
    if !re.is_match(input) {
        *input = String::from("0x00");
    }
    let trimmed_input = input.trim_start_matches("0x");
    *input = match is_page {
        true => format!("{:02X}", u8::from_str_radix(&trimmed_input, 16).unwrap()),
        false => format!("{:04X}", u16::from_str_radix(&trimmed_input, 16).unwrap()),
    }
}
