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
    let mut args = std::env::args();
    args.next();
    let Some(text) = args.next() else {
        return Err("No text as command argument #1 given.".into());
    };
    let Some(bible_ref) = args
        .next() else {
            return Err("No Bible reference as command argument #2 given.".into());
        };
    let parse_result = bible_ref::parse_references(&bible_ref)
        .map_err(|err| err.to_string(bible_ref::Locale::En))?;

    let osis_source_target = match text.as_str() {
        "KJV" => "assets/kjv.xml",
        "R1933/-38" => "assets/r1933-38.xml",
        _ => return Err(format!("Unsupported text: {}", text)),
    };
    let osis_source_path = env::current_dir().unwrap().join(osis_source_target);
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
