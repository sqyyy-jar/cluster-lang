# Design

## TODO

- [ ] Specify all root structures
  - [ ] Function
  - [ ] Struct
  - [ ] Constant declaration
- [ ] Specify all control-flow statements
  - [ ] If
  - [ ] While
  - [ ] For
  - [ ] Switch/Match (???)
- [ ] Specify all primitive data types
  - [ ] `uint`
  - [ ] `int`
  - [ ] `float`
  - [ ] `bool`
- [ ] Specify all other statement types
  - [ ] Assignment
  - [ ] Variable declaration
  - [ ] Constant declaration
  - [ ] Call
- [ ] Specify module system
  - [ ] Imports
  - [ ] Module declaration

## Ideas

```rs
// A module from another file
module util;

import std:{io.println, math.sqrt};

trait Length {
    fun length(self) -> float;
}

// Maybe change up this syntax
enum Number {
    integer: int;
    float: float;
}

struct Vec3 {
    float x;
    float y;
    float z;

    fun new(float x, float y, float z) -> Vec3 {
        // Maybe add `x: x` -> `x` from Rust
        return Vec3{
            x: x,
            y: y,
            z: z,
        };
    }

    fun add(self, other: Vec3) -> Vec3 {
        return Vec3.new(
            self.x + other.x,
            self.y + other.y,
            self.z + other.z,
        );
    }
}

// Maybe use `impl` or `implement` instead
// Maybe swap `Vec3` and `Length` and replace `:` with `for` in between
derive Vec3 : Length {
    fun len(self) -> float {
        return sqrt(self.x * self.x + self.y * self.y + self.z * self.z);
    }
}

const a = 1;

fun do_math(int x, int y) -> int {
    let z = 7;
    return x + y * 7;
}

fun main() {
    println("Hello world!");
    let my_vec = Vec3.new(5.0, 2.0, 3.0);
    // Maybe change up enum initialization syntax
    let my_num = Number{ int: 10 };
}
```
