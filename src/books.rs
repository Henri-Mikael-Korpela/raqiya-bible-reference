use std::collections::HashMap;

extern crate lazy_static;

use lazy_static::lazy_static;

use crate::{BookHashMap, BookId};

lazy_static! {
    pub static ref BOOK_INFO_FOR_EN_LSB: BookHashMap = HashMap::from([
        (BookId::Genesis, ("Genesis", 50)),
        (BookId::Exodus, ("Exodus", 40)),
        (BookId::Leviticus, ("Leviticus", 27)),
        (BookId::Numbers, ("Numbers", 36)),
        (BookId::Deuteronomy, ("Deuteronomy", 34)),
        (BookId::Matthew, ("Matthew", 28)),
        (BookId::John, ("John", 21)),
    ]);
    pub static ref BOOK_INFO_FOR_FI_R1933_38: BookHashMap = HashMap::from([
        (BookId::Genesis, ("1. Moos.", 50)),
        (BookId::Exodus, ("2. Moos.", 40)),
        (BookId::Leviticus, ("3. Moos.", 27)),
        (BookId::Numbers, ("4. Moos.", 36)),
        (BookId::Deuteronomy, ("5. Moos.", 34)),
        (BookId::Matthew, ("Matt.", 28)),
        (BookId::John, ("Joh.", 21))
    ]);
    pub static ref BOOK_ABBREVIATIONS_TO_IDS_EN: HashMap<&'static str, BookId> = HashMap::from([
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
    pub static ref BOOK_ABBREVIATIONS_TO_IDS_FI: HashMap<&'static str, BookId> = HashMap::from([
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
