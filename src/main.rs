use anstream::println;
use comfy_table::Table;
use owo_colors::OwoColorize as _;
use std::fs;
use std::io::{BufReader, Read};

const FW: &'static str = "fw";
const FILE_FLAG: &'static str = "-f";
const WORD_FLAG: &'static str = "-w";
const HELP_FLAG: &'static str = "-help";

const DESKTOP_SHORT_PATH: &'static str = "C:\\Users\\User\\Desktop";

const SUPPORTED_FILE_EXT: &'static [&str] = &[".txt", ".html", ".xml", ".json"];

#[derive(Debug)]
struct FoundItem {
    word: String,
    count: u32,
    result: String,
}
struct FindWord;

impl FindWord {
    fn find(file_content: String, _word: String) -> Result<FoundItem, FindError> {
        let mut result = String::new();
        let mut count: u32 = 0;
        for line in file_content.lines() {
            let words = line.split_whitespace().collect::<Vec<&str>>();
            let mut updated_line = String::new();
            for word in words {
                if word == _word {
                    let colored = word.red();
                    updated_line.push_str(&colored.to_string());
                    updated_line.push_str(" ");
                    count += 1;
                } else {
                    updated_line.push_str(word);
                    updated_line.push_str(" ")
                }
            }
            result.push_str(updated_line.as_str());
            result.push_str("\n");
        }
        if count == 0 {
            return Err(FindError::NotFound(String::from(
                "En az 1 tane bile kelime bulunamadı",
            )));
        }
        return Ok(FoundItem {
            word: _word,
            count: count,
            result: result,
        });
    }
}

#[derive(Debug)]
enum FindError {
    NotFound(String),
}

fn read_file(file_path: String) -> String {
    let file = fs::File::open(file_path).unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut content = String::new();
    buf_reader.read_to_string(&mut content).unwrap();
    return content;
}

#[derive(Debug)]
struct ParseArgs {
    cmd: String,
    file: String,
    word: String,
}

impl ParseArgs {
    fn parse(args: &Vec<String>) -> Result<Option<ParseArgs>, ParseError> {
        if args.len() == 2 && args[1] == HELP_FLAG {
            return Ok(None);
        }

        if args.len() != 5 {
            return Err(ParseError::InvalidArgsLen(String::from("invalid args len")));
        }
        let cmd = &args[0];

        if cmd != &FW.to_string() {
            return Err(ParseError::InvalidCmd(String::from("invalid cmd")));
        }

        if !args.contains(&FILE_FLAG.to_string()) || !args.contains(&WORD_FLAG.to_string()) {
            return Err(ParseError::MissingOption(String::from("missing option")));
        }

        let mut file = args[2].clone();
        if !is_supported_file_ext(
            &(String::from(".") + &file.split(".").collect::<Vec<_>>()[1].to_string()),
        ) {
            return Err(ParseError::FileExtNotSupported(String::from(
                "file ext not supported",
            )));
        }

        if file.contains("desk/") {
            let new_file = DESKTOP_SHORT_PATH.to_string() + &file[4..];
            file = new_file
        }

        let word = &args[4];
        return Ok(Some(ParseArgs {
            cmd: cmd.to_string(),
            file: file.to_string(),
            word: word.to_string(),
        }));
    }
}

#[derive(Debug)]
enum ParseError {
    InvalidCmd(String),
    InvalidArgsLen(String),
    MissingOption(String),
    FileExtNotSupported(String),
}

fn is_supported_file_ext(ext: &String) -> bool {
    return SUPPORTED_FILE_EXT.contains(&ext.as_str());
}

fn create_table() -> Result<(Table, Table), ()> {
    let mut commands_table = Table::new();
    commands_table.set_header(vec!["Komut", "Komut Açıklaması"]);
    commands_table.add_row(vec![
        "fw",
        "Komut satırı uygulaması olan 'Find Word' uygulamasını çalıştırır.",
    ]);
    commands_table.add_row(vec!["-f", "Hedef dosyayı belirtir."]);
    commands_table.add_row(vec!["-w", "Hedef kelimeyi belirtir."]);

    let mut short_cuts_table = Table::new();
    short_cuts_table.set_header(vec!["Kısa Yol", "Kısa Yol Açıklaması", "Örnek Kullanım"]);
    short_cuts_table.add_row(vec![
        "desk/",
        "Masaüstü yolu için bir kısa yoldur.",
        "desk/a.txt",
    ]);

    Ok((commands_table, short_cuts_table))
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>()[2..].to_vec();
    let parsed_args = ParseArgs::parse(&args).expect("parse error");

    match parsed_args {
        Some(parsed_args) => {
            let content = read_file(parsed_args.file);
            match FindWord::find(content, parsed_args.word) {
                Ok(result) => {
                    println!(
                        "\n{} tane {} bulundu.\n----------------------------------------------------------------",
                        result.count.cyan(),
                        result.word.cyan()
                    );
                    for line in result.result.lines() {
                        println!("{}", line)
                    }
                }
                Err(e) => println!("{:?}", e),
            }
        }
        None => {
            let (commands_table, short_cuts_table) = create_table().unwrap();
            println!("{}\n{}", commands_table, short_cuts_table)
        }
    }
}
