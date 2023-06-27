# Design

## TODO

- [ ] Specify all root structures
  - [ ] Function
  - [ ] Struct
  - [ ] Enum
  - [ ] Trait
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
- [ ] Specify type system
  - [ ] Generics
  - [ ] Type inference
  - [ ] References
  - [ ] Mutability
- [ ] Specify HIR
- [ ] Specify MIR

## Ideas

```rust
// A module from another file
module util;

import std:{io.println, math.sqrt, Array};

trait Length {
    fun length(*self) -> float;
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
        return Vec3{
            x: x,
            y: y,
            z: z,
        };
    }

    fun add(*self, Vec3 other) -> Vec3 {
        return Vec3.new(
            self.x + other.x,
            self.y + other.y,
            self.z + other.z,
        );
    }
}

impl Vec3 : Length {
    fun len(*self) -> float {
        return sqrt(self.x * self.x + self.y * self.y + self.z * self.z);
    }
}

const a = 1;

fun do_math(int x, int y) -> int {
    const z = 7;
    return x + y * 7;
}

fun create_array(int start, int end) -> Array<int> {
    var array = Array.new(end - start + 1);
    for i in 0..(end - start) {
        array[i] = i + start;
    }
    return array;
}

fun main() {
    println("Hello world!");
    const my_vec = Vec3.new(5.0, 2.0, 3.0);
    const my_num = Number{ int: 10 };
}
```

## Reference

### Other

**Syntax**

```
Type:
    IDENTIFIER GenericArguments?

GenericArguments: (todo)
    `<` ... `>`

GenericParameters: (todo)
    `<` ... `>`
```

### Root structures

#### Functions

**Syntax**

```
Function:
    FunctionQualifiers `fun` IDENTIFIER GenericParameters?
      `(` FunctionParameters? `)`
      FunctionReturnType?
      BlockExpression

FunctionQualifiers:
    `pub`?

FunctionParameters:
    SelfParam `,`?
    | (SelfParam `,`)? FunctionParam (`,` FunctionParam)* `,`?

SelfParam:
    (`*` | `*mut`)? `self`

FunctionParam:
    Type IDENTIFIER

FunctionReturnType:
    `->` Type
```
