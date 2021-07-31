use crate::structs;

pub static BOOKCOLLECTIONS : [structs::BookCollection; 2]= [
    structs::BookCollection{
        lang: "de",
        title: "Altes Testament",
        color: "blue",
        books: &[
            structs::Book{title: "1 Mose"},
            structs::Book{title: "2 Mose"},
            structs::Book{title: "3 Mose"},
            structs::Book{title: "4 Mose"},
            structs::Book{title: "5 Mose"},
            structs::Book{title: "Josua"},
            structs::Book{title: "Richter"},
            structs::Book{title: "Ruth"},
            structs::Book{title: "1 Samuel"},
            structs::Book{title: "2 Samuel"},
            structs::Book{title: "1 Könige"},
            structs::Book{title: "2 Könige"},
            structs::Book{title: "1 Chronik"},
            structs::Book{title: "2 Chronik"},
            structs::Book{title: "Esra"},
            structs::Book{title: "Nehemia"},
            structs::Book{title: "Ester"},
            structs::Book{title: "Hiob"},
            structs::Book{title: "Psalm"},
            structs::Book{title: "Sprüche"},
            structs::Book{title: "Prediger"},
            structs::Book{title: "Hohelied"},
            structs::Book{title: "Jesaja"},
            structs::Book{title: "Jeremia"},
            structs::Book{title: "Klagelieder"},
            structs::Book{title: "Hesekiel"},
            structs::Book{title: "Daniel"},
            structs::Book{title: "Hosea"},
            structs::Book{title: "Joel"},
            structs::Book{title: "Amos"},
            structs::Book{title: "Obadja"},
            structs::Book{title: "Jona"},
            structs::Book{title: "Micha"},
            structs::Book{title: "Nahum"},
            structs::Book{title: "Habakuk"},
            structs::Book{title: "Zephanja"},
            structs::Book{title: "Haggai"},
            structs::Book{title: "Sacharja"},
            structs::Book{title: "Maleachi"},
        ],
    },
    structs::BookCollection{
        lang: "de",
        title: "Neues Testament",
        color: "yellow",
        books: &[
           structs::Book{title: "Matthäus"},
           structs::Book{title: "Markus"},
           structs::Book{title: "Lukas"},
           structs::Book{title: "Johannes"},
           structs::Book{title: "Apostelgeschichte"},
           structs::Book{title: "Römer"},
           structs::Book{title: "1 Korinther"},
           structs::Book{title: "2 Korinther"},
           structs::Book{title: "Galater"},
           structs::Book{title: "Epheser"},
           structs::Book{title: "Philipper"},
           structs::Book{title: "Kolosser"},
           structs::Book{title: "1 Thessalonicher"},
           structs::Book{title: "2 Thessalonicher"},
           structs::Book{title: "1 Thimotheus"},
           structs::Book{title: "2 Thimotheus"},
           structs::Book{title: "Titus"},
           structs::Book{title: "Philemon"},
           structs::Book{title: "Hebräer"},
           structs::Book{title: "Jakobus"},
           structs::Book{title: "1 Petrus"},
           structs::Book{title: "2 Petrus"},
           structs::Book{title: "1 Johannes"},
           structs::Book{title: "2 Johannes"},
           structs::Book{title: "3 Johannes"},
           structs::Book{title: "Judas"},
           structs::Book{title: "Offenbarung"},
        ],
    },
];