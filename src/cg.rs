// MIT License
//
// Copyright (c) 2021 Ferhat Geçdoğan All Rights Reserved.
// Distributed under the terms of the MIT License.
//
//

use {
    crate::{
        ast::{GreteaKeywords},
        tokenizer::gretea_tokenizer::{TOKEN_LIST}
    },
    std::collections::{HashMap}
};

pub struct GreteaCodegen {
    pub generated: String,

    pub sources  : HashMap<String, bool>
}

pub struct CodegenData {
    pub return_type: String,
    pub arguments  : Vec<String>
}

impl GreteaCodegen {
    pub fn import(&mut self, subdirectory: String) {
        let subdirectories: Vec<String> = subdirectory.split('.')
            .map(|data| data.to_string()).collect();
        let mut dir = String::new();
        let mut is_stl = false;

        for lol in subdirectories {
            if lol == "tea" { is_stl = true; }

            dir.push_str(format!("{}/", lol).as_str());
        } dir.pop();

        self.sources  .insert(dir.clone(), is_stl);
        self.generated.push_str(
            &*format!("#{}{}{}.hpp{}\n", "include",
                      if is_stl { '<' } else { '"' }, dir.replace('\n', ""), if is_stl { '>' } else { '"' }));
    }

    pub fn function(&mut self, args: &HashMap<String, String>, name: &String, return_val: &String) {
        let mut arguments = String::new();

        for    map in args.iter() {
            // ^^^ (name, type) -> (type, name)
            arguments.push_str(&*format!("{} {},", map.1.clone(), map.0.clone()));
        } arguments.pop();

        self.generated.push_str(&*format!("{} {}({})\n", return_val, name, arguments));
    }

    pub fn function_call(&mut self, args: &Vec<String>, name: &String) {
        let mut arguments = String::new();

        if name != &"main" {
            for arg in args.iter() {
                arguments.push_str(&*format!("{},", arg));
            } arguments.pop();
        }

        self.generated.push_str(&*format!("{}({});\n", name, arguments));
    }

    pub fn variable_definition(&mut self, data: &String, var_type: &String, name: &String, is_mut: bool) {
        let __data = format!("= {};", data).clone();

        self.generated.push_str(&*format!("{} {} {} {}\n", if !is_mut {
            "const"
        } else { "" }, if var_type.is_empty() {
            "auto"
        } else { var_type }, name, if data.is_empty() {
            ";"
        } else {
            __data.as_str()
        }));
    }

    pub fn preprocess_set(&mut self, data: &String, name: &String) {
        self.generated.push_str(&*format!("#define {} {}\n", name, if !data.is_empty() {
            data
        } else { "0" }));
    }

    pub fn character(&mut self, character: &String) {
        self.generated.push_str(&*format!("{}\n", character));
    }
}