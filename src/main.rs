mod bible;

use bible::TextId;

fn main() {
    let text = &TextId::FiR1933_38;
    let reference = bible::parse_reference_by_text("Joh. 1:15", text);

    if let Some(reference) = reference {
        println!(
            "Reference: {} {}{}",
            reference.get_book_abbreviation(text),
            reference.get_chapter(),
            match reference.get_number() {
                Some(number) => format!(":{}", number),
                None => String::new(),
            }
        );
        println!("Reference: {}", reference.to_string(TextId::EnLSB));
    } else {
        eprintln!("Referenced given not found.");
    }

    println!(
        "{:?}",
        bible::find_reference_matches_in("Test Joh 1 (Matt. 15:10)", &TextId::FiR1933_38)
    );
}
