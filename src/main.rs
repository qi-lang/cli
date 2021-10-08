/*
 * Copyright Qi Lang. 2021 All Rights Reserved.
 * This file is licensed under the MIT License.
 * License text available at https://opensource.org/licenses/MIT
 */

use qi_core;

fn main() {
    let earlier = std::time::Instant::now();
    let result =
        qi_core::parse_from_file("./playground/other.qi", qi_core::parsers::definition::parse);
    let later = std::time::Instant::elapsed(&earlier);

    println!("Time to complete: {:#?}", later);
    println!("Result: {:#?}", result);
}
