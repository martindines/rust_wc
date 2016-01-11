extern crate getopts;

use getopts::Options;
use std::env;
use std::io::prelude::*;
use std::fs::OpenOptions;

fn get_file_contents(filename: &String) -> String {
    let open_file = OpenOptions::new().read(true).open(filename);
    
    let mut file = match open_file {
        Ok(val) => val,
        Err(err) => { panic!(err.to_string()) }
    };

    let mut buffer = String::new();
    file.read_to_string(&mut buffer).unwrap();

    buffer
}

fn get_newline_count(string: &String) -> usize {
    let count = string.lines().count();
    count
}

fn get_word_count(string: &String) -> usize {
    let words: Vec<&str> = string.split(&[' ', '\n'][..]).filter(|&c| !c.is_empty()).collect();
    let count = words.iter().count();
    count
}

fn get_byte_count(string: &String) -> usize {
    let count = string.bytes().count();
    count
}

fn write_output(lines: &usize, words: &usize, bytes: &usize, name: &String, display_lines: &bool, display_words: &bool, display_bytes: &bool) {
    let column_size: usize = 8;

    if *display_lines {
        print!("{:1$}", lines, column_size);
    }

    if *display_words {
        print!("{:1$}", words, column_size);
    }

    if *display_bytes {
        print!("{:1$}", bytes, column_size);
    }
    
    println!(" {}", name);
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [-clmw] [FILE ...]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let mut display_lines: bool = true;
    let mut display_words: bool = true;
    let mut display_bytes: bool = true;

    let mut opts = Options::new();
    opts.optflag("c", "", "The number of bytes in each input file is written to the standard output.");
    opts.optflag("l", "", "The number of lines in each input file is written to the standard output.");
    opts.optflag("w", "", "The number of words in each input file is written to the standard output.");
    opts.optflag("h", "help", "print this help menu");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    if matches.opts_present(&["c".to_owned(), "l".to_owned(), "w".to_owned()]) {
        display_bytes = matches.opt_present("c");
        display_lines = matches.opt_present("l");
        display_words = matches.opt_present("w");
    }

    let files = if !matches.free.is_empty() {
        matches.free.clone()
    } else {
        print_usage(&program, opts);
        return;
    };

    let mut total_lines: usize = 0;
    let mut total_words: usize = 0;
    let mut total_bytes: usize = 0;

    for file in &files {
        let contents = get_file_contents(&file);

        let lines = get_newline_count(&contents);
        let words = get_word_count(&contents);
        let bytes = get_byte_count(&contents);

        write_output(&lines, &words, &bytes, &file, &display_lines, &display_words, &display_bytes);

        total_lines += lines;
        total_words += words;
        total_bytes += bytes;
    }

    if files.len() > 1 {
        let name = "total".to_string();
        write_output(&total_lines, &total_words, &total_bytes, &name, &display_lines, &display_words, &display_bytes);
    }
}
