extern crate colored;

use colored::*;

fn ot_books() -> &'static[&'static str] {
    &[
        "1 Mose",
        "2 Mose",
        "3 Mose",
        "4 Mose",
        "5 Mose",
        "Josua",
        "Richter",
        "Ruth",
        "1 Samuel",
        "2 Samuel",
        "1 Könige",
        "2 Könige",
        "1 Chronik",
        "2 Chronik",
        "Esra",
        "Nehemia",
        "Ester",
        "Hiob",
        "Psalm",
        "Sprüche",
        "Prediger",
        "Hohelied",
        "Jesaja",
        "Jeremia",
        "Klagelieder",
        "Hesekiel",
        "Daniel",
        "Hosea",
        "Joel",
        "Amos",
        "Obadja",
        "Jona",
        "Micha",
        "Nahum",
        "Habakuk",
        "Zephanja",
        "Haggai",
        "Sacharja",
        "Maleachi",
    ]
}

fn nt_books() -> &'static[&'static str] {
    &[
       "Matthäus",
       "Markus",
       "Lukas",
       "Johannes",
       "Apostelgeschichte",
       "Römer",
       "1 Korinther",
       "2 Korinther",
       "Galater",
       "Epheser",
       "Philipper",
       "Kolosser",
       "1 Thessalonicher",
       "2 Thessalonicher",
       "1 Thimotheus",
       "2 Thimotheus",
       "Titus",
       "Philemon",
       "Hebräer",
       "Jakobus",
       "1 Petrus",
       "2 Petrus",
       "1 Johannes",
       "2 Johannes",
       "3 Johannes",
       "Judas",
       "Offenbarung",
    ]
}

fn main() {
    let title = "Biblebooks".bold();
    let subtitle = "Find books in the bible".green();

    println!("{}\n{}\n", title, subtitle);

    for book in ot_books() {
        println!("{}", book.blue().italic())
    }

    for book in nt_books() {
        println!("{}", book.yellow().italic())
    }
}
