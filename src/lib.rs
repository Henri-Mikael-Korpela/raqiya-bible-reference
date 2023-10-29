pub enum Locale {
    English,
}

#[derive(Debug, PartialEq)]
pub enum ParseResult<'a> {
    Chapter {
        book_name: &'a str,
        chapter: u8,
    },
    Verse {
        book_name: &'a str,
        chapter: u8,
        number: u8,
    },
    VerseFromTo {
        book_name: &'a str,
        chapter: u8,
        number_from: u8,
        number_to: u8,
    },
}
#[derive(Debug, PartialEq)]
pub enum ParseErrorCode {
    BookNameNeverEnds,
    InvalidChapterFormat,
    InvalidRangeBetweenVerseNumbers,
    InvalidVerseNumberFormat,
    UnknownError,
}
impl ParseErrorCode {
    pub fn to_string(&self, locale: Locale) -> &'static str {
        match locale {
            Locale::English => match self {
                ParseErrorCode::BookNameNeverEnds => "Book name never ends.",
                ParseErrorCode::InvalidChapterFormat => "Invalid chapter format.",
                ParseErrorCode::InvalidRangeBetweenVerseNumbers => {
                    "Invalid range between verse numbers."
                }
                ParseErrorCode::InvalidVerseNumberFormat => "Invalid verse number format.",
                ParseErrorCode::UnknownError => "Unknown error.",
            },
        }
    }
}

const EMPTY_STRING: &str = "";

pub fn parse(value: &str) -> Result<ParseResult, ParseErrorCode> {
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
            return Err(ParseErrorCode::BookNameNeverEnds);
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
                    .map_err(|_| ParseErrorCode::InvalidVerseNumberFormat)?;

                // Ensure that the end verse number is greater than the start verse number.
                if end_number < number {
                    return Err(ParseErrorCode::InvalidRangeBetweenVerseNumbers);
                }

                return Ok(ParseResult::VerseFromTo {
                    book_name,
                    chapter,
                    number_from: number,
                    number_to: end_number,
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
                            .map_err(|_| ParseErrorCode::InvalidVerseNumberFormat)?;
                        continue 'value_chars_loop;
                    } else {
                        break 'collect_number;
                    }
                }

                let number_str = &value[i..i + number_str_end];
                let number = number_str
                    .parse::<u8>()
                    .map_err(|_| ParseErrorCode::InvalidChapterFormat)?;
                return Ok(ParseResult::Verse {
                    book_name,
                    chapter,
                    number,
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
                            .map_err(|_| ParseErrorCode::InvalidChapterFormat)?;
                        continue 'value_chars_loop;
                    } else {
                        break 'collect_chapter;
                    }
                }

                let chapter_str = &value[i..i + chapter_str_end];
                let chapter = chapter_str
                    .parse::<u8>()
                    .map_err(|_| ParseErrorCode::InvalidChapterFormat)?;

                return Ok(ParseResult::Chapter { book_name, chapter });
            }
        }
    }

    Err(ParseErrorCode::UnknownError)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fail_parse_reference_to_many_verses_with_invalid_range_between_verse_numbers() {
        // The end verse number is less than the start verse number,
        // which doesn't make sense and should fail.
        let parse_result = parse("John 3:2-1");
        assert_eq!(
            parse_result,
            Err(ParseErrorCode::InvalidRangeBetweenVerseNumbers)
        );
    }
    #[test]
    fn parse_reference_to_chapter_with_one_word_book_name() {
        let parse_result = parse("John 3").unwrap();
        assert_eq!(
            parse_result,
            ParseResult::Chapter {
                book_name: "John",
                chapter: 3
            }
        );
    }
    #[test]
    fn parse_reference_to_chapter_with_two_word_book_name() {
        let parse_result = parse("1 John 3").unwrap();
        assert_eq!(
            parse_result,
            ParseResult::Chapter {
                book_name: "John",
                chapter: 3
            }
        );

        let parse_result = parse("1 John 15").unwrap();
        assert_eq!(
            parse_result,
            ParseResult::Chapter {
                book_name: "John",
                chapter: 15
            }
        );
    }
    #[test]
    fn parse_reference_to_one_verse() {
        let parse_result = parse("John 3:1").unwrap();
        assert_eq!(
            parse_result,
            ParseResult::Verse {
                book_name: "John",
                chapter: 3,
                number: 1
            }
        );

        let parse_result = parse("John 3:16").unwrap();
        assert_eq!(
            parse_result,
            ParseResult::Verse {
                book_name: "John",
                chapter: 3,
                number: 16
            }
        );
    }
    #[test]
    fn parse_reference_to_many_verses() {
        let parse_result = parse("John 3:1-2").unwrap();
        assert_eq!(
            parse_result,
            ParseResult::VerseFromTo {
                book_name: "John",
                chapter: 3,
                number_from: 1,
                number_to: 2
            }
        );
    }
}
