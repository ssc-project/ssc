pub mod extract_svelte_ignore;
pub mod html;

pub fn full_char_code_at(str: &str, pos: usize) -> u32 {
    let code = str.as_bytes()[pos] as u32;
    if code <= 0xd7ff || code >= 0xe000 {
        return code;
    }
    let next = str.as_bytes()[pos + 1] as u32;
    (code << 10) + next - 0x35fdc00
}
