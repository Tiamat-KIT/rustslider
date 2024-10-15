use std::{fs::File, path::Path,io::Write};

use pulldown_cmark::Options;


fn parse(path: &std::path::Path) -> String {
    use std::fs;
    let parsed = fs::read_to_string(path.to_str().unwrap())
        .expect("Failed to read file");
    parsed
}
fn render(content: String,output_path: &std::path::Path) -> Result<(),Box<dyn std::error::Error>> {
    let mut file = File::create(output_path).unwrap();
    write!(file, "{}", content);
    Ok(())
}

fn main() {

    let markdown = parse(Path::new("./index.md"));

    let mut options = pulldown_cmark::Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    let parser = pulldown_cmark::Parser::new_ext(&markdown, options);

    let mut output = String::new();
    pulldown_cmark::html::push_html(&mut output, parser);

    render(output, Path::new("./output.html")).unwrap();
}