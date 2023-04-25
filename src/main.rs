use std::collections::HashMap;

extern crate lazy_static;

use lazy_static::lazy_static;

struct BookAbbreviation;
impl BookAbbreviation {
    pub fn get_by_book_id_and_text(bookId: &BookId, text: TextId) -> Option<&'static str> {
        match text {
            TextId::EnLSB => BOOK_ABBREVIATIONS_FOR_EN_LSB.get(bookId).copied(),
            TextId::FiR1933_38 => BOOK_ABBREVIATIONS_FOR_FI_R1933_38.get(bookId).copied(),
            _ => None,
        }
    }
    pub fn sanitize(value: &str) -> String {
        value.replace(".", "").to_lowercase()
    }
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
#[repr(u8)]
enum BookId {
    Matthew,
    John,
}
impl BookId {
    pub fn find_by_sanitized_abbreviation<'a>(
        text: &TextId,
        abbreviation: &String,
    ) -> Option<&'a BookId> {
        match text {
            TextId::EnLSB => BOOK_ABBREVIATIONS_TO_IDS_EN.get(abbreviation.as_str()),
            TextId::FiR1933_38 => BOOK_ABBREVIATIONS_TO_IDS_FI.get(abbreviation.as_str()),
        }
    }
}

lazy_static! {
    static ref BOOK_ABBREVIATIONS_FOR_EN_LSB: HashMap<BookId, &'static str> =
        HashMap::from([(BookId::Matthew, "Matthew"), (BookId::John, "John"),]);
    static ref BOOK_ABBREVIATIONS_FOR_FI_R1933_38: HashMap<BookId, &'static str> =
        HashMap::from([(BookId::Matthew, "Matt."), (BookId::John, "Joh.")]);
    static ref BOOK_ABBREVIATIONS_TO_IDS_EN: HashMap<&'static str, BookId> = HashMap::from([
        ("matt", BookId::Matthew),
        ("matthew", BookId::Matthew),
        ("john", BookId::John),
    ]);
    static ref BOOK_ABBREVIATIONS_TO_IDS_FI: HashMap<&'static str, BookId> = HashMap::from([
        ("matt", BookId::Matthew),
        ("matteus", BookId::Matthew),
        ("joh", BookId::John),
        ("johannes", BookId::John),
    ]);
}

struct Reference {
    book_id: BookId,
    chapter: u8,
}
impl Reference {
    pub fn to_string_by_text(&self, text: TextId) -> String {
        format!(
            "{} {}",
            BookAbbreviation::get_by_book_id_and_text(&self.book_id, text).unwrap_or("undefined"),
            self.chapter
        )
    }
}

fn parse_reference_by_text<S>(reference: S, text: &TextId) -> Option<Reference>
where
    S: Into<String>,
{
    let r: String = reference.into();
    let parts = r.trim().split(" ").collect::<Vec<_>>();
    match parts.len() {
        2 => {
            let part_as_sanitized_book_abbreviation = BookAbbreviation::sanitize(parts[0]);
            let Some(book_id) =
                BookId::find_by_sanitized_abbreviation(text, &part_as_sanitized_book_abbreviation) else {
                    return None;
                };

            let Ok(chapter) = parts[1].parse::<u8>() else {
                return None;
            };

            Some(Reference {
                book_id: book_id.clone(),
                chapter,
            })
        }
        _ => None,
    }
}
fn parse_references_by_text<S>(reference: S, text: &TextId) -> Vec<Option<Reference>>
where
    S: Into<String>,
{
    let s: String = reference.into();
    s.split(";")
        .map(|part| parse_reference_by_text(part, text))
        .collect::<Vec<_>>()
}

fn main() {
    println!("Hello, world!");
}

#[derive(Clone, Debug)]
enum TextId {
    EnLSB,
    FiR1933_38,
}

#[cfg(test)]
mod tests {
    use crate::{parse_reference_by_text, parse_references_by_text, BookId, TextId};

    #[test]
    fn convert_valid_reference_from_one_text_to_another() {
        let reference = parse_reference_by_text("Joh 1", &TextId::FiR1933_38).unwrap();
        let result = reference.to_string_by_text(TextId::EnLSB);
        assert_eq!(result, "John 1");
    }
    #[test]
    fn fail_parse_reference_with_book_and_chapter_when_reference_is_incorrect() {
        let text = TextId::FiR1933_38;

        macro_rules! test_book_and_chapter {
            ($reference: literal) => {
                let reference = parse_reference_by_text($reference, &text);
                assert!(reference.is_none());
            };
        }

        test_book_and_chapter!("1");
        test_book_and_chapter!("Nothing");
        test_book_and_chapter!("Matt");
        test_book_and_chapter!("Mat. 1");
    }
    #[test]
    fn parse_multiple_references_when_references_are_correct() {
        let references = parse_references_by_text("Matt 1; Joh. 1", &TextId::FiR1933_38);

        assert_eq!(references[0].as_ref().unwrap().book_id, BookId::Matthew);
        assert_eq!(references[0].as_ref().unwrap().chapter, 1);

        assert_eq!(references[1].as_ref().unwrap().book_id, BookId::John);
        assert_eq!(references[1].as_ref().unwrap().chapter, 1);
    }
    #[test]
    fn parse_reference_with_book_and_chapter_when_reference_is_correct() {
        let text = TextId::FiR1933_38;

        macro_rules! test_book_and_chapter {
            ($reference: literal, $bookId: ident, $chapter:literal) => {
                let reference = parse_reference_by_text($reference, &text).unwrap();
                assert_eq!(reference.book_id, BookId::$bookId);
                assert_eq!(reference.chapter, $chapter);
            };
        }

        test_book_and_chapter!("matt 1", Matthew, 1);
        test_book_and_chapter!("Matt. 1", Matthew, 1);
        test_book_and_chapter!("Matt. 10", Matthew, 10);
        test_book_and_chapter!("Joh. 1", John, 1);
    }
}
