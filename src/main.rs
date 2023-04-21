use std::collections::HashMap;

extern crate lazy_static;

use lazy_static::lazy_static;

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
#[repr(u8)]
enum BookId {
    Matthew,
    John,
}
impl BookId {
    pub fn get_book_abbreviation_by_text(&self, text: TextId) -> Option<&'static str> {
        match text {
            TextId::EnLSB => BOOK_ABBREVIATIONS_FOR_EN_LSB.get(&self).copied(),
            _ => None,
        }
    }
}

lazy_static! {
    static ref BOOK_ABBREVIATIONS_FOR_EN_LSB: HashMap<BookId, &'static str> =
        HashMap::from([(BookId::Matthew, "Matthew"), (BookId::John, "John"),]);
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
            self.book_id
                .get_book_abbreviation_by_text(text)
                .unwrap_or("undefined"),
            self.chapter
        )
    }
}

fn find_book_id_by_sanitized_abbreviation<'a>(
    text: &TextId,
    abbreviation: &String,
) -> Option<&'a BookId> {
    match text {
        TextId::EnLSB => BOOK_ABBREVIATIONS_TO_IDS_EN.get(abbreviation.as_str()),
        TextId::FiR1933_38 => BOOK_ABBREVIATIONS_TO_IDS_FI.get(abbreviation.as_str()),
    }
}
fn parse_by_text<'a, S>(text: &TextId, reference: S) -> Option<Vec<Reference>>
where
    S: Into<String>,
{
    let r: String = reference.into();
    let parts = r.split(" ").collect::<Vec<_>>();
    match parts.len() {
        2 => {
            let part_as_sanitized_book_abbreviation = sanitize_as_book_abbreviation(parts[0]);
            let Some(book_id) =
                find_book_id_by_sanitized_abbreviation(text, &part_as_sanitized_book_abbreviation) else {
                    return None;
                };

            let Ok(chapter) = parts[1].parse::<u8>() else {
                return None;
            };

            Some(vec![Reference {
                book_id: book_id.clone(),
                chapter,
            }])
        }
        _ => None,
    }
}
fn sanitize_as_book_abbreviation(value: &str) -> String {
    value.replace(".", "").to_lowercase()
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
    use crate::{parse_by_text, BookId, TextId};

    #[test]
    fn convert_reference_from_one_text_to_another() {
        let reference = parse_by_text(&TextId::FiR1933_38, "Joh 1").unwrap();
        let result = reference[0].to_string_by_text(TextId::EnLSB);
        assert_eq!(result, "John 1");
    }
    #[test]
    fn fail_parse_fi_1933_38_reference_with_book_and_chapter_when_reference_is_incorrect() {
        let text = TextId::FiR1933_38;

        macro_rules! test_book_and_chapter {
            ($reference: literal) => {
                let references = parse_by_text(&text, $reference);
                assert!(references.is_none());
            };
        }

        test_book_and_chapter!("1");
        test_book_and_chapter!("Nothing");
        test_book_and_chapter!("Matt");
        test_book_and_chapter!("Mat. 1");
    }
    #[test]
    fn parse_fi_1933_38_reference_with_book_and_chapter_when_reference_is_correct() {
        let text = TextId::FiR1933_38;

        macro_rules! test_book_and_chapter {
            ($reference: literal, $bookId: ident, $chapter:literal) => {
                let references = parse_by_text(&text, $reference).unwrap();
                assert_eq!(references.len(), 1);
                assert_eq!(references[0].book_id, BookId::$bookId);
                assert_eq!(references[0].chapter, $chapter);
            };
        }

        test_book_and_chapter!("matt 1", Matthew, 1);
        test_book_and_chapter!("Matt. 1", Matthew, 1);
        test_book_and_chapter!("Matt. 10", Matthew, 10);
        test_book_and_chapter!("Joh. 1", John, 1);
    }
}
