// MIT License
//
// Copyright (c) 2021 Ferhat Geçdoğan All Rights Reserved.
// Distributed under the terms of the MIT License.
//
//

use {
    crate::{
        ast::{
            GreteaKeywords    ,

            GreteaSyntax      ,
            GreteaVariableData,
            GreteaVariableList,

            ast_helpers::{to}
        },
        cg::{
            CodegenData,
            GreteaCodegen
        }
    },
    std::collections::{HashMap}
};
use std::env::var;
use crate::ast::ast_helpers::from_module;

pub struct GreteaParser {
    pub init_ast : GreteaSyntax      ,
    pub data_list: GreteaVariableList,
    pub func_list: Vec<String>
}

impl GreteaParser {
    pub fn parse(&mut self, tokens: &Vec<String>) -> GreteaCodegen {
        let mut codegen = GreteaCodegen {
            generated: to(""),
            sources  : Default::default()
        };

        let mut matched_type = GreteaKeywords::Undefined;

        let mut is_import = false;

        let mut is_fn_data        = false;
        let mut is_fn             = false;
        let mut is_fn_name        = false; let mut fn_name   = String ::new();
        let mut is_generic        = false; let mut fn_generic= String::new();
        let mut is_expandable     = false;
        let mut is_fn_argument    = false; let mut fn_args: HashMap<String, String>
                                                                       = HashMap::new();
        let mut is_fn_return_value= false; let mut fn_val  = String ::new();
        let mut is_void           = false;

        let mut is_var            = false;
        let mut is_mutable        = false; let mut var_name = String::new();
        let mut is_var_data       = false; let mut var_data = String::new();
        let mut is_var_type       = false; let mut variable_type    = String::new();

        let mut is_cpp_linker     = false; let mut cpp_block= String::new();

        let mut is_fn_call        = false;

        let mut is_alias_replace  = false;
        let mut is_alias          = false; let mut alias_name = String::new();
        let mut is_alias_name     = false; let mut alias_data = String::new();

        let mut alias_list: HashMap<String, String>
                                        = HashMap::new();

        let (mut argument_name,
             mut argument_value) = (String::new(), String::new());

        let mut function_name       = String::new();
        let mut function_args: Vec<String> = Vec   ::new();

        let mut is_preprocessor   = false;
        let mut is_directive      = false;
        let mut is_set            = false;

        let mut is_statement      = false; let mut statement_data: Vec<String> = Vec::new();

        let mut is_module         = false; let mut module_name = String::new();

        let mut is_for            = false;
        let mut is_for_variable   = false;
        let mut is_for_in         = false; let mut for_var  = String::new();
        let mut is_for_iter       = false; let mut for_iter = String::new();

        let mut is_return         = false;

        let mut set_name = String::new();
        let mut set_data = String::new();

        for mut token in tokens {
            if token.is_empty() || token == " " || token == "\n" { continue; }

            if is_alias_replace {
                let get_alias_data = alias_list.get(token);

                if !get_alias_data.is_none() {
                    token = get_alias_data.unwrap();
                } is_alias_replace = false;
            }


            if token == "#"
                && !is_cpp_linker
                && !is_fn_call
                && !is_fn_name
                && !is_fn_argument {
                is_alias_replace = true; continue;
            }

            matched_type = *self.init_ast.match_keyword(&to(token));

            match matched_type {
                GreteaKeywords::Import => {
                    is_import = true; continue;
                },
                GreteaKeywords::Fn => {
                    is_fn = true; continue;
                },

                GreteaKeywords::Var => {
                    is_var = true; continue;
                },
                GreteaKeywords::Mut => {
                    if is_var {
                        is_mutable = true;
                    } continue;
                },

                GreteaKeywords::Cpp => {
                    is_cpp_linker = true; continue;
                },
                GreteaKeywords::Alias=> {
                    is_alias = true; continue;
                },
                GreteaKeywords::If |
                GreteaKeywords::Else => {
                    is_statement = true;
                    statement_data.push(token.clone()); continue;
                },

                GreteaKeywords::Module => {
                    is_module = true; continue;
                },

                GreteaKeywords::For => {
                    is_for = true; continue;
                },

                GreteaKeywords::LeftSqBracket => {
                    println!("found : [");
                },
                GreteaKeywords::RightSqBracket => {
                    println!("found : ]");
                },

                GreteaKeywords::RightCurlyBracket=> {
                    if is_directive { continue; }

                    if !is_cpp_linker {
                        codegen.character(&self.init_ast.ast_curly_right_bracket);
                    } else {
                        codegen.character(&cpp_block);
                        is_cpp_linker = false; cpp_block.clear();
                    } continue;
                },

                GreteaKeywords::Preprocessor => {
                    is_preprocessor = true; continue;
                },
                GreteaKeywords::Set          => {
                    if is_preprocessor {
                        is_set = true;
                    } continue;
                },

                GreteaKeywords::DirectiveEnd => {
                    if is_directive {
                        codegen.directive_end();
                        is_directive = false; continue;
                    }
                },

                _ => {
                    if is_statement {
                        if token == "{" {
                            if is_preprocessor {
                                codegen.statement_directive(&statement_data);

                                is_directive = true;
                            } else { codegen.statement(&statement_data); }

                            is_statement    = false;
                            is_preprocessor = false;
                            statement_data.clear(); continue;
                        }

                        statement_data.push(token.clone()); continue;
                    }

                    if is_module {
                        // if token == "{" { }
                        module_name = token.clone();

                        codegen.module(&module_name);

                        is_module = false;
                        module_name.clear(); continue;
                    }

                    if is_set {
                        if token == "=" { continue; }

                        if !set_name.is_empty() {
                            set_data = token.clone();

                            codegen.preprocess_set(&set_data, &set_name);

                            is_preprocessor = false;
                            is_set          = false;
                            set_name.clear(); set_data.clear(); continue;
                        }

                        set_name = token.clone(); continue;
                    }

                    if is_fn_call {
                        if token == "(" || token == ")" || token == "," {
                            if token == ")" {
                                codegen.function_call(&function_args,
                                                      &function_name, is_expandable);

                                is_fn_call    = false;
                                is_expandable = false;

                                function_name.clear(); function_args.clear();
                            }

                            continue;
                        }

                        match token.as_str() {
                            "+_" => {
                                is_expandable = true; continue;
                            },
                            _ => {
                                function_args.push(token.clone());
                            }
                        }
                    }

                    if is_import {
                        codegen.import(token.clone());

                        is_import = false; continue;
                    }

                    if is_fn {
                        if is_fn_name {
                            if token == "<" {
                                is_generic = true; continue;
                            }

                            if is_generic {
                                if token == ">" { is_generic = false; continue; }

                                if token == "'" {
                                    is_expandable = true; continue;
                                }

                                fn_generic = token.clone(); continue;
                            }

                            if token == "(" {
                                is_fn_argument = true; continue;
                            }


                            if is_fn_argument {
                                if token == ":" || token == "," {
                                    continue;
                                }

                                if token == "{"  {
                                    // if is_fn_return_value { }
                                    // ^^^^^^^^^^^^^^^^^^^^^^^^ error ( fn ...(....) = (no return type) {....}

                                    is_void = true;
                                }

                                if is_fn_return_value || is_void {
                                    fn_val = if is_fn_return_value { token.clone() } else { to("void") };

                                    codegen.function(&fn_args,
                                                     &fn_name.clone(),
                                                     &fn_val.clone(),
                                                     &fn_generic.clone(),
                                                     is_expandable,
                                                     is_void);

                                    fn_args.clear(); fn_val.clear();

                                    is_fn_data        = true ;
                                    is_fn             = false;
                                    is_fn_name        = false;
                                    is_generic        = false;
                                    is_expandable     = false;
                                    is_fn_argument    = false;
                                    is_fn_return_value= false;
                                    is_void           = false;

                                    fn_generic.clear(); continue;
                                }

                                if token == "=" {
                                    is_fn_return_value = true; is_void = false; continue;
                                }

                                if argument_name.is_empty() {
                                    argument_name = token.clone();

                                    continue;
                                }

                                if argument_value.is_empty() {
                                    argument_value = token.clone();

                                    if is_expandable {
                                        if argument_value == fn_generic {
                                            argument_value.push_str("...");
                                        }
                                    }

                                    continue;
                                }

                                if argument_value == ")" {
                                    argument_name.clear(); argument_value.clear(); continue;
                                }

                                fn_args.insert(argument_name.clone(), argument_value.clone());

                                argument_name.clear(); argument_value.clear(); continue;
                            }

                            if token == ")" {
                                is_void = true; continue;
                            }

                            is_fn = false; is_fn_name = false; continue;
                        }

                        fn_name    = token.split('#').last().unwrap().to_string();
                        is_fn_name = true;

                        self.func_list.push(fn_name.clone()); continue;
                    }

                    if token == "{" {
                        if !is_cpp_linker { codegen.character(&self.init_ast.ast_curly_left_bracket); }

                        continue;
                    }

                    if is_var {
                        if !var_name.is_empty() {
                            if is_var_type {
                                is_var_type = false;
                                variable_type = token.clone(); continue;
                            }

                            if !is_var_type {
                                if is_var_data {
                                    var_data = token.clone();

                                    codegen.variable_definition(&var_data, &variable_type, &var_name, is_mutable);

                                    self.data_list.variable_list.push(GreteaVariableData {
                                        __keyword_type: GreteaKeywords::Var,
                                        __name        : var_name.clone(),
                                        __data        : variable_type.clone()
                                    });

                                    is_var     = false;
                                    is_var_type= false;
                                    is_var_data= false;
                                    is_mutable = false;

                                    var_data     .clear();
                                    variable_type.clear();
                                    var_name     .clear(); continue;
                                }

                                if token == "=" {
                                    is_var_data = true; continue;
                                }
                            }

                            if token == ":" {
                                is_var_type = true; continue;
                            }
                        }

                        var_name = token.clone(); continue;
                    }

                    if is_for {
                        if is_for_variable {
                            if is_for_in {
                                for_iter    = token.clone();
                                is_for_iter = true;

                                codegen.for_iter(&for_var, &for_iter);

                                for_var.clear(); for_iter.clear();

                                is_for          = false;
                                is_for_variable = false;
                                is_for_in       = false;
                                is_for_iter     = false; continue;
                            }

                            if token == "in" {
                                is_for_in = true; continue;
                            }
                        }

                        for_var         = token.clone();
                        is_for_variable = true; continue;
                    }

                    if is_alias {
                        if is_alias_name {
                            if token == "=" { continue; }

                            alias_data    = from_module(&to(token.clone().trim_end()));

                            if is_preprocessor {
                                codegen.preprocess_set(&alias_data, &alias_name);
                                is_preprocessor = false;
                            }
                            else { alias_list.insert(alias_name.clone(), alias_data.clone()); }

                            is_alias        = false; alias_name.clear();
                            is_alias_name   = false; alias_data.clear(); continue;
                        }

                        alias_name = token.clone(); is_alias_name = true;

                        continue;
                    }

                    if is_cpp_linker {
                        cpp_block.push_str(&*token); continue;
                    }

                    let function_token = token.clone().split('#').last().unwrap().to_string();

                    for name in self.func_list.clone() {
                        if name.split('#').last().unwrap().trim() == function_token.trim() {
                            is_fn_call = true;
                            function_name = from_module(&token); break;
                        }
                    }

                    if is_fn_data {
                        if token == "." || token == "return" {
                            is_return = true; continue;
                        }

                        if is_return {
                            is_return = false;

                            for variable in &self.data_list.variable_list {
                                if token == &variable.__name {
                                    codegen.return_variable(&variable.__data); break;
                                }
                            }

                            if token.trim_end() == "_" {
                                codegen.return_variable(&to(""));
                            } else { codegen.return_variable(&token.clone()); }
                        }

                        continue;
                    }
                }
            }
        }

        codegen
    }
}