extern crate colored;

use colored::*;
use std::env;

struct BookCollection {
    title: &'static str,
    books: &'static[&'static str],
    color: &'static str,
}

/// Returns a list of all books from
/// the old testament of the bible
/// in german
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

/// Returns a list of all books from
/// the new testament of the bible
/// in german
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
    // get search string
    let arguments: Vec<String> = env::args().collect();

    let has_search = arguments.len() >= 2;
    let mut search = "";
    if has_search {
        search = arguments[1].as_str();
    }

    // surrounding books on match
    let surround = 2;

    // teaser of program
    let title = "Biblebooks".bold();
    let subtitle = "Finde Bücher in der Bibel".green();

    println!("{}\n{}", title, subtitle);

    // collect books
    let books = [
        BookCollection{title: "Altes Testament", books: ot_books(), color: "blue"},
        BookCollection{title: "Neues Testament", books: nt_books(), color: "yellow"}
        ];

    for book_collection in books.iter() {
        let mut last_five : Vec<usize> = vec![];
        let mut counter = 0;

        println!("\n{}", book_collection.title.italic());
        for (book_index, book) in book_collection.books.iter().enumerate() {
            let mut show = true;

            // filter only the matches
            if has_search {
                if book.to_lowercase().find(&search.to_lowercase()) == None {
                    show = false;
                }
            }

            if show {
                // print books before the match
                for last_one in last_five {
                    println!("{} {}", last_one + 1, book_collection.books[last_one]);
                }

                // print match
                println!("{} {}", book_index + 1, book.color(book_collection.color).italic().bold());

                // clear surrounding books state
                last_five = vec![];
                counter = surround;
            } else {
                if counter > 0 {
                    // print surrounding books after match
                    println!("{} {}", book_index + 1, book);
                    counter -= 1;
                } else {
                    // add book to before match vector
                    last_five.push(book_index);
                    while last_five.len() > surround {
                        last_five.remove(0);
                    }
                }
            }
        }
    }
}
