use fontdue::{Font, FontSettings};

const FONT_PATH: &[u8] = include_bytes!("../../fonts/segoeui.ttf") as &[u8];

// lines - 1
// const MAX_LINES: usize = 2;

// pub fn wrap_text(text: &String) -> String {
//     let lines = textwrap::wrap(text, 60);
//     let wrapped_text: Vec<String> = lines
//         .iter()
//         .enumerate()
//         .filter(|(idx, _)| idx <= &MAX_LINES)
//         .map(|(idx, line)| {
//             let mut text = line.to_string();
//             if idx == MAX_LINES {
//                 text += "...";
//             }

//             // add escape html
//             format!(r##"<tspan dy="1.25em" x="0">{text}</tspan>"##)
//         })
//         .collect();
//     wrapped_text.join("\n")
// }

pub fn calc_width(text: &str, font_size: f32) -> usize {
    let font = Font::from_bytes(FONT_PATH, FontSettings::default()).unwrap();
    let mut width = 0.0;
    for c in text.chars() {
        let (metrics, _) = font.rasterize(c, font_size);
        width += metrics.advance_width;
    }

    width.ceil() as usize
}
