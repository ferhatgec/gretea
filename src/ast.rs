// MIT License
//
// Copyright (c) 2021 Ferhat Geçdoğan All Rights Reserved.
// Distributed under the terms of the MIT License.
//
//

use std::collections::{HashMap};

#[derive(Copy, Clone, PartialEq)]
pub enum GreteaKeywords {
    Import,
    Fn,

    Var,
    Mut,

    Cpp,
    Alias,

    If,
    Else,

    Module,

    For,

    LeftParenthese,
    RightParenthese,

    LeftSqBracket,
    RightSqBracket,

    LeftCurlyBracket,
    RightCurlyBracket,

    Preprocessor,
    Set,

    Unpack,

    Undefined
}

pub struct GreteaSyntax {
    pub ast_import              : String,
    pub ast_fn                  : String,

    pub ast_var                 : String,
    pub ast_mut                 : String,

    pub ast_cpp                 : String,
    pub ast_alias               : String,

    pub ast_if                  : String,
    pub ast_else                : String,

    pub ast_module              : String,

    pub ast_for                 : String,

    pub ast_left_parenthese     : String,
    pub ast_right_parenthese    : String,

    pub ast_square_left_bracket : String,
    pub ast_square_right_bracket: String,

    pub ast_curly_left_bracket  : String,
    pub ast_curly_right_bracket : String,

    pub ast_preprocessor        : String,
    pub ast_set                 : String,

    pub syntax_list             : HashMap<String, GreteaKeywords>
}

pub struct GreteaVariableData {
    pub __keyword_type: GreteaKeywords,
    pub __name        : String,
    pub __data        : String
}

pub struct GreteaVariableList {
    pub variable_list : Vec<GreteaVariableData>
}

impl Default for GreteaSyntax {
    fn default() -> Self {
        let mut init = GreteaSyntax {
            ast_import              : ast_helpers::to("import"),
            ast_fn                  : ast_helpers::to("fn"    ),

            ast_var                 : ast_helpers::to("var"   ),
            ast_mut                 : ast_helpers::to("mut"   ),

            ast_cpp                 : ast_helpers::to("cpp"   ),
            ast_alias               : ast_helpers::to("alias" ),

            ast_if                  : ast_helpers::to("if"    ),
            ast_else                : ast_helpers::to("else"  ),

            ast_module              : ast_helpers::to("module"),

            ast_for                 : ast_helpers::to("for"   ),

            ast_left_parenthese     : ast_helpers::to("("),
            ast_right_parenthese    : ast_helpers::to(")"),

            ast_square_left_bracket : ast_helpers::to("["),
            ast_square_right_bracket: ast_helpers::to("]"),

            ast_curly_left_bracket  : ast_helpers::to("{"),
            ast_curly_right_bracket : ast_helpers::to("}"),

            ast_preprocessor        : ast_helpers::to("`"  ),
            ast_set                 : ast_helpers::to("set"),

            syntax_list             : Default::default()
        };

        init.add(init.ast_import.clone(), GreteaKeywords::Import         );
        init.add(init.ast_fn    .clone(), GreteaKeywords::Fn             );

        init.add(init.ast_var   .clone(), GreteaKeywords::Var            );
        init.add(init.ast_mut   .clone(), GreteaKeywords::Mut            );

        init.add(init.ast_cpp   .clone(), GreteaKeywords::Cpp            );
        init.add(init.ast_alias .clone(), GreteaKeywords::Alias          );

        init.add(init.ast_if    .clone(), GreteaKeywords::If             );
        init.add(init.ast_else  .clone(), GreteaKeywords::Else           );

        init.add(init.ast_module.clone(), GreteaKeywords::Module         );

        init.add(init.ast_for   .clone(), GreteaKeywords::For            );

        init.add(init.ast_left_parenthese .clone(), GreteaKeywords::LeftParenthese );
        init.add(init.ast_right_parenthese.clone(), GreteaKeywords::RightParenthese);

        init.add(init.ast_square_left_bracket .clone(), GreteaKeywords::LeftSqBracket );
        init.add(init.ast_square_right_bracket.clone(), GreteaKeywords::RightSqBracket);

        init.add(init.ast_curly_left_bracket  .clone(), GreteaKeywords::LeftCurlyBracket );
        init.add(init.ast_curly_right_bracket .clone(), GreteaKeywords::RightCurlyBracket);

        init.add(init.ast_preprocessor        .clone(), GreteaKeywords::Preprocessor     );
        init.add(init.ast_set                 .clone(), GreteaKeywords::Set              );

        init
    }
}

impl Default for GreteaVariableList {
    fn default() -> Self {
        GreteaVariableList {
            variable_list: vec![]
        }
    }
}

impl GreteaSyntax {
    pub fn match_keyword(&self, keyword: &String) -> &GreteaKeywords {
        let keyword_type = self.syntax_list.get(keyword);

        if keyword_type.is_none() {
            return &GreteaKeywords::Undefined;
        } keyword_type.unwrap()
    }

    pub fn extract(self, data: &String) -> String {
        return if data.starts_with('"') && data.ends_with('"') {
            let mut temporary = data.chars();

            temporary.next(); temporary.next_back();

            ast_helpers::to(temporary.as_str())
        } else { ast_helpers::to(" ") };
    }

    pub fn add(&mut self, __data: String, __type: GreteaKeywords) {
        self.syntax_list.insert(__data, __type);
    }
}

pub mod ast_helpers {
    pub fn to(data: &str) -> String {
        data.to_string()
    }
    pub fn from_module(data: &String) -> String {
        let mut temporary = String::new();

        for character in data.chars() {
            if character == '#' {
                temporary.push_str("::"); continue;
            }

            temporary.push(character);
        }

        temporary
    }
}