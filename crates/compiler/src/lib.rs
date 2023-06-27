pub mod error;
pub mod hir;
pub mod lexer;
pub mod prelude;
pub mod util;

#[cfg(test)]
mod test {
    use std::process::exit;
    use std::rc::Rc;

    use crate::hir::parser::Parser;

    #[test]
    fn debug_crash() {
        let mut parser = Parser::new(Rc::from(
            r#"
// A module from another file
module util;

import std:{io.println, math.sqrt, Array};

trait Length {
    fun length(Self self) -> float;
}

enum Number {
    integer(int);
    float(float);
    none;
}

struct Vec3 {
    float x;
    float y;
    float z;

    fun new(float x, float y, float z) -> Vec3 {
        // Maybe add `x: x` -> `x` from Rust
        // return Vec3{
        //     x: x,
        //     y: y,
        //     z: z,
        // };
    }

    fun add(Self self, Vec3 other) -> Vec3 {
        return Vec3.new(
            self.x + other.x,
            self.y + other.y,
            self.z + other.z,
        );
    }
}

impl Vec3 : Length {
    // todo: self type and param
    fun len(Self self) -> float {
        return sqrt(self.x * self.x + self.y * self.y + self.z * self.z);
    }
}

const a = 1;

fun do_math(int x, int y) -> int {
    const z = 7;
    return x + y * 7;
}

// todo: -> Array<int>
fun create_array(int start, int end) -> Array {
    var array = Array.new(end - start + 1);
    // for i in 0..(end - start) {
    //     array[i] = i + start;
    // }
    return array;
}

fun main() {
    println("Hello world!");
    const my_vec = Vec3.new(5.0, 2.0, 3.0);
    // const my_num = Number{ int: 10 };
}
        "#,
        ));
        if let Err(err) = parser.parse() {
            eprintln!("Error: {err:?} ({:?})", err.slice(&parser.lex));
            exit(1);
        }
        println!("{:#?}", parser.ast);
    }
}
