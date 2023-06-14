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
- [ ] Specify all other statement types
  - [ ] Assignment
  - [ ] Variable declaration
  - [ ] Constant declaration
  - [ ] Call
- [ ] Specify module system
  - [ ] Imports
  - [ ] Module declaration

## Ideas

```
const a = 1;

struct Vec3 {
    float x,
    float y,
    float z,
}

fun do_math(int x, int y) -> int {
    let z = 7;
    return x + y * 7;
}

fun main() {
    println("Hello world!");
}
```
