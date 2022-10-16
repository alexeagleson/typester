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


