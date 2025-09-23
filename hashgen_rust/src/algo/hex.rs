// src/algo/hex.rs

use hex;

/// `$HEX[]` decoder; returns (decoded, display, hex_err_flag)
pub fn check_for_hex(line: &[u8]) -> (Vec<u8>, Vec<u8>, i32) {
    const PFX: &[u8] = b"$HEX[";
    if line.len() >= PFX.len() && &line[..PFX.len()] == PFX {
        let mut hex_err = 0;
        let end = line.iter().rposition(|&b| b == b']').unwrap_or(line.len());
        if end == line.len() || line.last() != Some(&b']') {
            hex_err = 1;
        }
        let start = line
            .iter()
            .position(|&b| b == b'[')
            .map(|i| i + 1)
            .unwrap_or(5);
        let hex_content = &line[start..end];

        let try_decode = |bytes: &[u8]| -> Option<Vec<u8>> {
            if !bytes.is_empty() && bytes.len() % 2 == 0 {
                let mut dst = vec![0u8; bytes.len() / 2];
                if hex::decode_to_slice(bytes, &mut dst).is_ok() {
                    return Some(dst);
                }
            }
            None
        };

        if let Some(decoded) = try_decode(hex_content) {
            let mut disp = Vec::with_capacity(5 + hex_content.len() + 1);
            disp.extend_from_slice(PFX);
            disp.extend_from_slice(hex_content);
            disp.push(b']');
            return (decoded, disp, hex_err);
        }

        // cleanup
        hex_err = 1;
        let mut clean = Vec::with_capacity(hex_content.len());
        for &c in hex_content {
            let lc = c | 0x20;
            if (b'0'..=b'9').contains(&c) || (b'a'..=b'f').contains(&lc) {
                clean.push(c);
            }
        }
        if clean.len() % 2 == 1 {
            clean.insert(0, b'0');
        }

        if !clean.is_empty() {
            if let Some(decoded) = try_decode(&clean) {
                let mut disp = Vec::with_capacity(5 + hex_content.len() + 1);
                disp.extend_from_slice(PFX);
                disp.extend_from_slice(hex_content);
                disp.push(b']');
                return (decoded, disp, hex_err);
            }
        }

        eprintln!("Error decoding $HEX[] content");
        let mut disp = Vec::with_capacity(5 + hex_content.len() + 1);
        disp.extend_from_slice(PFX);
        disp.extend_from_slice(hex_content);
        disp.push(b']');
        return (line.to_vec(), disp, hex_err);
    }
    (line.to_vec(), line.to_vec(), 0)
}
