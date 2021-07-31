extern crate colored;

mod structs;
mod bookcollections;

use colored::*;
use std::env;

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

    let books = &bookcollections::BOOKCOLLECTIONS;

    for book_collection in books.iter() {
        let mut last_five : Vec<usize> = vec![];
        let mut counter = 0;

        println!("\n{}", book_collection.title.italic());
        for (book_index, book) in book_collection.books.iter().enumerate() {
            let mut show = true;

            // filter only the matches
            if has_search {
                if book.title.to_lowercase().find(&search.to_lowercase()) == None {
                    show = false;
                }
            }

            if show {
                // print books before the match
                for last_one in last_five {
                    println!("{} {}", last_one + 1, book_collection.books[last_one].title);
                }

                // print match
                println!("{} {}", book_index + 1, book.title.color(book_collection.color).italic().bold());

                // clear surrounding books state
                last_five = vec![];
                counter = surround;
            } else {
                if counter > 0 {
                    // print surrounding books after match
                    println!("{} {}", book_index + 1, book.title);
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
