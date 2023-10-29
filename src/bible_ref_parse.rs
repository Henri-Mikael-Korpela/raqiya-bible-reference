use raqiya_bible_reference::parse;
use raqiya_bible_reference::Locale;

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
    let parse_result = parse(bible_ref).map_err(|err| err.to_string(Locale::English))?;
    println!("{:?}", parse_result);
    Ok(())
}
