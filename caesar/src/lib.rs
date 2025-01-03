// based on https://github.com/TheAlgorithms/Rust/blob/master/src/ciphers/caesar.rs
use harfbuzz_wasm::{debug, Font, Glyph, GlyphBuffer};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn shape(
    _shape_plan: u32,
    font_ref: u32,
    buf_ref: u32,
    _features: u32,
    _num_features: u32,
) -> i32 {
    let font = Font::from_ref(font_ref);
    let mut buffer = GlyphBuffer::from_ref(buf_ref);

    // Get buffer as string
    let buf_u8: Vec<u8> = buffer.glyphs.iter().map(|g| g.codepoint as u8).collect();
    let str_buf = String::from_utf8_lossy(&buf_u8);

    /*
    two things need to happen here: the caesar rotation needs to be stored
    and the "Caesar():" prefix needs to be omitted from the res_str
    */

    let (rotation, cut_str) = find_rotation(&str_buf);

    let res_str: String = format!("{}", cut_str)
        .chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                shift_char(c, rotation)
            } else {
                c
            }
        })
        .collect();
    

    buffer.glyphs = res_str
        .chars()
        .enumerate()
        .map(|(ix, x)| Glyph {
            codepoint: x as u32,
            flags: 0,
            x_advance: 0,
            y_advance: 0,
            cluster: ix as u32,
            x_offset: 0,
            y_offset: 0,
        })
        .collect();

    for item in buffer.glyphs.iter_mut() {
        // Map character to glyph
        item.codepoint = font.get_glyph(item.codepoint, 0);
        // Set advance width
        item.x_advance = font.get_glyph_h_advance(item.codepoint);
    }
    // Buffer is written back to HB on drop
    1
}

fn shift_char(c: char, rotation: isize) -> char {
    let first = if c.is_ascii_lowercase() { b'a' } else { b'A' };
    let rotation = rotation as u8; // Safe cast as rotation is within [0, 25]

    (((c as u8 - first) + rotation) % 26 + first) as char
}

fn find_rotation(str_buf: &str) -> (isize, &str) {
    if let Some((rotation_part, text)) = str_buf.split_once(':') { // ["Caesar(1):", "Hello World!"]
        if rotation_part.starts_with("Caesar(") && rotation_part.ends_with(")") {
            let rotation_str = &rotation_part[7..rotation_part.len() - 1];
            if let Ok(rotation) = rotation_str.parse::<isize>() {
                return (rotation, text.trim());
            }
        }
    }
    // default
    (0, str_buf)
}