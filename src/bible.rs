use std::{borrow::Cow, collections::HashMap};

extern crate lazy_static;
extern crate regex;

use lazy_static::lazy_static;
use regex::{Captures, Regex};

type BookHashMap = HashMap<BookId, (&'static str, u8)>;

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
#[repr(u8)]
pub enum BookId {
    // All book IDs are listed in the canonical order,
    // from the Old Testament to the New Testament.
    Genesis,
    Exodus,
    Leviticus,
    Numbers,
    Deuteronomy,
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
    fn get_by_book_id_and_text(book_id: &BookId, text: &TextId) -> Option<(&'static str, u8)> {
        match text {
            TextId::EnLSB => BOOK_INFO_FOR_EN_LSB.get(book_id).copied(),
            TextId::FiR1933_38 => BOOK_INFO_FOR_FI_R1933_38.get(book_id).copied(),
        }
    }
    pub fn sanitize(value: &str) -> String {
        value.to_lowercase()
    }
}

lazy_static! {
    static ref BOOK_INFO_FOR_EN_LSB: BookHashMap = HashMap::from([
        (BookId::Genesis, ("Genesis", 50)),
        (BookId::Exodus, ("Exodus", 40)),
        (BookId::Leviticus, ("Leviticus", 27)),
        (BookId::Numbers, ("Numbers", 36)),
        (BookId::Deuteronomy, ("Deuteronomy", 34)),
        (BookId::Matthew, ("Matthew", 28)),
        (BookId::John, ("John", 21)),
    ]);
    static ref BOOK_INFO_FOR_FI_R1933_38: BookHashMap = HashMap::from([
        (BookId::Genesis, ("1. Moos.", 50)),
        (BookId::Exodus, ("2. Moos.", 40)),
        (BookId::Leviticus, ("3. Moos.", 27)),
        (BookId::Numbers, ("4. Moos.", 36)),
        (BookId::Deuteronomy, ("5. Moos.", 34)),
        (BookId::Matthew, ("Matt.", 28)),
        (BookId::John, ("Joh.", 21))
    ]);
    static ref BOOK_ABBREVIATIONS_TO_IDS_EN: HashMap<&'static str, BookId> = HashMap::from([
        ("gen", BookId::Genesis),
        ("genesis", BookId::Genesis),
        ("gn", BookId::Genesis),
        ("ex", BookId::Exodus),
        ("exo", BookId::Exodus),
        ("exodus", BookId::Exodus),
        ("lev", BookId::Leviticus),
        ("leviticus", BookId::Leviticus),
        ("lv", BookId::Leviticus),
        ("nm", BookId::Numbers),
        ("num", BookId::Numbers),
        ("numbers", BookId::Numbers),
        ("de", BookId::Deuteronomy),
        ("deu", BookId::Deuteronomy),
        ("deuteronomy", BookId::Deuteronomy),
        ("dt", BookId::Deuteronomy),
        ("matt", BookId::Matthew),
        ("matthew", BookId::Matthew),
        ("mt", BookId::Matthew),
        ("jh", BookId::John),
        ("john", BookId::John),
    ]);
    static ref BOOK_ABBREVIATIONS_TO_IDS_FI: HashMap<&'static str, BookId> = HashMap::from([
        ("1mo", BookId::Genesis),
        ("1 moos", BookId::Genesis),
        ("1 mooses", BookId::Genesis),
        ("1. moos.", BookId::Genesis),
        ("2mo", BookId::Exodus),
        ("2 moos", BookId::Exodus),
        ("2 mooses", BookId::Exodus),
        ("2. moos.", BookId::Exodus),
        ("3mo", BookId::Leviticus),
        ("3 moos", BookId::Leviticus),
        ("3 mooses", BookId::Leviticus),
        ("3. moos.", BookId::Leviticus),
        ("4mo", BookId::Numbers),
        ("4 moos", BookId::Numbers),
        ("4 mooses", BookId::Numbers),
        ("4. moos.", BookId::Numbers),
        ("5mo", BookId::Deuteronomy),
        ("5 moos", BookId::Deuteronomy),
        ("5 mooses", BookId::Deuteronomy),
        ("5. moos.", BookId::Deuteronomy),
        ("matt", BookId::Matthew),
        ("matt.", BookId::Matthew),
        ("matteus", BookId::Matthew),
        ("joh", BookId::John),
        ("joh.", BookId::John),
        ("johannes", BookId::John),
    ]);
}

/// Represents a reference to a Bible passage via chapter and or more verses.
#[derive(Debug)]
pub enum Reference {
    BookChapter(BookId, u8),
    BookChapterNumber(BookId, u8, u8),
    BookChapterNumberFromTo(BookId, u8, u8, u8),
}
impl Reference {
    /// TODO: Optimize by avoiding match statement and just get data somewhere directly.
    pub fn get_book_abbreviation(&self, text: &TextId) -> &'static str {
        let book_id = &match self {
            Self::BookChapter(book_id, _) => book_id.clone(),
            Self::BookChapterNumber(book_id, _, _) => book_id.clone(),
            Self::BookChapterNumberFromTo(book_id, _, _, _) => book_id.clone(),
        };
        let book_info = match text {
            TextId::EnLSB => BOOK_INFO_FOR_EN_LSB.get(book_id),
            TextId::FiR1933_38 => BOOK_INFO_FOR_FI_R1933_38.get(book_id),
        };

        // Book info should always be found. That is whu unwrapping is performed here
        book_info.unwrap().0
    }
    /// TODO: Optimize by avoiding match statement and just get data somewhere directly.
    #[inline]
    pub fn get_chapter(&self) -> u8 {
        match self {
            Self::BookChapter(_, chapter) => *chapter,
            Self::BookChapterNumber(_, chapter, _) => *chapter,
            Self::BookChapterNumberFromTo(_, chapter, _, _) => *chapter,
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
    pub fn to_string(&self, text: &TextId) -> String {
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
            Self::BookChapterNumberFromTo(book_id, chapter, number_from, number_to) => {
                if let Some((abbreviation, _)) = BookInfo::get_by_book_id_and_text(book_id, text) {
                    format!("{} {}:{}-{}", abbreviation, chapter, number_from, number_to)
                } else {
                    format!("{} {}:{}-{}", UNDEFINED, chapter, number_from, number_to)
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct ReferenceMatch<'a> {
    pub content: &'a str,
    pub position: usize,
}

fn find_book_info_by_text(text: &TextId) -> &BookHashMap {
    match text {
        TextId::EnLSB => &BOOK_INFO_FOR_EN_LSB,
        TextId::FiR1933_38 => &BOOK_INFO_FOR_FI_R1933_38,
    }
}
/// Finds all Bible passage references in a given value with their content and position.
/// Only those book abbreviations included in a given text are supported.
pub fn find_reference_matches_in<'a, S>(content: S, text: &TextId) -> Vec<ReferenceMatch<'a>>
where
    S: Into<&'a str>,
{
    let re = make_reference_match_pattern(text);
    re.captures_iter(content.into())
        .map(|captures| {
            let capture = captures.get(0).unwrap();
            ReferenceMatch {
                content: capture.as_str(),
                position: capture.start(),
            }
        })
        .collect::<Vec<_>>()
}
fn make_reference_match_pattern(text: &TextId) -> Regex {
    let abbreviations = match text {
        TextId::EnLSB => BOOK_ABBREVIATIONS_TO_IDS_EN.keys(),
        TextId::FiR1933_38 => BOOK_ABBREVIATIONS_TO_IDS_FI.keys(),
    };

    let match_pattern = {
        let abbreviations_in_pattern = abbreviations.map(|a| *a).collect::<Vec<_>>().join("|");
        let chapter_pattern = "\\s\\d{1,}";
        let chapter_and_number_pattern = "\\s\\d{1,}:\\d{1,}";
        let abbreviations_and_chapter_and_number_in_pattern = format!(
            "({})({}|{})",
            abbreviations_in_pattern, chapter_and_number_pattern, chapter_pattern
        );
        format!("(?i)({})", abbreviations_and_chapter_and_number_in_pattern)
    };

    let re = Regex::new(match_pattern.as_str()).unwrap();
    re
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

            let Some((_, chapter_count)) = find_book_info_by_text(text).get(book_id) else {
                return None;
            };

            // Construct chapter or chapter and number if there is a separator ':' between integers
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

                    // Construct number or number from and number to if there is a separator '-' between integers
                    match number.split("-").collect::<Vec<_>>()[..] {
                        [number] => {
                            let Ok(number_num) = number.parse::<u8>() else {
                                return None;
                            };

                            Some(Reference::BookChapterNumber(
                                book_id.clone(),
                                chapter_num,
                                number_num,
                            ))
                        }
                        [number_from, number_to] => {
                            let Ok(number_from_num) = number_from.parse::<u8>() else {
                                return None;
                            };
                            let Ok(number_to_num) = number_to.parse::<u8>() else {
                                return None;
                            };

                            Some(Reference::BookChapterNumberFromTo(
                                book_id.clone(),
                                chapter_num,
                                number_from_num,
                                number_to_num,
                            ))
                        }
                        _ => None,
                    }
                }
                _ => None,
            }
        }
        _ => None,
    }
}
/// The same as [parse_reference_by_text] except it parses and
/// returns multiple references which are separated by a semicolon character (';').
pub fn parse_references_by_text<S>(reference: S, text: &TextId) -> Vec<Option<Reference>>
where
    S: Into<String>,
{
    let s: String = reference.into();
    s.split(";")
        .map(|part| parse_reference_by_text(part, text))
        .collect::<Vec<_>>()
}
/// Replaces all references found with a corresponding reference found according to a given text.
///
/// In case a replacement reference for the original reference cannot be parsed, the original reference remains.
pub fn replace_reference_matches_in<'a, S, Replacer>(
    content: S,
    text: &'a TextId,
    replacer: Replacer,
) -> Cow<str>
where
    S: Into<&'a str>,
    Replacer: Fn(&Reference) -> String,
{
    let re = make_reference_match_pattern(text);
    let content_with_replacements =
        re.replace_all(content.into(), |captures: &Captures| -> String {
            let capture_content = captures.get(0).unwrap().as_str();
            if let Some(reference) = parse_reference_by_text(capture_content, text) {
                replacer(&reference)
            } else {
                capture_content.to_string()
            }
        });
    content_with_replacements
}

/// Represents a text containg Bible content.
/// It can be
/// - a critical edition of the Old Testament (like BHS, Biblia Hebraica Stuttgartensia)
/// - a critical edition of the New Testament (like NA28, Nestle-Aland Novum Testamentum Graece 28)
/// - a partial translation of the Bible (like Septuagint, which contains only the Old Testament in Greek).
/// - a complete translation of the Bible (like LSB, Legacy Standard Bible, an English translation).
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum TextId {
    EnLSB,
    FiR1933_38,
}

#[cfg(test)]
mod tests {
    use super::{
        find_reference_matches_in, parse_reference_by_text, parse_references_by_text, BookId,
        Reference, TextId,
    };

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
        let result = reference.to_string(&TextId::EnLSB);
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
    fn find_references_in_str() {
        let matches = find_reference_matches_in("Example Matt. 3 (Joh 12:24)", &TextId::FiR1933_38);

        assert_eq!(matches.len(), 2);

        assert_eq!(matches[0].content, "Matt. 3");
        assert_eq!(matches[0].position, 8);

        assert_eq!(matches[1].content, "Joh 12:24");
        assert_eq!(matches[1].position, 17);
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
    fn parse_reference_with_book_and_chapter_and_number_when_reference_is_correct() {
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
    #[test]
    fn parse_reference_with_book_and_chapter_and_number_from_and_number_to_when_reference_is_correct(
    ) {
        let text = TextId::FiR1933_38;

        let reference = parse_reference_by_text("Joh 1:3-8", &text);

        unwrap_enum_variant!(reference.unwrap(), Reference::BookChapterNumberFromTo(book_id, chapter, number_from, number_to) => {
            assert_eq!(book_id, BookId::John);
            assert_eq!(chapter, 1);
            assert_eq!(number_from, 3);
            assert_eq!(number_to, 8);
        });

        let reference = parse_reference_by_text("Joh 20:15-27", &text);

        unwrap_enum_variant!(reference.unwrap(), Reference::BookChapterNumberFromTo(book_id, chapter, number_from, number_to) => {
            assert_eq!(book_id, BookId::John);
            assert_eq!(chapter, 20);
            assert_eq!(number_from, 15);
            assert_eq!(number_to, 27);
        });
    }
}
