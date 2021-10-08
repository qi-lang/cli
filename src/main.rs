/*
 * Copyright Qi Lang. 2021 All Rights Reserved.
 * This file is licensed under the MIT License.
 * License text available at https://opensource.org/licenses/MIT
 */

use qi_core::parsers;
use qi_interactive;

fn main() {
    qi_interactive::init(parsers::module::parse);
}
