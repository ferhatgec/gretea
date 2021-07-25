// MIT License
//
// Copyright (c) 2021 Ferhat Geçdoğan All Rights Reserved.
// Distributed under the terms of the MIT License.
//
//

pub mod gretea_lexer {
    use {
        crate::{
            ast::{
                GreteaSyntax,
                GreteaVariableList
            },
            parser::{GreteaParser},
            read::{GreteaFileData},
            tokenizer::gretea_tokenizer::{tokenize}
        },
        std::collections::{BTreeMap}
    };

    pub fn init_lexer(init: GreteaFileData) -> (String, BTreeMap<String, bool>, Vec<String>) {
        let tokens = tokenize(&init);
        let ast   = GreteaSyntax::default();
        let mut parser = GreteaParser {
            init_ast : ast,
            data_list: GreteaVariableList::default(),
            func_data: vec![],
            raw_data : init.clone(),
            func_list: init.func_list.clone(),
            compile_list: vec![]
        };
        let data = parser.parse(&tokens);

        (data.generated, data.sources, parser.func_list)
    }
}