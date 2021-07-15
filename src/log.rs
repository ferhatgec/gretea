// MIT License
//
// Copyright (c) 2021 Ferhat Geçdoğan All Rights Reserved.
// Distributed under the terms of the MIT License.
//
//

#[allow(unused_variables)]
#[derive(PartialEq)]
pub enum LogTypes {
    Info,
    Warning,
    Error,
    Help
}

pub mod log {
    use crate::{
        log::{LogTypes},
        read::{GreteaFileData}
    };

    pub fn gen(log_type: LogTypes,
               error: &str,
               matched_token: &str,
               data: &GreteaFileData,
               line: &usize,
               column: &u32) {
        println!("\x1b[1;{}:\x1b[0;97m (l: {}, c: {}, t: {}): {}\n   {} | {}", match log_type {
            LogTypes::Info => {
                "93minfo"
            },
            LogTypes::Warning => {
                "33m warning"
            },
            LogTypes::Error => {
                "31m error"
            },
            LogTypes::Help => {
                "36m help"
            }
        }, line, column, matched_token, error, line, data.lines[*line]);

        if log_type == LogTypes::Error {
            std::process::exit(1);
        }
    }
}