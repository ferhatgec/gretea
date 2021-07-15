// MIT License
//
// Copyright (c) 2021 Ferhat Geçdoğan All Rights Reserved.
// Distributed under the terms of the MIT License.
//
//

use {
    std::io::BufRead,

    crate::{
        tokenizer::gretea_tokenizer::{is_comment}
    }
};

pub struct GreteaFileData {
    pub raw_data : String,
    pub unparsed : Vec<String>,
    pub lines    : Vec<String>,
    pub func_list: Vec<String>
}

impl GreteaFileData {
    pub fn read_raw_file(&mut self, file: &String) {
        let mut raw_data = String::new();

        if let Ok(lines) = self.read_lines(file) {
            for line in lines {
                if let Ok(ip) = line {
                    if is_comment(&ip.as_str()) { continue; }

                    raw_data.push(' '); raw_data.push_str(&ip); raw_data.push('\n');
                    self.lines.push(ip);
                }
            }
        }

        self.raw_data = raw_data;
    }

    fn read_lines<P>(&self, file: &P) -> std::io::Result<
        std::io::Lines<std::io::BufReader<std::fs::File>>
    > where P: AsRef<std::path::Path>, {
        Ok(std::io::BufReader::new(
            std::fs::File::open(file)?).lines())
    }
}