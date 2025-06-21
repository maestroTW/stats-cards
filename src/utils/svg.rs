use fontdue::{Font, FontSettings};

const FONT_PATH: &[u8] = include_bytes!("../../fonts/segoeui.ttf") as &[u8];
const MAX_LINES: usize = 2;

pub fn wrap_text(text: &String, font_size: f32, max_width: usize) -> Vec<String> {
    let mut lines = Vec::new();
    let mut current_line = String::new();
    let mut words = text.split_whitespace().peekable();
    let mut truncated = false;

    while let Some(word) = words.next() {
        let next_line = if current_line.is_empty() {
            word.to_string()
        } else {
            format!("{} {}", current_line, word)
        };

        if calc_width(&next_line, font_size) <= max_width {
            current_line = next_line;
        } else {
            lines.push(current_line);
            current_line = word.to_string();

            if lines.len() == MAX_LINES - 1 {
                while let Some(next_word) = words.next() {
                    let test = format!("{} {}", current_line, next_word);
                    if calc_width(&test, font_size) <= max_width {
                        current_line = test;
                    } else {
                        truncated = true;
                        break;
                    }
                }

                if truncated {
                    current_line.push_str("...");
                }

                lines.push(current_line);
                return lines;
            }
        }
    }

    if !current_line.is_empty() && lines.len() < MAX_LINES {
        lines.push(current_line);
    }

    lines
}

pub fn calc_width(text: &str, font_size: f32) -> usize {
    let font = Font::from_bytes(FONT_PATH, FontSettings::default()).unwrap();
    let mut width = 0.0;
    for c in text.chars() {
        let (metrics, _) = font.rasterize(c, font_size);
        width += metrics.advance_width;
    }

    width.ceil() as usize
}
