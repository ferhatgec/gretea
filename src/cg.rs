// MIT License
//
// Copyright (c) 2021-2022 Ferhat Geçdoğan All Rights Reserved.
// Distributed under the terms of the MIT License.
//
//

use {
    crate::{
        ast::{
            ast_helpers::{
                to,
                from_module
            }
        },
        parser::{GreteaParser},
        optimize::{
            OptimizeBlocks,
            optimizer
        }
    },
    std::collections::{BTreeMap}
};
use crate::ast::GreteaFunctionData;

pub struct GreteaCodegen {
    pub generated: String,

    pub sources  : BTreeMap<String, bool>,

    pub optimize : bool
}

impl GreteaCodegen {
    pub fn import(&mut self, subdirectory: String, is_include: bool) {
        if is_include {
            self.generated.push_str(
                &*format!("#{} {}\n", "include", subdirectory));
            return;
        }

        let subdirectories: Vec<String> = subdirectory.split('.')
            .map(|data| data.to_string()).collect();
        let mut dir = String::new();
        let mut is_stl = false;

        for lol in subdirectories {
            if lol == "tea" { is_stl = true; }

            dir.push_str(format!("{}/", lol).as_str());
        } dir.pop();

        dir = to(dir.trim());

        self.sources  .insert(dir.clone(), is_stl);
        self.generated.push_str(
            &*format!("#{}{}{}.hpp{}\n", "include",
                      if is_stl { '"' } else { '"' }, dir.replace('\n', "").split('/').last().unwrap(), if is_stl { '"' } else { '"' }));
    }

    pub fn function(&mut self,
                    parser    : &mut GreteaParser,
                    args      : &BTreeMap<String, String>,
                    name      : &String,
                    return_val: &String,
                    generic   : &String,
                    is_expand : bool,

                    is_void   : bool) {
        if !generic.is_empty() {
            self.generated.push_str(format!("template<typename{} {}>\n", if is_expand {
                "..."
            } else { "" }, generic).as_str());
        }

        parser.func_data.push(GreteaFunctionData {
            __function_name       : name.clone(),
            __function_return_type: return_val.clone(),
            __function_arguments  : args.clone()
        });

        let mut arguments = String::new();

        if *name == "main" {
            arguments = format!("int argc, char** argv");
        } else {
            for map in args.iter() {
                // ^^^ (name, type) -> (type, name)
                arguments.push_str(&*format!("{} {},", map.1.clone(), map.0.clone()));
            }
            arguments.pop();
        }

        self.generated.push_str(&*format!("{} {}({}) {}\n",
                                          return_val, name, arguments, if is_void {
                "{"
            } else { "" }));
    }

    pub fn function_call(&mut self, args: &Vec<String>, name: &String, is_unpack: bool) {
        let mut arguments = String::new();

        if name != &"main" {
            for arg in args.iter() {
                match arg.as_str() {
                    "+" |
                    "-" |
                    "/" |
                    "*" |
                    "%" |
                    "(" |
                    ")" |
                    "[" |
                    "]" => {  if arguments.chars().last().unwrap() == ',' { arguments.pop(); } },
                    _   => {}
                }

                arguments.push_str(&*format!("{},", arg));

                match arg.as_str() {
                    "+" |
                    "-" |
                    "/" |
                    "*" |
                    "%" |
                    "(" |
                    ")" |
                    "[" |
                    "]" => { if arguments.chars().last().unwrap() == ',' { arguments.pop(); } },
                    _   => {}
                }
            } if !arguments.is_empty() && arguments.chars().last().unwrap() == ',' { arguments.pop(); }
        }

        self.generated.push_str(&*format!("{}{}({}){};\n", if is_unpack {
            "("
        } else { "" }, name, arguments, if is_unpack {
            ", ...)"
        } else { "" }));
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

    pub fn statement(&mut self, tokens: &Vec<String>, is_while: bool) {
        if !is_while && tokens.last().unwrap() == "else" {
            self.generated.push_str(format!("{} {{\n", "else").as_str());
        } else {
            let mut is_else_if = false;

            let mut statement = String::from(
                format!("{} {}", tokens.get(0).unwrap(), if !is_while && tokens.get(1).unwrap() == "if" {
                    is_else_if = true;
                    "if("
                } else { "(" }));

            for token in
                if self.optimize {
                    optimizer::optimize(tokens, OptimizeBlocks::StatementBool)
                } else { tokens.clone() }.iter().skip( if is_else_if { 2 } else { 1 })  {
                statement.push_str(format!("{}", from_module(token)).as_str());
            }

            self.generated.push_str(format!("{}) {{\n", statement).as_str());
        }
    }

    pub fn statement_directive(&mut self, tokens: &Vec<String>) {
        if tokens.last().unwrap() == "else" {
            self.generated.push_str(format!("#{}\n", "else").as_str());
        } else {
            let mut is_else_if = false;

            let mut statement = String::from(
                format!("#{} ", if tokens.get(1).unwrap() == "if" {
                    is_else_if = true;
                    "elif"
                } else { "if" }));

            for token in tokens.iter().skip( if is_else_if { 2 } else { 1 }) {
                statement.push_str(format!("{}", token).as_str());
            }

            self.generated.push_str(format!("{}\n", statement).as_str());
        }
    }

    pub fn directive_end(&mut self) {
        self.generated.push_str(format!("#{}\n", "endif").as_str());
    }

    pub fn module(&mut self, module_name: &String) {
        self.generated.push_str(format!("namespace {}", module_name).as_str());
    }

    pub fn structure(&mut self, struct_name: &String, struct_generic: &String) {
        self.generated.push_str(format!("{}class {}\n",
                                        if !struct_generic.is_empty() {
                                            format!("template<typename {}>\n", struct_generic)
                                        } else { to("") }, struct_name).as_str());
    }

    pub fn enumeration(&mut self, name: &String, _type: &String, data: &String) {
        self.generated.push_str(format!("enum {} {} {{\n{}\n}};", name,
                                        if !_type.is_empty() {
                                            format!(": {}", _type.clone())
                                        } else { to("") }, data).as_str());
    }

    pub fn for_loop(&mut self) {
        self.generated.push_str("while(1)");
    }

    pub fn for_iter(&mut self, var_name: &String, var_iter: &String) {
        self.generated.push_str(format!("for(auto {} : {})", var_name, var_iter).as_str());
    }

    pub fn for_continue(&mut self) {
        self.generated.push_str("continue;\n");
    }

    pub fn for_break(&mut self) {
        self.generated.push_str("break;\n");
    }

    pub fn return_variable(&mut self, return_variable: &String) {
        self.generated.push_str(format!("return {};", if return_variable.is_empty() {
            ""
        } else { return_variable.as_str() }).as_str())
    }

    pub fn header_guards(&mut self) {
        self.generated.push_str(format!("#{} {}\n", "pragma", "once").as_str());
    }

    pub fn assembly(&mut self, data: &String) {
        let mut is_direct = false;

        // No compiler-specific.
        self.generated.push_str("asm(\n");

        for line in data.split('\n') {
            if is_direct {
                if line.trim() == "_?" { is_direct = false; continue; }

                self.generated.push_str(format!("{}\n", line).as_str()); continue;
            }

            if line.trim() == "?_" {
                is_direct = true; continue;
            }

            if line.len() >= 2 {
                self.generated.push_str(format!("\"{}\"\n", line).as_str());
            }
        }

        self.generated.push_str(");\n");
    }

    pub fn cpp_vector(&mut self, __type: &String) {
        self.generated.push_str(&*format!("std::vector<{}>", __type));
    }

    pub fn cpp_args(&mut self) {
        self.generated.push_str("std::vector<string> arguments(argv, argv + argc);\n");
    }

    pub fn character(&mut self, character: &String) {
        self.generated.push_str(&*format!("{}\n", character));
    }
}