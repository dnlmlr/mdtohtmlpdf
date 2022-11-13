mod cli;
mod math_extractor;

use std::{env::temp_dir, path::Path};

use clap::Parser;
use cli::CliArgs;
use comrak::{
    nodes::NodeValue, plugins::syntect::SyntectAdapter, Arena, ComrakOptions, ComrakPlugins,
};
use headless_chrome::protocol::page::PrintToPdfOptions;

use math_extractor::katex_replace;

const HTML_TEMPLATE: &str = include_str!(concat!(env!("OUT_DIR"), "/template.mustache"));

fn setup_comrak_opts() -> ComrakOptions {
    let mut opts = ComrakOptions::default();
    opts.parse.smart = true;
    opts.extension.autolink = true;
    opts.extension.description_lists = true;
    opts.extension.footnotes = true;
    opts.extension.front_matter_delimiter = Some("---".to_string());
    opts.extension.strikethrough = true;
    opts.extension.superscript = false;
    opts.extension.table = true;
    opts.extension.tagfilter = true;
    opts.extension.tasklist = true;

    opts
}

fn comrak_convert_to_html(markdown: &str) -> String {
    let opts = setup_comrak_opts();

    let mut plugins = ComrakPlugins::default();
    let syntax_highlighter = SyntectAdapter::new("InspiredGitHub");
    plugins.render.codefence_syntax_highlighter = Some(&syntax_highlighter);

    let arena = Arena::new();

    let markdown_ast = comrak::parse_document(&arena, &markdown, &opts);

    for node_edge in markdown_ast.traverse() {
        let node = match node_edge {
            comrak::arena_tree::NodeEdge::Start(start) => start,
            _ => continue,
        };

        match &mut node.data.borrow_mut().value {
            // Add an escape to $ in codeblocks. This will prevent the custom parser from parsing
            // them as maths scopes
            NodeValue::Code(code) => {
                let text = String::from_utf8_lossy(&code.literal);
                code.literal = text.replace('$', "\\$").as_bytes().to_vec();
            }
            _ => (),
        }
    }

    let mut html_out = Vec::new();
    comrak::format_html_with_plugins(markdown_ast, &opts, &mut html_out, &plugins).unwrap();
    String::from_utf8_lossy(&html_out).to_string()
}

fn chrome_render_to_pdf(
    html_file: impl AsRef<Path>,
    print_options: Option<PrintToPdfOptions>,
) -> Vec<u8> {
    let html_file = html_file.as_ref().canonicalize().unwrap();
    let html_file = html_file.as_os_str().to_string_lossy();

    let browser = headless_chrome::Browser::default().unwrap();

    let tab = browser.wait_for_initial_tab().unwrap();
    let tab = tab
        .navigate_to(&format!("file://{}", html_file))
        .unwrap()
        .wait_until_navigated()
        .unwrap();

    tab.print_to_pdf(print_options).unwrap()
}

fn default_print_to_pdf_options() -> PrintToPdfOptions {
    PrintToPdfOptions {
        landscape: Some(false),
        display_header_footer: Some(false),
        print_background: Some(true),
        scale: None,
        // Din A4
        paper_width: Some(8.27),
        paper_height: Some(11.7),
        margin_top: None,
        margin_bottom: None,
        margin_left: None,
        margin_right: None,
        page_ranges: None,
        ignore_invalid_page_ranges: None,
        header_template: None,
        footer_template: None,
        prefer_css_page_size: Some(true),
    }
}

fn main() {
    let cli_args = CliArgs::parse();

    let md_in = std::fs::read_to_string(&cli_args.input).unwrap();

    // comrak seems to replace `\$` by `$`, so double up the backslash
    let md_in = md_in.replace("\\$", "\\\\$");

    // Conver the actual markdown to HTML
    let html_out = comrak_convert_to_html(&md_in);

    // Run the custom math scope parser together with KaTeX to render maths symbols
    let html_out = katex_replace(&html_out);

    // Insert the rendered HTML into the HTML Template
    let html_out = HTML_TEMPLATE.replace("{{RENDERED_MARKDOWN}}", &html_out);

    match (cli_args.html_out, cli_args.pdf_out) {
        (Some(html_file), Some(pdf_file)) => {
            std::fs::write(&html_file, &html_out).unwrap();

            let pdf = chrome_render_to_pdf(&html_file, Some(default_print_to_pdf_options()));
            std::fs::write(&pdf_file, &pdf).unwrap();
        }
        (Some(html_file), _) => {
            std::fs::write(&html_file, &html_out).unwrap();
        }
        (_, Some(pdf_file)) => {
            let tmp_html = temp_dir().join(format!("temp-html-pdf-{}.html", rand::random::<u64>()));
            std::fs::write(&tmp_html, &html_out).unwrap();

            let pdf = chrome_render_to_pdf(&tmp_html, Some(default_print_to_pdf_options()));
            std::fs::write(&pdf_file, &pdf).unwrap();
            std::fs::remove_file(&tmp_html).unwrap();
        }
        (_, _) => {
            println!("No output specified");
        }
    }
}
