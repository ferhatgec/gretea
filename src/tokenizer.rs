// MIT License
//
// Copyright (c) 2021 Ferhat Geçdoğan All Rights Reserved.
// Distributed under the terms of the MIT License.
//
//

pub mod gretea_tokenizer {
    use crate::{
        ast::ast_helpers::{to},
        read::{GreteaFileData}
    };

    pub static TOKEN_LIST: &'static [char] = &[
        '`' ,
        '(' ,
        ')' ,
        '[' ,
        ']' ,
        '{' ,
        '}' ,
        ':' ,
        '=' ,
        '<' ,
        '>' ,
        '\'',
        ',' ,
        '+' ,
        '-' ,
        '/' ,
        '%' ,
        '@'
    ];

    pub fn tokenize(raw_data: &GreteaFileData) -> Vec<String> {
        let temporary_tokens: Vec<_> = raw_data.raw_data.split(' ').collect();
        let mut tokenized_data: Vec<String> = Vec::new();
        let mut variable_data = String::new();
        let mut optional = String::new();
        let mut found_data = false;

        let mut i: usize = 0;

        let mut last_data: &str = "";

        let mut is_vect = false;

        let mut is_data = false;
        let mut is_seq = false;
        let mut data = String::new();

        while i < temporary_tokens.len() {
            if temporary_tokens[i].contains("[]") && temporary_tokens[i].starts_with('[') {
                let data: Vec<_> = temporary_tokens[i].split("[]").collect();

                if data.len() == 2 { // [] type or type []
                    tokenized_data.push(to("[]"));
                    tokenized_data.push(to(data.last().unwrap()));

                    i += 1;

                    continue;
                }
            }

            if temporary_tokens[i] == "[[" || temporary_tokens[i].trim_end() == "]]" {
                tokenized_data.push(temporary_tokens[i].to_string());
                
                i += 1; continue;
            }
            else if temporary_tokens[i].starts_with('\'') && temporary_tokens[i].trim_end().ends_with('\'') {
                // ' x ' -> 'x'
                tokenized_data.push(temporary_tokens[i].to_string()); i += 1; continue;   
            }

            if found_data {
                variable_data.push_str(format!("{} ", temporary_tokens[i]).as_str());

                if variable_data.ends_with(',') ||
                    variable_data.ends_with('(') ||
                    variable_data.ends_with(')') {
                    optional = variable_data.chars().last().unwrap().to_string(); variable_data.pop();
                }

                if is_end_of_data(&temporary_tokens[i]) {
                    found_data = false;
                    tokenized_data.push(variable_data.clone());
                    tokenized_data.push(optional.clone());

                    variable_data.clear(); optional.clear();
                } i += 1; continue;
            }

            if is_start_of_data(&temporary_tokens[i]) {
                if !is_end_of_data(&temporary_tokens[i]) {
                    found_data = true;

                    variable_data.push_str(format!("{} ", temporary_tokens[i]).as_str());
                }
                else {
                    let mut data = to(temporary_tokens[i].trim());
                    let mut ends_with = false;
                    if data.ends_with(',') ||
                        data.ends_with('(') ||
                        data.ends_with(')') {
                        optional = data.chars().last().unwrap().to_string(); data.pop(); ends_with = true;
                    }

                    tokenized_data.push(data);
                    if ends_with { tokenized_data.push(optional.clone());
                    } optional.clear();
                }

                i += 1;

                continue;
            }

            let mut token             = String::from(replace(&temporary_tokens[i].to_string()));
            let mut retokenize: Vec<_> = token.split(' ').collect::<Vec<&str>>();

            let mut is_unpack = false;

            i += 1;

            for tokens in retokenize {
                if is_unpack {
                    is_unpack = false;

                    if tokens == "_" {
                        tokenized_data.push(to("+_")); continue;
                    } else {
                        tokenized_data.push(to("+"));
                    }
                }

                if !is_unpack && tokens == "+" { is_unpack = true; continue; }

                if is_data {
                    if tokens.contains("\\x1b") {
                        is_seq = true;
                    }

                    if tokens.trim_end().ends_with('"') {
                        is_data = false;
                        data.push_str(tokens);
                        tokenized_data.push(data.clone()); data.clear();
                    } else {
                        data.push_str(&*format!("{}{}", tokens.clone(), if is_seq {
                            if tokens == "[" { is_seq = false; }
                            ""
                        } else { " " }));
                    } continue;
                }

                if tokens.trim_start().starts_with('"') && !tokens.trim_end().ends_with('"') {
                    is_data = true;
                    data.push_str(&*format!("{}{}", tokens.clone(), if tokens[1..] != *"\\x1b" {
                        " "
                    } else { is_seq = true; "" })); continue;
                }

                tokenized_data.push(to(tokens));
            }
        }

        tokenized_data
    }

    pub fn get_data  (tokens: &Vec<&str>, n: usize) -> (String, usize) {
        let mut temporary = String::new();
        let mut i        : usize = 0;

        for (index, token) in tokens.iter().enumerate().skip(n) {
            i = index; if token.is_empty() { continue; }

            temporary.push_str(
                format!("{} ", token).as_str());

            if !is_data(token) { continue; }

            break;
        }

        (temporary, i)
    }

    pub fn replace   (token: &String) -> String {
        let mut token = String::from(token);

        for character in TOKEN_LIST {
            token = replace_with(&token, *character);
        }

        token
    }

    pub fn is_data   (token: &&str) -> bool {
        return if is_start_of_data(token) || is_end_of_data(token) {
            true
        } else { false };
    }

    pub fn is_start_of_data(token: &&str) -> bool {
        return if token.trim_start().starts_with('"') {
            true
        } else { false };
    }

    pub fn is_end_of_data(token: &&str) -> bool {
        let mut token = to(token.trim_end());

        if token.ends_with(',') { token.pop(); }

        return if token.ends_with('"') {
            true
        } else { false };
    }

    pub fn is_comment(token: &&str) -> bool {
        if token.len() < 2 { return false; }

        let token = token.trim();

        return if token.starts_with('/') && token.chars().nth(1).unwrap() == '/' {
            true
        } else { false };
    }

    pub fn replace_with(token: &String, character: char) -> String {
        token.replace(character, format!(" {} ", character).as_str())
    }
}