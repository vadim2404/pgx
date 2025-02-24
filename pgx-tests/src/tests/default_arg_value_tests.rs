/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/

use pgx::prelude::*;

#[pg_extern]
fn negative_default_argument(i: default!(i32, -1)) -> i32 {
    i
}

#[pg_extern]
fn default_argument(a: default!(i32, 99)) -> i32 {
    a
}

#[pg_extern]
fn option_default_argument(a: default!(Option<&str>, "NULL")) -> &str {
    match a {
        Some(a) => a,
        None => "got default of null",
    }
}

#[cfg(any(test, feature = "pg_test"))]
#[pgx::pg_schema]
mod tests {
    #[allow(unused_imports)]
    use crate as pgx_tests;

    use pgx::prelude::*;

    #[test]
    fn make_idea_happy() {}

    #[pg_test]
    fn test_negative_default_argument() {
        let result = Spi::get_one::<i32>("SELECT negative_default_argument();");
        assert_eq!(result, Ok(Some(-1)));
    }

    #[pg_test]
    fn test_default_argument() {
        let result = Spi::get_one::<i32>("SELECT default_argument();");
        assert_eq!(result, Ok(Some(99)));
    }

    #[pg_test]
    fn test_default_argument_specified() {
        let result = Spi::get_one::<i32>("SELECT default_argument(2);");
        assert_eq!(result, Ok(Some(2)));
    }

    #[pg_test]
    fn test_option_default_argument() {
        let result = Spi::get_one::<&str>("SELECT option_default_argument();");
        assert_eq!(result, Ok(Some("got default of null")));
    }

    #[pg_test]
    fn test_option_default_argument_specified() {
        let result = Spi::get_one::<&str>("SELECT option_default_argument('test');");
        assert_eq!(result, Ok(Some("test")));
    }
}
