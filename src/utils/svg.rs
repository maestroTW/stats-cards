// lines - 1
const MAX_LINES: usize = 2;

pub fn wrap_text(text: &String) -> String {
    let lines = textwrap::wrap(text, 60);
    let wrapped_text: Vec<String> = lines
        .iter()
        .enumerate()
        .filter(|(idx, _)| idx <= &MAX_LINES)
        .map(|(idx, line)| {
            let mut text = line.to_string();
            if idx == MAX_LINES {
                text += "...";
            }

            // add escape html
            format!(r##"<tspan dy="1.25em" x="0">{text}</tspan>"##)
        })
        .collect();

    wrapped_text.join("\n")
}
