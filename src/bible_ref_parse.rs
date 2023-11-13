use std::{env, fs::File, io::BufReader};

use raqiya_bible_reference as bible_ref;

fn main() {
    if let Err(err) = run() {
        eprintln!("{}", err);
        std::process::exit(1);
    }
}
fn run() -> Result<(), String> {
    let args = std::env::args();
    let bible_ref = &args
        .skip(1)
        .next()
        .ok_or_else(|| "No Bible reference as command argument #1 given.")?;
    let parse_result = bible_ref::parse_references(bible_ref)
        .map_err(|err| err.to_string(bible_ref::Locale::En))?;

    let project_dir = env::current_dir().unwrap();
    let xml_file_path = project_dir.join("assets/kjv.xml");

    for parsed_reference in parse_result.iter() {
        let file = File::open(&xml_file_path).unwrap();
        let mut file_reader = BufReader::new(file);
        let references = bible_ref::find_content_in_source(&mut file_reader, parsed_reference)?;

        for reference in references {
            println!(
                "{} {}:{} {}",
                parsed_reference.book_name,
                parsed_reference.chapter,
                reference.number,
                reference.content
            );
        }
    }

    Ok(())
}
