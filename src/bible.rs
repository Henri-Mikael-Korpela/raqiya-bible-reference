use std::collections::HashMap;

extern crate lazy_static;

use lazy_static::lazy_static;

type BookHashMap = HashMap<BookId, (&'static str, u8)>;

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
#[repr(u8)]
pub enum BookId {
    Matthew,
    John,
}
impl BookId {
    fn find_by_sanitized_abbreviation<'a>(
        text: &TextId,
        abbreviation: &String,
    ) -> Option<&'a BookId> {
        match text {
            TextId::EnLSB => BOOK_ABBREVIATIONS_TO_IDS_EN.get(abbreviation.as_str()),
            TextId::FiR1933_38 => BOOK_ABBREVIATIONS_TO_IDS_FI.get(abbreviation.as_str()),
        }
    }
}

struct BookInfo;
impl BookInfo {
    fn get_by_book_id_and_text(book_id: &BookId, text: TextId) -> Option<(&'static str, u8)> {
        match text {
            TextId::EnLSB => BOOK_INFO_FOR_EN_LSB.get(book_id).copied(),
            TextId::FiR1933_38 => BOOK_INFO_FOR_FI_R1933_38.get(book_id).copied(),
        }
    }
    pub fn sanitize(value: &str) -> String {
        value.replace(".", "").to_lowercase()
    }
}

lazy_static! {
    static ref BOOK_INFO_FOR_EN_LSB: BookHashMap = HashMap::from([
        (BookId::Matthew, ("Matthew", 28)),
        (BookId::John, ("John", 21)),
    ]);
    static ref BOOK_INFO_FOR_FI_R1933_38: BookHashMap = HashMap::from([
        (BookId::Matthew, ("Matt.", 28)),
        (BookId::John, ("Joh.", 21))
    ]);
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

#[derive(Debug)]
pub enum Reference {
    BookChapter(BookId, u8),
    BookChapterNumber(BookId, u8, u8),
}
impl Reference {
    /// TODO: Optimize by avoiding match statement and just get data somewhere directly.
    #[inline]
    pub fn get_book_abbreviation(&self, text: &TextId) -> &'static str {
        let book_id = &match self {
            Self::BookChapter(book_id, _) => book_id.clone(),
            Self::BookChapterNumber(book_id, _, _) => book_id.clone(),
        };
        let book_info = match text {
            TextId::EnLSB => BOOK_INFO_FOR_EN_LSB.get(book_id),
            TextId::FiR1933_38 => BOOK_INFO_FOR_FI_R1933_38.get(book_id),
        };

        // Book info should always be found, thus unwrapping is performed here
        book_info.unwrap().0
    }
    /// TODO: Optimize by avoiding match statement and just get data somewhere directly.
    #[inline]
    pub fn get_chapter(&self) -> u8 {
        match self {
            Self::BookChapter(_, chapter) => *chapter,
            Self::BookChapterNumber(_, chapter, _) => *chapter,
        }
    }
    /// TODO: Optimize by avoiding match statement and just get data somewhere directly.
    #[inline]
    pub fn get_number(&self) -> Option<u8> {
        match self {
            Self::BookChapterNumber(_, _, number) => Some(*number),
            _ => None,
        }
    }
    pub fn to_string(&self, text: TextId) -> String {
        static UNDEFINED: &'static str = "undefined";

        match self {
            Self::BookChapter(book_id, chapter) => {
                if let Some((abbreviation, _)) = BookInfo::get_by_book_id_and_text(book_id, text) {
                    format!("{} {}", abbreviation, chapter)
                } else {
                    format!("{} {}", UNDEFINED, chapter)
                }
            }
            Self::BookChapterNumber(book_id, chapter, number) => {
                if let Some((abbreviation, _)) = BookInfo::get_by_book_id_and_text(book_id, text) {
                    format!("{} {}:{}", abbreviation, chapter, number)
                } else {
                    format!("{} {}:{}", UNDEFINED, chapter, number)
                }
            }
        }
    }
}

fn find_books_by_text(text: &TextId) -> &BookHashMap {
    match text {
        TextId::EnLSB => &BOOK_INFO_FOR_EN_LSB,
        TextId::FiR1933_38 => &BOOK_INFO_FOR_FI_R1933_38,
    }
}
/// Parses a single reference from a string by a given text for the Bible.
///
/// Parsing takes into consideration number of chapters found in a book.
/// If the given chapter exceeds the number of chapters, parsing a reference fails.
pub fn parse_reference_by_text<S>(reference: S, text: &TextId) -> Option<Reference>
where
    S: Into<String>,
{
    let r: String = reference.into();
    let parts = r.trim().split(" ").collect::<Vec<_>>();
    match parts.len() {
        2 => {
            // Construct book abbreviation
            let part_as_sanitized_book_abbreviation = BookInfo::sanitize(parts[0]);

            let Some(book_id) =
                BookId::find_by_sanitized_abbreviation(text, &part_as_sanitized_book_abbreviation) else {
                    return None;
                };

            let Some((_, chapter_count)) = find_books_by_text(text).get(book_id) else {
                return None;
            };

            // Construct chapter or chapter and number, based on if there is a separator ':' between integers
            match parts[1].split(":").collect::<Vec<_>>()[..] {
                [chapter] => {
                    let Ok(chapter_num) = chapter.parse::<u8>() else {
                        return None;
                    };

                    if chapter_num < 1 || chapter_num > *chapter_count {
                        return None;
                    }

                    Some(Reference::BookChapter(book_id.clone(), chapter_num))
                }
                [chapter, number] => {
                    let Ok(chapter_num) = chapter.parse::<u8>() else {
                        return None;
                    };

                    if chapter_num < 1 || chapter_num > *chapter_count {
                        return None;
                    }

                    let Ok(number_num) = number.parse::<u8>() else {
                        return None;
                    };

                    Some(Reference::BookChapterNumber(
                        book_id.clone(),
                        chapter_num,
                        number_num,
                    ))
                }
                _ => None,
            }
        }
        _ => None,
    }
}
/// The same as [parse_reference_by_text] except it parses and returns multiple references which are separated by a semicolon character (';').
pub fn parse_references_by_text<S>(reference: S, text: &TextId) -> Vec<Option<Reference>>
where
    S: Into<String>,
{
    let s: String = reference.into();
    s.split(";")
        .map(|part| parse_reference_by_text(part, text))
        .collect::<Vec<_>>()
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum TextId {
    EnLSB,
    FiR1933_38,
}

#[cfg(test)]
mod tests {
    use super::{parse_reference_by_text, parse_references_by_text, BookId, Reference, TextId};

    macro_rules! unwrap_enum_variant {
        ($value:expr, $pattern:pat => $extracted_value:expr) => {
            match $value {
                $pattern => $extracted_value,
                _ => panic!("Given pattern does not match!"),
            }
        };
    }

    #[test]
    fn convert_valid_reference_from_one_text_to_another() {
        let reference = parse_reference_by_text("Joh 1", &TextId::FiR1933_38).unwrap();
        let result = reference.to_string(TextId::EnLSB);
        assert_eq!(result, "John 1");
    }
    #[test]
    fn fail_parse_reference_with_book_and_chapter_when_reference_is_contains_chapter_that_does_not_exist(
    ) {
        let text = TextId::FiR1933_38;

        let reference = parse_reference_by_text("Joh 0", &text);
        assert!(reference.is_none());

        let reference = parse_reference_by_text("Joh 22", &text);
        assert!(reference.is_none());
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
    fn parse_multiple_references_with_book_and_chapter_when_references_are_correct() {
        let references = parse_references_by_text("Matt 1; Joh. 1", &TextId::FiR1933_38);

        unwrap_enum_variant!(references[0].as_ref().unwrap(), Reference::BookChapter(book_id, chapter) => {
            assert_eq!(*book_id, BookId::Matthew);
            assert_eq!(*chapter, 1);
        });
        unwrap_enum_variant!(references[1].as_ref().unwrap(), Reference::BookChapter(book_id, chapter) => {
            assert_eq!(*book_id, BookId::John);
            assert_eq!(*chapter, 1);
        });
    }
    #[test]
    fn parse_multiple_references_with_book_and_chapter_and_number_when_references_are_correct() {
        let references = parse_references_by_text("Matt 19:18; Joh. 11:12", &TextId::FiR1933_38);

        unwrap_enum_variant!(references[0].as_ref().unwrap(), Reference::BookChapterNumber(book_id, chapter, number) => {
            assert_eq!(*book_id, BookId::Matthew);
            assert_eq!(*chapter, 19);
            assert_eq!(*number, 18);
        });
        unwrap_enum_variant!(references[1].as_ref().unwrap(), Reference::BookChapterNumber(book_id, chapter, number) => {
            assert_eq!(*book_id, BookId::John);
            assert_eq!(*chapter, 11);
            assert_eq!(*number, 12);
        });
    }
    #[test]
    fn parse_reference_with_book_and_chapter_when_reference_is_correct() {
        let text = TextId::FiR1933_38;

        macro_rules! test_book_and_chapter {
            ($reference: literal, $bookId: ident, $chapter:literal) => {
                let reference = parse_reference_by_text($reference, &text).unwrap();

                unwrap_enum_variant!(reference, Reference::BookChapter(book_id, chapter) => {
                    assert_eq!(book_id, BookId::$bookId);
                    assert_eq!(chapter, $chapter);
                });
            };
        }

        test_book_and_chapter!("matt 1", Matthew, 1);
        test_book_and_chapter!("Matt. 1", Matthew, 1);
        test_book_and_chapter!("Matt. 10", Matthew, 10);
        test_book_and_chapter!("Joh. 1", John, 1);
    }
    #[test]
    fn parse_reference_with_book_and_chapter_and_verse_when_reference_is_correct() {
        let text = TextId::FiR1933_38;

        let reference = parse_reference_by_text("Joh 1:1", &text);

        unwrap_enum_variant!(reference.unwrap(), Reference::BookChapterNumber(book_id, chapter, number) => {
            assert_eq!(book_id, BookId::John);
            assert_eq!(chapter, 1);
            assert_eq!(number, 1);
        });

        let reference = parse_reference_by_text("Joh 20:23", &text);

        unwrap_enum_variant!(reference.unwrap(), Reference::BookChapterNumber(book_id, chapter, number) => {
            assert_eq!(book_id, BookId::John);
            assert_eq!(chapter, 20);
            assert_eq!(number, 23);
        });
    }
}
