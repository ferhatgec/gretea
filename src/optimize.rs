// MIT License
//
// Copyright (c) 2021 Ferhat Geçdoğan All Rights Reserved.
// Distributed under the terms of the MIT License.
//
//

pub enum OptimizeBlocks {
    StatementBool
}

pub mod optimizer {
    use crate::optimize::{
        OptimizeBlocks,
        optimize_helpers::{*}
    };

    pub fn optimize(data: &Vec<String>, optimize_type: OptimizeBlocks) -> Vec<String> {
        let mut data = data.clone();
        match optimize_type {
            OptimizeBlocks::StatementBool => {
                let mut __is_op = false;
                let mut is_op = false;

                for i in 0..data.len() {
                    if __is_op {
                        if is_op {
                            if is_boolean(&data[i]) {
                                if data[i] == "true" {
                                    data[i - 2].clear(); // ! | =
                                    data[i - 1].clear(); // =
                                    data[i    ].clear(); // true

                                    is_op = false; continue;
                                } else {
                                    data[i - 3] = format!("!{}", data[i - 3]);
                                    data[i - 2].clear(); // ! | =
                                    data[i - 1].clear(); // =
                                    data[i    ].clear(); // false

                                    is_op = false; continue;
                                }
                            } continue;
                        }

                        if is_eq_or_uneq(&data[i]) {
                            is_op = true; continue;
                        }
                    }

                    if is_eq_or_uneq( &data[i]) {
                        __is_op = true; continue;
                    }
                }
            },
            _ => {}
        }

        data.clone()
    }
}

pub mod optimize_helpers {
    pub fn is_eq_or_uneq(data: &String) -> bool {
        return if data == "!" || data == "=" {
            true
        } else { false };
    }

    pub fn is_boolean(data: &String) -> bool {
        return if data == "true" || data == "false" {
            true
        } else { false };
    }
}