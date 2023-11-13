use std::{fs::File, io::BufReader};

use xml::reader::XmlEvent;

pub enum Locale {
    En,
}

#[derive(Debug, PartialEq)]
pub struct Reference {
    pub chapter: u8,
    pub number: u8,
    pub content: String,
}

#[derive(Debug, PartialEq)]
pub struct ReferenceParseResult<'a> {
    pub book_name: &'a str,
    pub chapter: u8,
    reference_type: ReferenceParseResultType,
}
#[derive(Debug, PartialEq)]
pub enum ReferenceParseResultType {
    /// Bible verse reference to a chapter.
    Chapter,
    /// Bible verse reference to a verse.
    Verse { number: u8 },
    /// Bible verse reference to a verse and all verses onwards in a chapter.
    VerseFromOnwards { number_from: u8 },
    /// Bible verse reference to a range of verses.
    VerseFromTo { number_from: u8, number_to: u8 },
}
#[derive(Debug, PartialEq)]
pub enum ReferenceParseErrorCode {
    BookNameNeverEnds,
    InvalidChapterFormat,
    InvalidRangeBetweenVerseNumbers,
    InvalidVerseNumberFormat,
    UnknownError,
}
impl ReferenceParseErrorCode {
    pub fn to_string(&self, locale: Locale) -> &'static str {
        match locale {
            Locale::En => match self {
                ReferenceParseErrorCode::BookNameNeverEnds => "Book name never ends.",
                ReferenceParseErrorCode::InvalidChapterFormat => "Invalid chapter format.",
                ReferenceParseErrorCode::InvalidRangeBetweenVerseNumbers => {
                    "Invalid range between verse numbers."
                }
                ReferenceParseErrorCode::InvalidVerseNumberFormat => "Invalid verse number format.",
                ReferenceParseErrorCode::UnknownError => "Unknown error.",
            },
        }
    }
}

const EMPTY_STRING: &str = "";

pub fn find_content_in_source(
    file_reader: &mut BufReader<File>,
    parse_result: &ReferenceParseResult,
) -> Result<Vec<Reference>, String> {
    let mut parser = xml::EventReader::new(file_reader);

    match parse_result.reference_type {
        ReferenceParseResultType::VerseFromTo {
            number_from,
            number_to,
        } => {
            let verse_reference_ids = (number_from..=number_to)
                .map(|n| format!("{}.{}.{}", parse_result.book_name, parse_result.chapter, n))
                .collect::<Vec<_>>();

            let mut verse_references = Vec::<Reference>::with_capacity(verse_reference_ids.len());
            let mut verse_references_handled_count = 0;

            while let Ok(element) = parser.next() {
                if let XmlEvent::StartElement {
                    name, attributes, ..
                } = element
                {
                    if name.local_name == "verse" {
                        // Expect all verses to be in numerical order.
                        if let Some(_) = attributes.iter().find(|attribute| {
                            attribute.name.local_name == "osisID"
                                && attribute.value
                                    == verse_reference_ids[verse_references_handled_count]
                        }) {
                            if let Ok(XmlEvent::Characters(content)) = parser.next() {
                                verse_references.push(Reference {
                                    chapter: parse_result.chapter,
                                    number: number_from + verse_references_handled_count as u8,
                                    content,
                                });
                            } else {
                                return Err(String::from("Failed to parse verse content. Expected verse content to follow verse start element."));
                            }

                            verse_references_handled_count += 1;

                            if verse_references_handled_count == verse_reference_ids.len() {
                                break;
                            }
                        }
                    }
                }
            }

            Ok(verse_references)
        }
        _ => Ok(vec![]),
    }
}
/// Parses a Bible reference string into a parse result object.
pub fn parse_reference(value: &str) -> Result<ReferenceParseResult, ReferenceParseErrorCode> {
    let mut book_name = EMPTY_STRING;
    let mut chapter = 0;
    let mut number = 0;
    let mut value_chars = value.chars().enumerate().peekable();

    'value_chars_loop: while let Some((i, c)) = value_chars.next() {
        // If a character is an alphabetic character, then expect a book name to follow it.
        if c.is_alphabetic() {
            while let Some((j, book_name_c)) = value_chars.peek() {
                if book_name_c.is_digit(10) {
                    let mut trim_offset = 0;
                    loop {
                        if value.chars().nth(*j - 1 - trim_offset).unwrap() == ' ' {
                            trim_offset += 1;
                        } else {
                            break;
                        }
                    }
                    book_name = &value[i..*j - trim_offset];
                    continue 'value_chars_loop;
                } else {
                    value_chars.next();
                }
            }
            return Err(ReferenceParseErrorCode::BookNameNeverEnds);
        } else if c.is_digit(10) {
            // If a (verse) number is found, then expect an end verse number to follow it.
            if number != 0 {
                let mut end_number_str_end = 1;

                'collect_number: while let Some((_, number_c)) = value_chars.peek() {
                    if number_c.is_digit(10) {
                        value_chars.next();
                        end_number_str_end += 1;
                    } else {
                        break 'collect_number;
                    }
                }

                let end_number_str = &value[i..i + end_number_str_end];
                let end_number = end_number_str
                    .parse::<u8>()
                    .map_err(|_| ReferenceParseErrorCode::InvalidVerseNumberFormat)?;

                // Ensure that the end verse number is greater than the start verse number.
                if end_number < number {
                    return Err(ReferenceParseErrorCode::InvalidRangeBetweenVerseNumbers);
                }

                return Ok(ReferenceParseResult {
                    book_name,
                    chapter,
                    reference_type: ReferenceParseResultType::VerseFromTo {
                        number_from: number,
                        number_to: end_number,
                    },
                });
            }
            // If a chapter is already found, then expect a verse number to follow it.
            else if chapter != 0 {
                let mut number_str_end = 1;

                'collect_number: while let Some((_, number_c)) = value_chars.peek() {
                    if number_c.is_digit(10) {
                        value_chars.next();
                        number_str_end += 1;
                    } else if *number_c == '-' {
                        value_chars.next();
                        let number_str = &value[i..i + number_str_end];
                        number = number_str
                            .parse::<u8>()
                            .map_err(|_| ReferenceParseErrorCode::InvalidVerseNumberFormat)?;
                        continue 'value_chars_loop;
                    } else if *number_c == '+' {
                        let number_str = &value[i..i + number_str_end];
                        let number_from = number_str
                            .parse::<u8>()
                            .map_err(|_| ReferenceParseErrorCode::InvalidVerseNumberFormat)?;
                        return Ok(ReferenceParseResult {
                            book_name,
                            chapter,
                            reference_type: ReferenceParseResultType::VerseFromOnwards {
                                number_from,
                            },
                        });
                    } else {
                        break 'collect_number;
                    }
                }

                let number_str = &value[i..i + number_str_end];
                let number = number_str
                    .parse::<u8>()
                    .map_err(|_| ReferenceParseErrorCode::InvalidChapterFormat)?;
                return Ok(ReferenceParseResult {
                    book_name,
                    chapter,
                    reference_type: ReferenceParseResultType::Verse { number },
                });
            }
            // If a book name is already found, then expect a chapter number to follow it.
            else if book_name != EMPTY_STRING {
                let mut chapter_str_end = 1;

                'collect_chapter: while let Some((_, chapter_c)) = value_chars.peek() {
                    if chapter_c.is_digit(10) {
                        value_chars.next();
                        chapter_str_end += 1;
                    } else if *chapter_c == ':' {
                        // If a chapter and verse number separator (:) is found, then store chapter
                        // and start the next iteration to expect to collect a verse number.
                        value_chars.next();
                        let chapter_str = &value[i..i + chapter_str_end];
                        chapter = chapter_str
                            .parse::<u8>()
                            .map_err(|_| ReferenceParseErrorCode::InvalidChapterFormat)?;
                        continue 'value_chars_loop;
                    } else {
                        break 'collect_chapter;
                    }
                }

                let chapter_str = &value[i..i + chapter_str_end];
                let chapter = chapter_str
                    .parse::<u8>()
                    .map_err(|_| ReferenceParseErrorCode::InvalidChapterFormat)?;

                return Ok(ReferenceParseResult {
                    book_name,
                    chapter,
                    reference_type: ReferenceParseResultType::Chapter,
                });
            }
        }
    }

    Err(ReferenceParseErrorCode::UnknownError)
}
pub fn parse_references(value: &str) -> Result<Vec<ReferenceParseResult>, ReferenceParseErrorCode> {
    let mut references = Vec::new();

    for reference_str in value.split(';') {
        let reference = parse_reference(reference_str)?;
        references.push(reference);
    }

    Ok(references)
}

#[cfg(test)]
mod tests {
    use std::{env, fs::File};

    use super::*;

    #[test]
    fn find_content_in_source_kjv() {
        let project_dir = env::current_dir().unwrap();
        let xml_file_path = project_dir.join("assets/kjv.xml");
        let file = File::open(xml_file_path).unwrap();
        let mut file_reader = BufReader::new(file);

        let parse_result = ReferenceParseResult {
            book_name: "John",
            chapter: 3,
            reference_type: ReferenceParseResultType::VerseFromTo {
                number_from: 1,
                number_to: 2,
            },
        };

        let content = find_content_in_source(&mut file_reader, &parse_result).unwrap();
        assert_eq!(
            content,
            vec![Reference {
                chapter: 3,
                number: 1,
                content: String::from("There was a man of the Pharisees, named Nicodemus, a ruler of the Jews:")
            },
            Reference {
                chapter: 3,
                number: 2,
                content: String::from("The same came to Jesus by night, and said unto him, Rabbi, we know that thou art a teacher come from God: for no man can do these miracles that thou doest, except God be with him.")
            }]
        );
    }

    #[test]
    fn fail_parse_reference_to_many_verses_with_invalid_range_between_verse_numbers() {
        // The end verse number is less than the start verse number,
        // which doesn't make sense and should fail.
        let parse_result = parse_reference("John 3:2-1");
        assert_eq!(
            parse_result,
            Err(ReferenceParseErrorCode::InvalidRangeBetweenVerseNumbers)
        );
    }
    #[test]
    fn parse_reference_to_chapter_with_one_word_book_name() {
        let parse_result = parse_reference("John 3").unwrap();
        assert_eq!(
            parse_result,
            ReferenceParseResult {
                book_name: "John",
                chapter: 3,
                reference_type: ReferenceParseResultType::Chapter
            }
        );
    }
    #[test]
    fn parse_reference_to_chapter_with_two_word_book_name() {
        let parse_result = parse_reference("1 John 3").unwrap();
        assert_eq!(
            parse_result,
            ReferenceParseResult {
                book_name: "John",
                chapter: 3,
                reference_type: ReferenceParseResultType::Chapter
            }
        );

        let parse_result = parse_reference("1 John 15").unwrap();
        assert_eq!(
            parse_result,
            ReferenceParseResult {
                book_name: "John",
                chapter: 15,
                reference_type: ReferenceParseResultType::Chapter
            }
        );
    }
    #[test]
    fn parse_reference_to_one_verse() {
        let parse_result = parse_reference("John 3:1").unwrap();
        assert_eq!(
            parse_result,
            ReferenceParseResult {
                book_name: "John",
                chapter: 3,
                reference_type: ReferenceParseResultType::Verse { number: 1 }
            }
        );

        let parse_result = parse_reference("John 3:16").unwrap();
        assert_eq!(
            parse_result,
            ReferenceParseResult {
                book_name: "John",
                chapter: 3,
                reference_type: ReferenceParseResultType::Verse { number: 16 }
            }
        );
    }
    #[test]
    fn parse_reference_to_one_verse_and_onwards() {
        let parse_result = parse_reference("John 3:1+").unwrap();
        assert_eq!(
            parse_result,
            ReferenceParseResult {
                book_name: "John",
                chapter: 3,
                reference_type: ReferenceParseResultType::VerseFromOnwards { number_from: 1 }
            }
        );
    }
    #[test]
    fn parse_reference_to_many_verses_in_range() {
        let parse_result = parse_reference("John 3:1-2").unwrap();
        assert_eq!(
            parse_result,
            ReferenceParseResult {
                book_name: "John",
                chapter: 3,
                reference_type: ReferenceParseResultType::VerseFromTo {
                    number_from: 1,
                    number_to: 2
                }
            }
        );
    }
    #[test]
    fn parse_references() {
        let parse_result = super::parse_references("John 3:1-2; John 3:4-5").unwrap();
        assert_eq!(
            parse_result,
            &[
                ReferenceParseResult {
                    book_name: "John",
                    chapter: 3,
                    reference_type: ReferenceParseResultType::VerseFromTo {
                        number_from: 1,
                        number_to: 2
                    }
                },
                ReferenceParseResult {
                    book_name: "John",
                    chapter: 3,
                    reference_type: ReferenceParseResultType::VerseFromTo {
                        number_from: 4,
                        number_to: 5
                    }
                }
            ]
        );
    }
}
