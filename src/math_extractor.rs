/// Extract the contents between a pair of `$` or `$$`, depending on the `block_mode` variable.
/// The input is expected to start behind the initial `$` sign. If block mode is set to `true`,
/// this function will expect `$$` as the terminator, otherwise it will expect `$`.
/// The `$` can be escaped with `\$`, in which case it is not used as a delimiter.
///
/// If a closing html tag is detected, or the input ends, `None` is returned. Otherwise the
/// contents are returned, excluding the closing `$` / `$$`.
fn extract_tex_math<'a>(text: &'a [u8], block_mode: bool) -> Option<&'a [u8]> {
    let mut i = 0;
    loop {
        match (text.get(i), text.get(i + 1)) {
            // Detected end of maths text
            (Some(b'$'), Some(b'$')) if block_mode => break,
            (Some(b'$'), _) if !block_mode => break,

            // Skip escaped $
            (Some(b'\\'), Some(b'$')) => i += 1,

            // This is kinda scuffed. Since the parser is run against the html text, "</" denotes
            // the end of a currently open html tag. This will be treated as an unfinished scope.
            // So there might be a case where `</` occurs in a maths equation, but until then I
            // guess this will stay
            (Some(b'<'), Some(b'/')) => return None,

            (None, _) => return None,
            (_, None) if block_mode => return None,

            _ => (),
        }
        i += 1;
    }

    Some(&text[..i])
}

fn katex_render(input: &str, block_mode: bool) -> Result<String, katex::Error> {
    let opts = katex::Opts::builder()
        .display_mode(block_mode)
        .build()
        .unwrap();
    katex::render_with_opts(&input, &opts)
}

pub fn katex_replace(input: &str) -> String {
    let bytes = input.as_bytes();

    let mut output = Vec::with_capacity(bytes.len());

    let mut i = 0;
    while i < bytes.len() {
        match (bytes[i], bytes.get(i + 1)) {
            // Escaped $
            (b'\\', Some(b'$')) => {
                output.push(b'$');
                i += 2;
            }
            // A normal $ results in trying to parse a math scope
            (b'$', next_char) => {
                // If it is $$, parse as a display block
                let block_mode = matches!(next_char, Some(b'$'));
                // For $$, 2 chars need to be skipped, for $ only 1
                let offset = block_mode.then(|| 2).unwrap_or(1);

                let Some(formula) = extract_tex_math(
                    &bytes[i + offset..],
                    block_mode
                ) else {
                    // If the extraction failed, just ignore it and add the characters
                    output.push(b'$');
                    if block_mode {
                        output.push(b'$');
                    }
                    i += offset;

                    // Todo: Maybe print a warning about an unterminated maths scope
                    continue;
                };

                // Skip over the start and end delimiters
                i += formula.len() + offset * 2;

                // Render the formula as html
                let rendered_html =
                    match katex_render(&String::from_utf8_lossy(formula), block_mode) {
                        Ok(rendered_html) => rendered_html,
                        Err(e) => {
                            eprintln!("{}", e);
                            continue;
                        }
                    };

                output.extend(rendered_html.as_bytes());
            }
            (c, _) => {
                output.push(c);
                i += 1
            }
        }
    }

    String::from_utf8_lossy(&output).to_string()
}
