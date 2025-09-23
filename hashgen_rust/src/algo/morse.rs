// src/algo/morse.rs

/// encode input bytes into ITU-R M.1677-1 Morse code
pub fn encode_morse(input: &[u8]) -> Vec<u8> {
    fn morse_map(b: u8) -> Option<&'static str> {
        match b {
            b'a' | b'A' => Some(".-"),
            b'b' | b'B' => Some("-..."),
            b'c' | b'C' => Some("-.-."),
            b'd' | b'D' => Some("-.."),
            b'e' | b'E' => Some("."),
            b'f' | b'F' => Some("..-."),
            b'g' | b'G' => Some("--."),
            b'h' | b'H' => Some("...."),
            b'i' | b'I' => Some(".."),
            b'j' | b'J' => Some(".---"),
            b'k' | b'K' => Some("-.-"),
            b'l' | b'L' => Some(".-.."),
            b'm' | b'M' => Some("--"),
            b'n' | b'N' => Some("-."),
            b'o' | b'O' => Some("---"),
            b'p' | b'P' => Some(".--."),
            b'q' | b'Q' => Some("--.-"),
            b'r' | b'R' => Some(".-."),
            b's' | b'S' => Some("..."),
            b't' | b'T' => Some("-"),
            b'u' | b'U' => Some("..-"),
            b'v' | b'V' => Some("...-"),
            b'w' | b'W' => Some(".--"),
            b'x' | b'X' => Some("-..-"),
            b'y' | b'Y' => Some("-.--"),
            b'z' | b'Z' => Some("--.."),
            b'0' => Some("-----"),
            b'1' => Some(".----"),
            b'2' => Some("..---"),
            b'3' => Some("...--"),
            b'4' => Some("....-"),
            b'5' => Some("....."),
            b'6' => Some("-...."),
            b'7' => Some("--..."),
            b'8' => Some("---.."),
            b'9' => Some("----."),
            b'.' => Some(".-.-.-"),
            b',' => Some("--..--"),
            b'?' => Some("..--.."),
            b'\'' => Some(".----."),
            b'!' => Some("-.-.--"),
            b'/' => Some("-..-."),
            b'(' => Some("-.--."),
            b')' => Some("-.--.-"),
            b'&' => Some(".-..."),
            b':' => Some("---..."),
            b';' => Some("-.-.-."),
            b'=' => Some("-...-"),
            b'+' => Some(".-.-."),
            b'-' => Some("-....-"),
            b'_' => Some("..--.-"),
            b'"' => Some(".-..-."),
            b'$' => Some("...-..-"),
            b'@' => Some(".--.-."),
            b' ' => Some(" "),
            _ => None,
        }
    }

    let mut out = Vec::with_capacity(input.len() * 6);
    for &c in input {
        if let Some(code) = morse_map(c) {
            out.extend_from_slice(code.as_bytes());
            out.push(b' ');
        }
    }
    if out.last() == Some(&b' ') {
        out.pop();
    }
    out
}
