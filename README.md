## About

This crate is designed as a teaching tool for writing a Rust to Typescript type conversion utility.  

It supports very basic types like primitives, structs and enums.  It is not meant for production usage.  

It will output a warning message for unsupported types and simply ignore them.

## How to Use

```
cargo install typester
typester --input=path/to/rustfile.rs --output=path/to/tsfile.ts
```

For more information use:

```
typester --help
```

## Sample Input

```rust
type NumberAlias = i32;

#[serde(tag = "t", content = "c")]
enum Colour {
    Red(i32),
    Green(i32),
    Blue(i32),
}

struct Person {
    name: String,
    age: u32,
    enjoys_coffee: bool,
}

struct ComplexType {
    colour_map: HashMap<String, Colour>,
    list_of_names: Vec<String>,
    optional_person: Option<Person>,
}
```

## Sample Output

```ts
type HashSet<T extends number | string> = Record<T, undefined>;
type HashMap<T extends number | string, U> = Record<T, U>;
type Vec<T> = Array<T>;
type Option<T> = T | undefined;
type Result<T, U> = T | U;

export type NumberAlias = number;

export type Colour =
  | { t: "Red"; c: number }
  | { t: "Green"; c: number }
  | { t: "Blue"; c: number };

  export interface Person {
  name: string;
  age: number;
  enjoys_coffee: boolean;
}

export interface ComplexType {
  colour_map: HashMap<string, Colour>;
  list_of_names: Vec<string>;
  optional_person: Option<Person>;
}
```
