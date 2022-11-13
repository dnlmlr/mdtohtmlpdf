fn main() {
    println!("cargo:rerun-if-changed=./res/");

    const HTML_TEMPLATE: &str = include_str!("res/template.mustache");
    const GITHUB_STYLE_CSS: &str = include_str!("res/github-markdown-light.min.css");
    const KATEX_CSS: &str = include_str!("res/katex.min.css");

    let mut katex_css = KATEX_CSS.to_string();

    // So this whole this is insanely scuffed. In order to render maths symbols the KaTeX library
    // is utilized which has its own CSS file and a bunch of fonts. This code converts all the
    // fonts into data urls which is then inserted inline into the CSS file.
    for entry in std::fs::read_dir("res/katex-fonts").unwrap() {
        let entry = entry.unwrap().path();
        let font_name = entry.file_name().unwrap().to_string_lossy();
        let font_data = std::fs::read(&entry).unwrap();
        let font_b64 = base64::encode(&font_data);

        let data_url = format!("url(data:font/woff2;base64,{})", font_b64);
        let orig_url = format!("url(fonts/{})", font_name);

        katex_css = katex_css.replace(&orig_url, &data_url);
    }

    // Precombine the html template
    let merged = HTML_TEMPLATE.replace(
        "{{STYLE}}",
        &format!("{}\n{}\n", GITHUB_STYLE_CSS, katex_css),
    );

    std::fs::write(
        format!("{}/template.mustache", std::env::var("OUT_DIR").unwrap()),
        merged,
    )
    .unwrap();
}
