#![feature(result_flattening)]

pub mod error;
pub mod hir;
pub mod lexer;
pub mod prelude;
pub mod util;

#[cfg(test)]
mod test {
    use std::rc::Rc;

    use crate::hir::parser::Parser;

    #[test]
    fn debug_crash() {
        let mut parser = Parser::new(Rc::from(
            r#"
module test;

import a:{b, c.d};

fun main() {
    // a.a.a = 2;
    // a();
    // a.b();
    // a().b();
    // (1 + 2).a();
    // "abc".len();
    // a((1 + 2) * 3);
}
        "#,
        ));
        parser.parse().expect("Parse");
        println!("{:#?}", parser.ast);
    }
}
