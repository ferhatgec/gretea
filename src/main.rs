// MIT License
//
// Copyright (c) 2021 Ferhat Geçdoğan All Rights Reserved.
// Distributed under the terms of the MIT License.
//
//

use std::{
    io::{
        Write,
        Error
    },
    process::{Output}
};

mod read;
mod tokenizer;
mod lexer;
mod ast;
mod parser;
mod cg;

fn main() {
    let commandline_arguments: Vec<_> = std::env::args().collect();

    if commandline_arguments.len() < 2 {
        println!("Gretea compiler");

        std::process::exit(1);
    }

    let mut filename = commandline_arguments.last().unwrap();

    let mut gretea_read = read::GreteaFileData {
        raw_data: "".to_string(),
        unparsed: vec![]
    };

    gretea_read.read_raw_file(filename);

    let generated          = lexer::gretea_lexer::init_lexer(&gretea_read);
    let object_name        = filename.replace(".tea", "");
    let generated_filename = format!("{}.cpp", object_name);

    let mut path           = std::path::Path::new(&generated_filename);

    let mut file = match std::fs::File::create(path) {
        Err(why) => panic!("gretea: couldn't create {}: {}", path.display(), why),
        Ok(file) => file
    };

    match file.write_all(generated.as_bytes()) {
        Err(why) => panic!("gretea: couldn't write to {}: {}", path.display(), why),
        Ok(file) => println!("gretea: success: {}", path.display())
    }

    let build_output = std::process::Command::new("c++")
        .arg("-std=c++17")
        .arg(generated_filename)
        .arg("-o")
        .arg(object_name)
        .status();

    //match build_output {
    //    Err(err) => panic!("gretea: build error: output:\n{}", err),
    //    _ => {}
    //}
}
