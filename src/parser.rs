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

        let mut is_cpp_linker     = false; let mut cpp_block= String::new();

        let mut is_fn_call        = false;

        let (mut argument_name,
             mut argument_value) = (String::new(), String::new());

        let mut function_list: Vec<String> = Vec   ::new();
        let mut function_name       = String::new();
        let mut function_args: Vec<String> = Vec   ::new();

        for mut token in tokens {
            if token.is_empty() { continue; }

            matched_type = *self.init_ast.match_keyword(token);

            match matched_type {
                GreteaKeywords::Import => {
                    is_import = true; continue;
                },
                GreteaKeywords::Fn => {
                    is_fn = true; continue;
                },
                GreteaKeywords::Var => {
                    println!("found : var");
                },
                GreteaKeywords::Cpp => {
                    is_cpp_linker = true; continue;
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

                _ => {
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