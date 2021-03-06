// MIT License
//
// Copyright (c) 2021 Ferhat Geçdoğan All Rights Reserved.
// Distributed under the terms of the MIT License.
//
//

extern crate elite;

use std::{
    io::{
        Write
    },
    process::{
        Command,
        exit
    }
};

mod read;
mod tokenizer;
mod lexer;
mod ast;
mod parser;
mod cg;
mod optimize;
mod log;

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
        Ok(_) => println!("gretea: success: {}", path.display())
    }
}

fn remove_and_check(filename: &str) {
    match std::fs::remove_file(filename) {
        Err(_) => panic!("gretea: failed to remove {}", filename),
        _ => {}
    }
}

fn main() {
    //#[cfg(target_os = "windows")] std::process::exit(1);

    let commandline_arguments: Vec<_> = std::env::args().collect();

    if commandline_arguments.len() < 2 {
        println!("Gretea compiler");

        exit(1);
    }

    let filename       = commandline_arguments.get(1).unwrap();

    let mut gretea_read = read::GreteaFileData::default();

    gretea_read.read_raw_file(filename);

    let (_, files, _, _comp_list) = lexer::gretea_lexer::init_lexer(gretea_read.clone());
    let object_name = normalize(filename   .clone());
    let generated_filename = add(object_name.clone());

    let path           = std::path::Path::new(&generated_filename);

    for file in files.clone() {
        if !file.1 {
            gretea_read.read_raw_file(&to_gretea(file.0.clone()));
            let (generated_data, _, mut func, mut _comp_list) = lexer::gretea_lexer::init_lexer(gretea_read.clone());
            let object_name         = header(normalize(file.0));

            gretea_read.func_list.append(&mut func);
            gretea_read.comp_list.append(&mut _comp_list);

            create_and_write(std::path::Path::new(&object_name), generated_data);
        }
        else {
            gretea_read.read_raw_file(&to_gretea(
                format!("/usr/include/gretea/{}", file.0.clone())));
            let (generated_data, _, mut func, mut _comp_list) = lexer::gretea_lexer::init_lexer(gretea_read.clone());
            let object_name         = header(normalize(file.0.split('/').last().unwrap().parse().unwrap()));

            gretea_read.func_list.append(&mut func);
            gretea_read.comp_list.append(&mut _comp_list);

            create_and_write(std::path::Path::new(&object_name), generated_data);
        }
    }

    gretea_read.read_raw_file(filename);

    let (generated, _, _, _) = lexer::gretea_lexer::init_lexer(gretea_read.clone());

    create_and_write(path, generated);

    let mut build_output = Command::new("c++");

    build_output.arg("-std=c++17")
        .arg(generated_filename.clone()).arg("-o").arg(object_name);

    if commandline_arguments.len() > 2 {
        for arg in commandline_arguments.iter().skip(2) {
            build_output.arg(arg);
        }
    }

    if build_output.status().unwrap().success() {
        remove_and_check(generated_filename.as_str());

        for file in files.to_owned() {
            remove_and_check(header(normalize(file.0.split('/')
                .last()
                .unwrap()
                .parse()
                .unwrap())).as_str());
        }
    }
}
