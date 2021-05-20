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

fn to_gretea(filename: String) -> String {
    format!("{}.tea", filename)
}

fn normalize(filename: String) -> String {
    filename.replace(".tea", "")
}

fn add(filename: String) -> String {
    format!("{}.cpp", filename)
}

fn header(filename: String) -> String {
    format!("{}.hpp", filename)
}

fn create_and_write(path: &std::path::Path, generated: String) {
    let mut file = match std::fs::File::create(path) {
        Err(why) => panic!("gretea: couldn't create {}: {}", path.display(), why),
        Ok(file) => file
    };

    match file.write_all(generated.as_bytes()) {
        Err(why) => panic!("gretea: couldn't write to {}: {}", path.display(), why),
        Ok(file) => println!("gretea: success: {}", path.display())
    }
}

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

    let (generated, files) = lexer::gretea_lexer::init_lexer(&gretea_read);
    let mut object_name        = normalize(filename   .clone());
    let mut generated_filename = add      (object_name.clone());

    let mut path           = std::path::Path::new(&generated_filename);

    create_and_write(path, generated);

    for file in files {
        if !file.1 {
            gretea_read.read_raw_file(&to_gretea(file.0.clone()));
            let (generated_data, _) = lexer::gretea_lexer::init_lexer(&gretea_read);
            let object_name         = header(normalize(file.0));
            path                           = std::path::Path::new(&object_name);

            create_and_write(path, generated_data);
        }
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
