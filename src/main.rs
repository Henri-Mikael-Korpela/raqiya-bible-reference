use std::collections::HashMap;

extern crate lazy_static;

use lazy_static::lazy_static;

#[derive(Clone, Debug, Eq, PartialEq)]
#[repr(u8)]
enum BookId {
    Matthew,
    John,
}

lazy_static! {
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

fn find_book_id_by_sanitized_abbreviation<'a>(
    text: &TextId,
    abbreviation: &String,
) -> Option<&'a BookId> {
    match text {
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
    FiR1933_38,
}

#[cfg(test)]
mod tests {
    use crate::{parse_by_text, BookId, TextId};

    #[test]
    fn fail_parse_fi_1933_38_reference_with_book_and_chapter_when_reference_is_incorrect() {
        let language = TextId::FiR1933_38;

        macro_rules! test_book_and_chapter {
            ($reference: literal) => {
                let result = parse_by_text(&language, $reference);
                assert!(result.is_none());
            };
        }

        test_book_and_chapter!("Nothing");
        test_book_and_chapter!("Matt");
        test_book_and_chapter!("Mat. 1");
    }
    #[test]
    fn parse_fi_1933_38_reference_with_book_and_chapter_when_reference_is_correct() {
        let language = TextId::FiR1933_38;

        macro_rules! test_book_and_chapter {
            ($reference: literal, $bookId: ident, $chapter:literal) => {
                let result = parse_by_text(&language, $reference).unwrap();
                assert_eq!(result[0].book_id, BookId::$bookId);
                assert_eq!(result[0].chapter, $chapter);
            };
        }

        test_book_and_chapter!("matt 1", Matthew, 1);
        test_book_and_chapter!("Matt. 1", Matthew, 1);
        test_book_and_chapter!("Matt. 10", Matthew, 10);
        test_book_and_chapter!("Joh. 1", John, 1);
    }
}
