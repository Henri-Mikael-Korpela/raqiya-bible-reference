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
    println!("{:?}", parse_result);
    Ok(())
}
