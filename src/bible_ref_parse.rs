use std::{env, fs::File};

use bible_ref::{OsisSource, Source};
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

    let osis_source_path = env::current_dir().unwrap().join("assets/kjv.xml");
    let file = File::open(&osis_source_path).unwrap();
    let osis_source = OsisSource::from_file(file);

    for parsed_reference in parse_result.iter() {
        let references = osis_source.find_content(parsed_reference)?;

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
