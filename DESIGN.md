# TODO

- [x] Lexer
  - [x] multiline comments
- [ ] HIR
  - [ ] ~~self parameters~~
  - [x] Self type
  - [ ] generics
  - [ ] struct initializer syntax
  - [ ] enum initializer syntax
- [ ] MIR

# Features

- [Modules](#modules)
- [Imports](#imports)
- [Constants](#constants)
- [Traits](#traits)
- [Structs](#structs)
- [Enums](#enums)
- [Trait implementations](#trait-implementations)
- [Functions](#functions)
- [Further information](#further-information)
  - [pub keyword](#pub-keyword)

## Modules

Adds a module to the module tree.

- may be marked as `pub` to be visible to other modules

### Syntax

```
module util;
```

## Imports

Makes a module, constant, type or function from elsewhere usable in the current module.

### Syntax

```java
import util;
import std.math:{sqrt, vec.Vec3};
```

## Constants

Defines a module-bound constant.

- may be marked as `pub` to be visible to other modules

### Syntax

```rust
const magic_number = 42;
const magic_number: uint = 42;
```

## Traits

Defines a trait type that contains abstract functions that implementing types require to
implement.

- may be marked as `pub` to be visible to other modules
- functions are public by default and must not be marked as `pub`

### Syntax

```rust
trait Sized {
  fun len(*self) -> uint;
}
```

## Structs

Defines a struct type that contains fields and member functions.

- may be marked as `pub` to be visible to other modules
- fields may be marked as `pub` to be visible to other modules
- member functions may be marked as `pub` to be visible to other modules

### Syntax

```rust
struct Vec3 {
  float x;
  float y;
  float z;
  // ...member functions
}
```

## Enums

Defines an enum type that contains variants and member functions.

- may be marked as `pub` to be visible to other modules
- variants and their fields are public by default and must not be marked as `pub`
- member functions may be marked as `pub` to be visible to other modules

### Syntax

```rust
enum Variant {
  Empty;
  Tuple(float, int);
  Struct {
    float a;
    int b;
  }
  // ...member functions
}
```

## Trait implementations

Implements a trait for a type.

- functions must not be marked as `pub`

### Syntax

```rust
impl Vec3 : Sized {
  fun len(*self) -> uint {
    // ...implementation
  }
}
```

- `Vec3`: target type
- `Sized`: trait type

## Functions

- may be marked as `pub` to be visible to other modules

### Syntax

```kotlin
fun main() {
  println("Hello world!");
}
```

## Further information

### `pub` keyword

The `pub` keyword is placed in front of the keyword it should be applied to
(`pub fun ...`).