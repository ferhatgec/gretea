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

pub struct GreteaParser {
    pub init_ast : GreteaSyntax,
    pub data_list: GreteaVariableList
}

impl GreteaParser {
    pub fn parse(&mut self, tokens: &Vec<String>) -> String {
        let mut codegen = GreteaCodegen {
            generated: to(""),
            sources  : Default::default()
        };

        let mut matched_type = GreteaKeywords::Undefined;

        let mut is_import = false;

        let mut is_fn             = false;
        let mut is_fn_name        = false; let mut fn_name = String ::new();
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

        let mut function_list: Vec<String> = Vec   ::new();
        let mut function_name       = String::new();
        let mut function_args: Vec<String> = Vec   ::new();

        let mut is_preprocessor   = false;
        let mut is_set            = false;

        let mut set_name = String::new();
        let mut set_data = String::new();

        for mut token in tokens {
            if token.is_empty() || token == " " { continue; }

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

            matched_type = *self.init_ast.match_keyword(token);

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

                //GreteaKeywords::LeftParenthese => {
                //    println!("found : (");
                //},
                //GreteaKeywords::RightParenthese => {
                //    println!("found : )");
                //},

                GreteaKeywords::LeftSqBracket => {
                    println!("found : [");
                },
                GreteaKeywords::RightSqBracket => {
                    println!("found : ]");
                },

                GreteaKeywords::RightCurlyBracket=> {
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

                _ => {
                    if is_set {
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
                                is_fn_call = false;

                                codegen.function_call(&function_args, &function_name);

                                function_name.clear(); function_args.clear();
                            }

                            continue;
                        }

                        function_args.push(token.clone());
                    }

                    if is_import {
                        codegen.import(token.clone());

                        is_import = false; continue;
                    }

                    if is_fn {
                        if is_fn_name {
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

                                    codegen.function(&fn_args, &fn_name.clone(), &fn_val.clone());

                                    fn_args.clear(); fn_val.clear();

                                    is_fn             = false;
                                    is_fn_name        = false;
                                    is_fn_argument    = false;
                                    is_fn_return_value= false;
                                    is_void           = false; continue;
                                }

                                if token == "=" {
                                    is_fn_return_value = true; is_void = false; continue;
                                }

                                if argument_name.is_empty() {
                                    argument_name = token.clone(); continue;
                                }

                                if argument_value.is_empty() {
                                    argument_value = token.clone();
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

                        fn_name    = token.clone();
                        is_fn_name = true;

                        function_list.push(fn_name.clone()); continue;
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

                    if is_alias {
                        if is_alias_name {
                            if token == "=" { continue; }

                            alias_data    = to(token.clone().trim_end());

                            if is_preprocessor {
                                codegen.preprocess_set(&alias_data, &alias_name);
                            }
                            else { alias_list.insert(alias_name.clone(), alias_data.clone()); }

                            is_alias      = false; alias_name.clear();
                            is_alias_name = false; alias_data.clear();

                            continue;
                        }

                        alias_name = token.clone(); is_alias_name = true;

                        continue;
                    }

                    if is_cpp_linker {
                        cpp_block.push_str(&*token); continue;
                    }

                    for name in function_list.clone() {
                        if &name == token {
                            is_fn_call = true;

                            function_name = name;

                            break;
                        }
                    }
                }
            }
        }

        codegen.generated
    }
}