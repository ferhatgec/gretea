// MIT License
//
// Copyright (c) 2021 Ferhat Geçdoğan All Rights Reserved.
// Distributed under the terms of the MIT License.
//
//

pub mod gretea_lexer {
    use crate::{
        ast::{
            GreteaSyntax,
            GreteaVariableList
        },
        parser::{GreteaParser},
        read::{GreteaFileData},
        tokenizer::gretea_tokenizer::{tokenize}
    };

    pub fn init_lexer(init: &GreteaFileData) -> String {
        let tokens = tokenize(init);
        let ast   = GreteaSyntax::default();
        let mut parser = GreteaParser {
            init_ast : ast,
            data_list: GreteaVariableList::default()
        };


        parser.parse(&tokens)
    }
}