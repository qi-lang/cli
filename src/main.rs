/*
 * Copyright Qi Lang. 2021 All Rights Reserved.
 * This file is licensed under the MIT License.
 * License text available at https://opensource.org/licenses/MIT
 */

use qi_core;

fn main() {
    println!(
        "{:?}",
        qi_core::parse_from_str("[]", qi_core::parsers::list::parse)
    );
    println!(
        "{:?}",
        qi_core::parse_from_str("[true, false]", qi_core::parsers::list::parse)
    );
    println!(
        "{:?}",
        qi_core::parse_from_str("[true, false, true]", qi_core::parsers::list::parse)
    );
}
