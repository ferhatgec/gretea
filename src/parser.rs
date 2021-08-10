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
            GreteaFunctionData,
            GreteaVariableList,

            GreteaCompileType,
            GreteaCompileData,

            ast_helpers::{
                to,
                from_module,
                make_vector
            }
        },
        cg::{GreteaCodegen},
        log::{
            LogTypes::{*},
            log::{gen}
        },
        tokenizer::gretea_tokenizer::{is_data},
        read::{GreteaFileData},
        elite::ast::ast_helpers::{extract_argument}
    },
    std::{
        collections::{
            HashMap,
            BTreeMap
        }
    }
};
use elite::logger::elite_logger::log;

pub struct GreteaParser {
    pub init_ast    : GreteaSyntax      ,
    pub data_list   : GreteaVariableList,
    pub func_data   : Vec<GreteaFunctionData>,
    pub raw_data    : GreteaFileData,
    pub func_list   : Vec<String>,
    pub compile_list: Vec<GreteaCompileData>
}

impl GreteaParser {
    pub fn parse(&mut self, tokens: &Vec<String>) -> GreteaCodegen {
        let mut codegen = GreteaCodegen {
            generated: to(""),
            sources  : Default::default(),
            optimize : true
        };

        let mut matched_type;

        let mut current_line = 1u32;
        let mut current_column = 0u32;

        let mut is_import = false;

        let mut is_fn_data = false;
        let mut is_fn = false;
        let mut is_fn_name = false; let mut fn_name = String ::new();
        let mut is_generic = false; let mut fn_generic = String::new();
        let mut is_expandable = false;
        let mut is_fn_argument = false; let mut fn_args: BTreeMap<String, String>
                                                                       = BTreeMap::new();

        let mut is_fn_return_value = false; let mut fn_val;
        let mut is_void = false;

        let mut is_var = false;
        let mut is_mutable = false; let mut var_name = String::new();
        let mut is_var_data = false; let mut var_data = String::new();
        let mut is_var_type = false; let mut variable_type = String::new();

        let mut is_var_data_vector = false;

        let mut is_var_struct = false;
        let mut var_struct_init_name = String::new();
        let mut var_struct_init_data = String::new();

        let mut is_inline_asm = false; let mut asm_block = String::new();
        let mut is_cpp_linker = false; let mut cpp_block = String::new();
        let mut is_runtime = false; let mut runtime_block = String::new();

        let mut is_fn_call = false;

        let mut is_alias_replace = false;
        let mut is_alias = false; let mut alias_name = String::new();
        let mut is_alias_name = false; let mut alias_data;

        let mut is_type = false;
        let mut type_name = String::new();

        let mut alias_list: HashMap<String, String> = HashMap::new();

        let (mut argument_name,
             mut argument_value) = (String::new(), String::new());

        let mut function_name = String::new();
        let mut function_args: Vec<String> = Vec::new();

        let mut is_pretty_arg = false;
        let mut pretty_arg = String::new();

        let mut is_preprocessor = false;
        let mut is_directive = false;
        let mut is_set = false;

        let mut is_statement = false;
        let mut statement_data: Vec<String> = Vec::new();

        let mut is_module = false; let mut module_name;

        let mut is_struct = false; let mut struct_name = String::new();
        let mut is_struct_generic = false; let mut struct_generic = String::new();

        let mut is_struct_member = false;

        let mut struct_list: Vec<String> = Vec::new();

        let mut struct_member_name = String::new();
        let mut struct_member_type = String::new();
        let mut struct_member_default = String::new(); let mut is_default = false;
        let mut struct_member_immutable = true;

        let mut is_enum = false;
        let mut is_enum_type = false;
        let mut is_enum_data = false;
        let mut enum_name = String::new();
        let mut enum_type = String::new();
        let mut enum_data = String::new();

        let mut is_for = false;
        let mut is_for_variable = false;
        let mut is_for_in = false; let mut for_var = String::new();
        let mut for_iter;

        let mut is_while = false;

        let mut is_compile = false;
        let mut is_compile_data = false;
        let mut compile_config = GreteaCompileType::Undefined;
        let mut compile_type = String::new();
        let mut compile_data = String::new();
        let mut count_compile_parentheses = 0u32;

        let mut is_return = false; let mut return_val = String::new();
        // let mut is_library = false;
        let mut is_library_setter = false;

        let mut is_unsafe = false;
        let mut is_safe = false;

        let mut is_vector = false;
        // let mut is_func_vector = false;


        let mut vector_type;

        let mut set_name = String::new();
        let mut set_data;


        let mut is_element = false;

        let mut is_line = false; let mut line     = String::new();

        let mut count_parentheses: u64 = 0;

        let mut is_get_data = false;
        let mut get_data = String::new();

        for mut token in tokens.clone() {
            current_column += 1;

            if token.is_empty() || token == " " || token == "\n" {
                if token == "\n" {
                    current_line += 1;
                } continue;
            } if token.ends_with('\n') { current_line += 1; }

            if is_get_data {
                get_data.push_str(token.clone().as_str());

                if token.trim_end().ends_with('"') {
                    is_get_data = false;
                    token = get_data.clone(); get_data.clear();
                } else { continue; }
            }
            else if token.starts_with('"') {
                if !token.trim_end().ends_with('"') {
                    is_get_data = true; get_data.push_str(token.clone().as_str()); continue;
                }
            }

            if is_alias_replace {
                let get_alias_data = alias_list.get(token.as_str());

                if !get_alias_data.is_none() {
                    token = get_alias_data.unwrap().clone();
                } is_alias_replace = false;
            }

            if token == "#"
                && !is_cpp_linker
                && !is_fn_call
                && !is_fn_name
                && !is_fn_argument {
                is_alias_replace = true; continue;
            }

            if is_type {
                for val in &self.data_list.variable_list {
                    if val.__name == token {
                        token = format!("\"{}\"", val.__type);
                        is_type = false;
                        break;
                    }
                }
            }

            matched_type = *self.init_ast.match_keyword(&token);

            if is_element && matched_type != GreteaKeywords::RightSqBracket {
                pretty_arg.push_str(token.as_str());
                continue;
            }

            if matched_type == GreteaKeywords::Type {
                if !is_fn_call && !is_var_data {
                    matched_type = GreteaKeywords::Undefined;
                }
            }

            if matched_type == GreteaKeywords::FlagRight
                && is_default {
                    if is_var_data {
                        let variable_type = to(variable_type.trim_end());
                        for arg in &self.compile_list {
                            if arg.__name == variable_type {
                                token = arg.__data.clone();
                                break;
                            }
                        }
                    } else if is_return {
                        let __type = &self.func_data.last().unwrap().__function_return_type;

                        for arg in &self.compile_list {
                            if arg.__name == *__type {
                                token = arg.__data.clone();
                                break;
                            }
                        }
                    }

                    is_library_setter = false;
                    is_default = false;
                    matched_type = *self.init_ast.match_keyword(&token);
            }

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
                    }
                    else if is_struct_member {
                        struct_member_immutable = false;
                    } continue;
                },

                GreteaKeywords::Assembly => {
                    is_inline_asm = true; continue;
                },
                GreteaKeywords::Cpp => {
                    is_cpp_linker = true; continue;
                },
                GreteaKeywords::Runtime => {
                    is_runtime = true; continue;
                },
                GreteaKeywords::Unsafe => {
                    if is_var && is_var_data {
                        codegen.variable_definition(&var_data, &variable_type, &var_name, is_mutable);

                        self.data_list.variable_list.push(GreteaVariableData {
                            __keyword_type: GreteaKeywords::Var,
                            __name        : var_name.clone(),
                            __type        : variable_type.clone(),
                            __data        : var_data.clone()
                        });

                        is_var     = false;
                        is_var_type= false;
                        is_var_data= false;
                        is_mutable = false;

                        var_data     .clear();
                        variable_type.clear();
                        var_name     .clear();
                    } else {
                        is_unsafe = true;
                    } continue;
                },
                GreteaKeywords::Safe => {
                    if is_unsafe {
                        is_safe = true;
                        is_unsafe = false;
                    }
                },

                GreteaKeywords::Alias=> {
                    is_alias = true; continue;
                },
                GreteaKeywords::Type => {
                    //if is_preprocessor {
                        is_type = true;
                    //}
                },

                GreteaKeywords::If |
                GreteaKeywords::Else |
                GreteaKeywords::While => {
                    is_statement = true;

                    if matched_type == GreteaKeywords::While {
                        is_while = true;
                    }

                    statement_data.push(token.clone()); continue;
                },

                GreteaKeywords::Compile => {
                    is_compile = true;
                },

                GreteaKeywords::Module => {
                    is_module = true; continue;
                },
                GreteaKeywords::Struct => {
                    if !is_var { 
                        is_struct = true; 
                    } else {
                        if is_var_type {
                            variable_type.push_str(&*token.clone());
                            variable_type.push(' ');
                        } else if is_var_data {
                            var_data.push_str(&*token.clone());
                            var_data.push(' ');
                        }
                    }
                    continue;
                },
                GreteaKeywords::Enum => {
                    if !is_fn_data {
                        is_enum = true;
                    }
                },

                GreteaKeywords::For => {
                    if !is_runtime && !is_cpp_linker && !is_compile { is_for = true; continue; }

                    if is_compile {
                        continue;
                    } else if is_runtime {
                        runtime_block.push_str("for ");
                    } else if is_cpp_linker {
                        cpp_block.push_str("for ");
                    }
                },
                GreteaKeywords::Continue => {
                    codegen.for_continue();
                },
                GreteaKeywords::Break => {
                    codegen.for_break();
                },

                GreteaKeywords::LeftSqBracket => {
                    if is_line {
                        codegen.character(&to("["));
                    } else if is_statement {
                        statement_data.push(to("["));
                    } else if is_fn_call {
                        let last = function_args.last().unwrap().clone();
                        function_args.pop();
                        pretty_arg.push_str(format!("{}[", last).as_str());
                        is_element = true;
                    }

                    //if is_library { is_library_setter = true; continue; }

                    //is_library = true; continue;
                },
                GreteaKeywords::RightSqBracket => {
                    if is_line {
                        codegen.character(&to("]"));
                    } else if is_statement {
                        statement_data.push(to("]"));
                    } else if is_fn_call {
                        pretty_arg.push(']');
                        function_args.push(pretty_arg.clone());
                        pretty_arg.clear();
                        is_element = false;
                    }
                    //if is_library || is_library_setter {
                    //    is_library        = false;
                    //   is_library_setter = false;
                    //} continue;
                },

                GreteaKeywords::RightCurlyBracket=> {
                    if is_directive { continue; }

                    if is_compile_data {
                        count_compile_parentheses -= 1;
                        
                        if count_compile_parentheses == 0 {
                            let __compile_type = to(compile_type.trim_end());
                            let mut found = false;

                            for arg in &mut self.compile_list {
                                if arg.__name == __compile_type {
                                    arg.__data = compile_data.clone();
                                    found = true;
                                    break;
                                }
                            }

                            if !found {
                                self.compile_list.push(GreteaCompileData {
                                    __type: compile_config.clone(),
                                    __name: to(compile_type.trim_end()),
                                    __data: compile_data.clone()
                                });
                            }

                            is_compile = false;
                            is_compile_data = false;
                            compile_config = GreteaCompileType::Undefined;
                            compile_data.clear();
                            compile_type.clear();
                        } else {
                            compile_data.push('}');
                        } continue;
                    }

                    if is_struct && is_struct_member {
                        if !struct_member_name.is_empty() {
                            is_struct = false;
                            is_struct_member = false;
                            is_struct_generic = false;
    
                            codegen.variable_definition(&struct_member_default, &struct_member_type,
                                                        &struct_member_name, true);

                            self.data_list.variable_list.push(GreteaVariableData {
                                __keyword_type: GreteaKeywords::Var,
                                __name        : struct_member_name.clone(),
                                __type        : struct_member_type.clone(),
                                __data        : struct_member_default.clone()
                            });

                            struct_generic.clear();

                            struct_member_name.clear();
                            struct_member_type.clear();
                            struct_member_default.clear();
                            is_default = false;
                        } codegen.character(&to("};"));
                    }
                    else if is_inline_asm {
                        codegen.assembly(&asm_block);
                        is_inline_asm = false; asm_block.clear();
                    }
                    else if is_safe {
                        is_safe = false;
                        is_unsafe = true;
                    }
                    else if is_unsafe {
                        is_unsafe = false;
                    }
                    else if is_enum {
                        codegen.enumeration(&enum_name, &enum_type, &enum_data);
                        is_enum = false;
                        is_enum_data = false;
                        is_enum_type = false;
                        enum_name.clear(); enum_type.clear(); enum_data.clear();
                    }
                    else if is_var_struct || is_var_data_vector {
                        codegen.variable_definition(&format!("{}}}", var_data),
                                                    &variable_type, &var_name, is_mutable);

                        self.data_list.variable_list.push(GreteaVariableData {
                            __keyword_type: GreteaKeywords::Var,
                            __name        : var_name.clone(),
                            __type        : variable_type.clone(),
                            __data        : var_data.clone()
                        });

                        is_var     = false;
                        is_var_type= false;
                        is_var_data= false;
                        is_mutable = false;
                        is_var_data_vector = false;

                        var_data     .clear();
                        variable_type.clear();
                        var_name     .clear();

                        is_var_struct = false;
                    }
                    else if !is_cpp_linker && !is_runtime && !is_inline_asm {
                        codegen.character(&self.init_ast.ast_curly_right_bracket);
                    }
                    else if is_cpp_linker {
                        codegen.character(&cpp_block);
                        is_cpp_linker = false; cpp_block.clear();
                    }
                    else if is_runtime {
                        let elite_read = elite::read::EliteFileData {
                            raw_data: runtime_block.clone(),
                            unparsed: vec![]
                        };

                        elite::lexer::elite_lexer::init_lexer(&elite_read);
                        is_runtime = false; runtime_block.clear();
                    }
                },

                GreteaKeywords::Preprocessor => {
                    is_preprocessor = true; continue;
                },
                GreteaKeywords::Set          => {
                    if !is_runtime || !is_cpp_linker {
                        if is_preprocessor {
                            is_set = true; continue;
                        }
                    }

                    if is_runtime {
                        runtime_block.push_str("set ");
                    }
                },

                GreteaKeywords::In => {
                    if is_for {
                        is_for_in = true;
                    }
                },

                GreteaKeywords::FlagLeft => {
                    is_library_setter = true;
                },
                GreteaKeywords::FlagRight => {
                    if is_library_setter {
                        is_library_setter = false;
                    }
                },

                GreteaKeywords::Vector => {
                    is_vector = true;
                },

                GreteaKeywords::DirectiveEnd => {
                    if is_directive {
                        codegen.directive_end();
                        is_directive = false; continue;
                    }
                },

                _ => {
                    if !is_line {
                        if is_compile {
                            if is_compile_data {
                                if token.trim_end() == "{" {
                                    count_compile_parentheses += 1;

                                    if count_compile_parentheses == 1 { continue; }
                                }

                                compile_data.push_str(&*format!("{} ", from_module(&token)));
                                continue;
                            }

                            if compile_config != GreteaCompileType::Undefined {
                                if compile_type.is_empty() {
                                    compile_type = token.clone();
                                } else {
                                    if token.trim_end() == "{" {
                                        count_compile_parentheses += 1;
                                        is_compile_data = true;
                                    }
                                }
                                continue;
                            }

                            compile_config =
                                match token.trim() {
                                    "default" => GreteaCompileType::Default,
                                    _ => {
                                        // undefined
                                        GreteaCompileType::Undefined
                                    }
                                };

                            continue;
                        }


                        if is_vector {
                            vector_type = token.clone();

                            is_vector = false;

                            if is_fn_argument || is_var {
                                token = make_vector(&token);
                            } else {
                                codegen.cpp_vector(&vector_type);

                                vector_type.clear();

                                continue;
                            }
                        }


                        if is_library_setter {
                            if is_default {
                                if token == "=" { continue; }

                                struct_member_default = token.clone();
                                continue;
                            }

                            match token.as_str() {
                                "library" | "stl" => {
                                    codegen.header_guards();
                                },
                                "no_optimize" => {
                                    codegen.optimize = false;
                                },
                                "optimize" => {
                                    codegen.optimize = true;
                                },
                                "default" => {
                                    is_default = true;
                                },
                                _ => {
                                    if token.starts_with('"') && token.ends_with('"') {
                                        codegen.character(&extract_argument(&token));
                                    } else {
                                        gen(Warning,
                                            "[[ ]] flag not found.",
                                            &*token,
                                            &self.raw_data,
                                            &(current_line as usize),
                                            &current_column);

                                        gen(Help,
                                            "you may have missed this: [[ \"..\" ]]",
                                            &*token,
                                            &self.raw_data,
                                            &(current_line as usize),
                                            &current_column);
                                    }
                                }
                            }
                            continue;
                        }

                        if is_unsafe || is_safe {
                            if token == "{" { continue; }
                        }

                        if is_enum {
                            if is_enum_data {
                                enum_data.push_str(token.clone().as_str());
                                if token == "," {
                                    enum_data.push('\n');
                                }
                                continue;
                            }

                            if token == "{" {
                                is_enum_data = true;
                                continue;
                            }

                            if enum_name.is_empty() {
                                enum_name = token.clone();
                                continue;
                            } else {
                                if is_enum_type {
                                    enum_type = token.clone();
                                    is_enum_type = false;
                                    continue;
                                }
                                if token == "=" {
                                    is_enum_type = true;
                                    continue;
                                }
                            }

                            continue;
                        }

                        if is_statement {
                            if token == "{" {
                                if is_preprocessor {
                                    codegen.statement_directive(&statement_data);

                                    is_directive = true;
                                } else { codegen.statement(&statement_data, is_while); }

                                is_statement = false;
                                is_preprocessor = false;
                                is_while = false;

                                statement_data.clear();
                                continue;
                            }

                            statement_data.push(token.clone());
                            continue;
                        }

                        if is_struct {
                            if is_struct_member {
                                if token == ">" {
                                    is_struct_generic = if is_struct_generic { false } else { false };
                                    continue;
                                }
                                if is_struct_generic {
                                    struct_generic = token.clone();
                                    continue;
                                }

                                if token == "<" {
                                    is_struct_generic = true;
                                    continue;
                                }


                                if token == "{" {
                                    codegen.structure(&struct_name, &struct_generic);
                                    struct_list.push(struct_name.clone());

                                    codegen.character(&to("{\npublic:"));
                                    continue;
                                }

                                if !struct_member_name.is_empty() {
                                    if token == ":" { continue; }

                                    if !struct_member_type.is_empty() {
                                        if token.ends_with('\n') || token.ends_with(',') || token.ends_with('\\') {
                                            codegen.variable_definition(&struct_member_default, &struct_member_type,
                                                                        &struct_member_name, struct_member_immutable);

                                            self.data_list.variable_list.push(GreteaVariableData {
                                                __keyword_type: GreteaKeywords::Var,
                                                __name: struct_member_name.clone(),
                                                __type: struct_member_type.clone(),
                                                __data: struct_member_default.clone()
                                            });

                                            struct_member_name.clear();
                                            struct_member_type.clear();
                                            struct_member_default.clear();
                                            is_default = false;
                                            struct_member_immutable = true;
                                            continue;
                                        }
                                    }

                                    struct_member_type = token.clone();
                                    continue;
                                }

                                struct_member_name = token.clone();
                                continue;
                            }

                            struct_name = token.clone();
                            is_struct_member = true;
                            continue;
                        }

                        if is_module {
                            // if token == "{" { }
                            module_name = token.clone();

                            codegen.module(&module_name);

                            is_module = false;
                            module_name.clear();
                            continue;
                        }

                        if is_set {
                            if token == "=" { continue; }

                            if !set_name.is_empty() {
                                set_data = token.clone();

                                codegen.preprocess_set(&set_data, &set_name);

                                is_preprocessor = false;
                                is_set = false;
                                set_name.clear();
                                set_data.clear();
                                continue;
                            }

                            set_name = token.clone();
                            continue;
                        }

                        if is_fn_call {
                            if token == "(" {
                                if is_pretty_arg {
                                    pretty_arg.push_str(token.clone().as_str());
                                } else {
                                    if count_parentheses >= 1 {
                                        function_args.push(token.clone());
                                    }
                                }

                                count_parentheses += 1;
                                continue;
                            }

                            if token == ")" {
                                if count_parentheses > 1 {
                                    if is_pretty_arg {
                                        pretty_arg.push_str(token.clone().as_str());

                                        if count_parentheses == 2 {
                                            is_pretty_arg = false;
                                            function_args
                                                .push(pretty_arg.clone());
                                            pretty_arg.clear();
                                        }
                                    } else {
                                        function_args.push(token.clone());
                                    }
                                }

                                count_parentheses -= 1;
                            }

                            if token == "(" || token == ")" || token == "," {
                                if count_parentheses == 0 {
                                    if token == ")" {
                                        codegen.function_call(&function_args,
                                                              &function_name, is_expandable);

                                        is_fn_call = false;
                                        is_expandable = false;

                                        count_parentheses = 0;

                                        function_name.clear();
                                        function_args.clear();
                                    }
                                } else if is_pretty_arg && token == "," {
                                    pretty_arg.push_str(token.as_str());
                                }

                                continue;
                            }

                            match token.as_str() {
                                "+_" => {
                                    is_expandable = true;
                                    continue;
                                },
                                _ => {
                                    if is_pretty_arg {
                                        pretty_arg.push_str(from_module(&token).as_str());
                                        continue;
                                    }

                                    let __token = token.split('#').last().unwrap().trim().to_string();

                                    for name in self.func_list.clone() {
                                        if name == __token.clone() {
                                            pretty_arg = from_module(&token);
                                            is_pretty_arg = true;
                                            break;
                                        }
                                    }

                                    if is_pretty_arg { continue; }

                                    function_args.push(token.clone());
                                }
                            }

                            continue;
                        }

                        if is_import {
                            codegen.import(token.clone());

                            is_import = false;
                            continue;
                        }

                        if is_fn {
                            if is_fn_name {
                                if token == "<" && !is_fn_argument {
                                    is_generic = true;
                                    continue;
                                }

                                if is_generic {
                                    if token == ">" {
                                        is_generic = false;
                                        continue;
                                    }

                                    if token == "'" {
                                        is_expandable = true;
                                        continue;
                                    }

                                    fn_generic = token.clone();
                                    continue;
                                }

                                if token == "(" {
                                    is_fn_argument = true;
                                    continue;
                                }

                                if is_fn_argument {
                                    if token == ":" {
                                        continue;
                                    }

                                    if token == "=" {
                                        is_fn_return_value = true;
                                        is_void = false;
                                        continue;
                                    }

                                    if token == "{" {
                                        if is_fn_return_value {
                                            gen(Error,
                                                "no such return type found after type operator declaration",
                                                &*token,
                                                &self.raw_data,
                                                &(current_line as usize),
                                                &current_column);
                                        }

                                        is_void = true;
                                    }

                                    if is_fn_return_value || is_void {
                                        fn_val = if is_fn_return_value { token.clone() } else { to("void") };

                                        codegen.function(self, &fn_args,
                                                         &fn_name.clone(),
                                                         &fn_val.clone(),
                                                         &fn_generic.clone(),
                                                         is_expandable,
                                                         is_void);

                                        fn_args.clear();
                                        fn_val.clear();

                                        is_fn_data = true;
                                        is_fn = false;
                                        is_fn_name = false;
                                        is_generic = false;
                                        is_expandable = false;
                                        is_fn_argument = false;
                                        is_fn_return_value = false;
                                        is_void = false;

                                        fn_generic.clear();
                                        continue;
                                    }

                                    if argument_name.is_empty() {
                                        if token != ")" {
                                            argument_name = token.clone();
                                        } else {
                                            argument_name.clear();
                                            argument_value.clear();
                                        }
                                        continue;
                                    }

                                    if argument_value.is_empty() {
                                        if token != ")" {
                                            argument_value = token.clone();
                                        }
                                        continue;
                                    } else {
                                        if token.trim() == "," || token.trim() == ")" {
                                            if is_expandable {
                                                if argument_value == fn_generic {
                                                    argument_value.push_str("...");
                                                }
                                            }

                                            fn_args.insert(argument_name.clone(), argument_value.clone());
                                            argument_name.clear();
                                            argument_value.clear();
                                        } else {
                                            argument_value.push_str(format!("{} ", token).as_str());
                                        }
                                        continue;
                                    }

                                    argument_name.clear();
                                    argument_value.clear();
                                    continue;
                                }

                                if token == ")" {
                                    is_void = true;
                                    continue;
                                }

                                is_fn = false;
                                is_fn_name = false;
                                continue;
                            }

                            fn_name = token.split('#').last().unwrap().to_string();
                            is_fn_name = true;

                            self.func_list.push(fn_name.clone());
                            continue;
                        }

                        if token == "{" {
                            if is_var_struct {
                                var_data.push_str("{\n");
                            } else if is_var {
                                is_var_data_vector = true;
                                var_data.push_str("{\n");
                            } else if !is_inline_asm
                                && !is_cpp_linker
                                && !is_runtime
                                && !is_var_struct {
                                codegen.character(&self.init_ast.ast_curly_left_bracket);
                            }

                            continue;
                        }

                        if is_var {
                            if !var_name.is_empty() {
                                if is_var_type {
                                    if token == "=" {
                                        is_var_type = false;
                                    } else {
                                        variable_type.push_str(format!("{} ", token.clone()).as_str());
                                        continue;
                                    }
                                }

                                if !is_var_type {
                                    if is_var_data {
                                        if is_var_struct {
                                            if token == "{" {
                                                var_data.push_str("{\n");
                                                continue;
                                            }

                                            if !var_struct_init_name.is_empty() {
                                                if token == ":" { continue; }
                                                if !var_struct_init_data.is_empty() {
                                                    if token == "," || token.ends_with('\n') {
                                                        var_data.push_str(format!(".{}={},", var_struct_init_name,
                                                                                  var_struct_init_data).as_str());
                                                        var_struct_init_name.clear();
                                                        var_struct_init_data.clear();
                                                        continue;
                                                    }
                                                }

                                                var_struct_init_data = token.clone();
                                                continue;
                                            }

                                            var_struct_init_name = token.clone();
                                            continue;
                                        } else if is_var_data_vector {
                                            if token.trim_end() == "\\" {
                                                var_data.push_str(",\n");
                                                continue;
                                            }

                                            var_data.push_str(&token.clone());
                                            continue;
                                        }

                                        var_data = token.clone();

                                        for structure in struct_list.clone() {
                                            if structure == var_data {
                                                is_var_struct = true;
                                                break;
                                            }
                                        }
                                        if is_var_struct {
                                            /*var_data.push_str("
                                        }{\n"); */ continue;
                                        }

                                        if variable_type.trim() == "_" {
                                            variable_type.clear();
                                        }

                                        codegen.variable_definition(&var_data, &variable_type, &var_name, is_mutable);

                                        self.data_list.variable_list.push(GreteaVariableData {
                                            __keyword_type: GreteaKeywords::Var,
                                            __name: var_name.clone(),
                                            __type: variable_type.clone(),
                                            __data: var_data.clone()
                                        });

                                        is_var = false;
                                        is_var_type = false;
                                        is_var_data = false;
                                        is_mutable = false;

                                        var_data.clear();
                                        variable_type.clear();
                                        var_name.clear();
                                        continue;
                                    }

                                    if token == "=" {
                                        is_var_data = true;
                                        continue;
                                    }
                                }

                                if token == ":" {
                                    is_var_type = true;
                                    continue;
                                }
                            }

                            var_name = token.clone();
                            continue;
                        }

                        if is_for {
                            if token == "_" {
                                codegen.for_loop();
                                is_for = false;
                                continue;
                            }

                            if is_for_variable {
                                if is_for_in {
                                    for_iter = token.clone();
                                    codegen.for_iter(&for_var, &for_iter);

                                    for_var.clear();
                                    for_iter.clear();

                                    is_for = false;
                                    is_for_variable = false;
                                    is_for_in = false;
                                    continue;
                                }
                            }

                            for_var = token.clone();
                            is_for_variable = true;
                            continue;
                        }

                        if is_cpp_linker {
                            if token.trim_end() == "\\" {
                                cpp_block.push('\n');
                                continue;
                            }

                            cpp_block.push_str(token.as_str());
                            continue;
                        }

                        if is_inline_asm {
                            let tok = token.trim_end();
                            if tok == "\\" {
                                asm_block.push('\n');
                                continue;
                            } else if tok == "%" {
                                asm_block.push('%');
                                continue;
                            }

                            asm_block.push_str(format!("{} ", token).as_str());
                            continue;
                        }

                        if is_alias {
                            if is_alias_name {
                                if token == "=" { continue; }

                                alias_data = from_module(&to(token.clone().trim_end()));

                                if is_preprocessor {
                                    codegen.preprocess_set(&alias_data, &alias_name);
                                    is_preprocessor = false;
                                } else { alias_list.insert(alias_name.clone(), alias_data.clone()); }

                                is_alias = false;
                                alias_name.clear();
                                is_alias_name = false;
                                alias_data.clear();
                                continue;
                            }

                            alias_name = token.clone();
                            is_alias_name = true;

                            continue;
                        }

                        if is_runtime {
                            if token.trim_end() == "\\" {
                                runtime_block.push('\n');
                                continue;
                            }

                            runtime_block.push_str(format!("{} ", token).as_str());
                            continue;
                        }

                        let function_token = token.clone().split('#').last().unwrap().to_string();

                        if is_unsafe {
                            if function_token.contains('.') {
                                let function_token = token.clone().split('.').last().unwrap().to_string();
                                if !function_token.is_empty() {
                                    is_fn_call = true;
                                    function_name = token.clone();
                                }
                                continue;
                            }
                        } else {
                            for name in self.func_list.clone() {
                                if name.split('#').last().unwrap().trim() == function_token.trim() {
                                    is_fn_call = true;
                                    function_name = from_module(&token);
                                    break;
                                }
                            }
                        }
                    }

                    if is_fn_data {
                        if token == "." || token == "return" {
                            is_return = true; continue;
                        }

                        if is_return {
                            if is_data(&token.as_str()) {
                                codegen.return_variable(&token.clone());
                                is_return = false; continue;
                            }

                            return_val.push_str(format!("{} ", token.clone()).as_str());

                            if token.ends_with('\n') || token.ends_with(';') {
                                is_return = false;

                                if is_unsafe {
                                    codegen.return_variable(&token.clone()); continue;
                                }
                                else {
                                    for variable in &self.data_list.variable_list {
                                        if token == variable.__name {
                                            codegen.return_variable(&variable.__data);
                                            break;
                                        }
                                    }
                                }

                                if token.trim_end() == "_" {
                                    codegen.return_variable(&to(""));
                                } else { codegen.return_variable(&from_module(&return_val));
                                } return_val.clear();
                            }
                        } else {
                            if is_line {
                                line.push_str(from_module(&token).as_str());

                                if !(token == "+"
                                    || token == "-"
                                    || token == "*"
                                    || token == "/"
                                    || token == "%"
                                    || token == "^"
                                    || token == "&"
                                    || token == ">"
                                    || token == "<") {
                                    line.push(' ');
                                }

                                if token.ends_with('\n') || token.ends_with(';') {
                                    codegen.character(&format!("{};", line)); line.clear(); is_line = false;
                                } continue;
                            }

                            for variable in &self.data_list.variable_list {
                                if token == variable.__name
                                    || token.split('.').last().unwrap() == variable.__name {
                                    is_line = true;
                                    line.push_str(format!("{}", token.clone()).as_str()); break;
                                }
                            }
                        }

                        continue;
                    }
                }
            }
        }

        codegen
    }
}