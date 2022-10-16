use clap::{Arg, Command};
use std::{
    fs::File,
    io::{Read, Write},
    path::Path,
};

fn main() {
    let matches = Command::new("Typester")
        .version("0.1.0")
        .author("Alex E")
        .about("Convert Rust types to Typescript types.  This crate is meant as a teaching tool and not for production.")
        .arg(
            Arg::new("input")
                .short('i')
                .long("input")
                .required(true)
                .help("The Rust file to process (including extension)"),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .required(true)
                .help("The name of the Typescript file to output (including extension)"),
        )
        .get_matches();

    let input_filename = matches.get_one::<String>("input").expect("input required");
    let output_filename = matches
        .get_one::<String>("output")
        .expect("output required");

    dbg!(input_filename);
    dbg!(output_filename);

    let input_path = Path::new(input_filename);

    let mut input_file =
        File::open(input_path).expect(&format!("Unable to open file {}", input_path.display()));

    let mut input_file_text = String::new();

    input_file
        .read_to_string(&mut input_file_text)
        .expect("Unable to read file");

    // This is our tokenized version of Rust file ready to process
    let input_syntax: syn::File = syn::parse_file(&input_file_text).expect("Unable to parse file");

    // This string will store the output of the Typescript file that we will
    // continuously append to as we process the Rust file
    let mut output_text = String::new();

    output_text.push_str(&create_initial_types());

    output_text.push_str(&parse_syn_file(input_syntax));

    let mut output_file = File::create(output_filename).unwrap();

    write!(output_file, "{}", output_text).expect("Failed to write to output file");
}

/// Transform the contents of a syn File of Rust types into
/// a string of valid Typescript types
fn parse_syn_file(file: syn::File) -> String {
    let mut output_text = String::new();

    for item in file.items.iter() {
        match item {
            // This `Item::Type` enum variant matches our type alias
            syn::Item::Type(item_type) => {
                let type_text = parse_item_type(item_type);
                output_text.push_str(&type_text);
            }
            syn::Item::Enum(item_enum) => {
                let enum_text = parse_item_enum(item_enum);
                output_text.push_str(&enum_text);
            }
            syn::Item::Struct(item_struct) => {
                let struct_text = parse_item_struct(item_struct);
                output_text.push_str(&struct_text);
            }

            _ => {
                dbg!("Encountered an unimplemented token");
            }
        }
    }

    output_text
}

/// Converts a Rust struct to a Typescript interface
///
/// ## Examples
///
/// **Input:**
/// struct Person {
///     name: String,
///     age: u32,
///     enjoys_coffee: bool,
/// }
///
/// **Output:**
/// export interface Person {
///     name: string;
///     age: number;
///     enjoys_coffee: boolean;
/// }
fn parse_item_struct(item_struct: &syn::ItemStruct) -> String {
    let mut output_text = String::new();

    let struct_name = item_struct.ident.to_string();
    output_text.push_str("export interface");
    output_text.push_str(" ");
    output_text.push_str(&struct_name);
    output_text.push_str(" ");
    output_text.push_str("{");
    match &item_struct.fields {
        syn::Fields::Named(named_fields) => {
            for named_field in named_fields.named.iter() {
                match &named_field.ident {
                    Some(ident) => {
                        let field_name = ident.to_string();
                        output_text.push_str(&field_name);
                        output_text.push_str(":");
                    }
                    None => {
                        dbg!("Encountered an unimplemented token");
                    }
                }
                let field_type = parse_type(&named_field.ty);
                output_text.push_str(&field_type);
                output_text.push_str(";");
            }
        }
        syn::Fields::Unnamed(fields) => {
            // Example: struct Something (i32, Anything);
            // Output: export interface Something { 0: i32, 1: Anything }
            for (index, field) in fields.unnamed.iter().enumerate() {
                output_text.push_str(&index.to_string());
                output_text.push_str(":");
                output_text.push_str(&parse_type(&field.ty));
                output_text.push_str(";");
            }
        }
        syn::Fields::Unit => (),
    }
    output_text.push_str("}");
    output_text.push_str(";");

    output_text
}

/// Converts a Rust enum to a Typescript type
///
/// ## Examples
///
/// **Input:**
/// enum Colour {
///     Red(i32, i32),
///     Green(i32),
///     Blue(i32),
/// }
///
/// **Output:**
/// export type Colour =
///   | { t: "Red"; c: number }
///   | { t: "Green"; c: number }
///   | { t: "Blue"; c: number };
fn parse_item_enum(item_enum: &syn::ItemEnum) -> String {
    let mut output_text = String::new();

    output_text.push_str("export type");
    output_text.push_str(" ");

    let enum_name = item_enum.ident.to_string();
    output_text.push_str(&enum_name);
    output_text.push_str(" ");
    output_text.push_str("=");
    output_text.push_str(" ");

    for variant in item_enum.variants.iter() {
        // Use the pipe character for union types
        // Typescript also allows it before the first type as valid syntax
        // https://www.typescriptlang.org/docs/handbook/2/everyday-types.html#union-types
        output_text.push_str(" | {");
        output_text.push_str(" ");

        // For simplicity this implementation we are using assumes that enums will be
        // using serde's "Adjacently Tagged" attribute
        // #[serde(tag = "t", content = "c")]
        // https://serde.rs/enum-representations.html#adjacently-tagged
        // As an improvement on this implementation you could parse the attribute
        // and handle the enum differently depending on which attribute the user chose
        output_text.push_str("t: \"");
        let variant_name = variant.ident.to_string();
        output_text.push_str(&variant_name);
        output_text.push_str("\" , c: ");

        match &variant.fields {
            syn::Fields::Named(named_fields) => {
                output_text.push_str("{");
                for field in named_fields.named.iter() {
                    if let Some(ident) = &field.ident {
                        output_text.push_str(&ident.to_string());
                        output_text.push_str(":");

                        let field_type = parse_type(&field.ty);
                        output_text.push_str(&field_type);
                        output_text.push_str(";");
                    }
                }
                output_text.push_str("}");
            }
            syn::Fields::Unnamed(unnamed_fields) => {
                // Currently only support a single unnamed field: e.g the i32 in Blue(i32)
                let unnamed_field = unnamed_fields.unnamed.first().unwrap();
                let field_type = parse_type(&unnamed_field.ty);
                output_text.push_str(&field_type);
            }
            syn::Fields::Unit => {
                output_text.push_str("undefined");
            }
        }

        output_text.push_str("}");
    }
    output_text.push_str(";");

    output_text
}

/// Converts a Rust item type to a Typescript type
///
/// ## Examples
///
/// **Input:** type NumberAlias = i32;
///
/// **Output:** export type NumberAlias = number;
fn parse_item_type(item_type: &syn::ItemType) -> String {
    let mut output_text = String::new();

    output_text.push_str("export type ");

    // `ident` is the name of the type alias, `NumberAlias` from the example
    output_text.push_str(&item_type.ident.to_string());
    output_text.push_str(" = ");

    let type_string = parse_type(&item_type.ty);
    output_text.push_str(&type_string);
    output_text.push_str(";");

    output_text
}

/// Converts a Rust type into a Typescript type
///
/// ## Examples
///
/// **Input:** (i32, i32) / Option<String>
///
/// **Output:** \[number, number\] / Option<string>;
fn parse_type(syn_type: &syn::Type) -> String {
    let mut output_text = String::new();

    match syn_type {
        // Primitive types like i32 will match Path
        // We currently do not do anything with full paths
        // so we take only the last() segment (the type name)
        syn::Type::Path(type_path) => {
            let segment = type_path.path.segments.last().unwrap();

            let field_type = segment.ident.to_string();

            let ts_field_type = parse_type_ident(&field_type).to_owned();
            output_text.push_str(&ts_field_type);

            match &segment.arguments {
                // A simple type like i32 matches here as it
                // does not include any arguments
                syn::PathArguments::None => {}
                // Example: HashMap<String, Colour>
                syn::PathArguments::AngleBracketed(angle_bracket_args) => {
                    output_text.push_str("<");
                    let args = angle_bracket_args.args.iter();
                    for arg in args {
                        match arg {
                            syn::GenericArgument::Type(inner_type) => {
                                output_text.push_str(&parse_type(inner_type));
                                output_text.push_str(",");
                            }
                            _ => {
                                dbg!("Encountered an unimplemented token");
                            }
                        }
                    }
                    output_text.push_str(">");
                }
                _ => {
                    dbg!("Encountered an unimplemented token");
                }
            }
        }
        // Tuple types like (i32, i32) will match here
        syn::Type::Tuple(type_tuple) => {
            output_text.push_str("[");
            for elem in type_tuple.elems.iter() {
                output_text.push_str(&parse_type(elem));
                output_text.push_str(",");
            }
            output_text.push_str("]");
        }
        _ => {
            dbg!("Encountered an unimplemented token");
        }
    };

    output_text
}

/// Convert a primitive Rust ident to an equivalent Typescript type name
/// Translate primitive types to Typescript equivalent otherwise
/// returns the ident untouched
///
/// ## Examples
///
/// **Input:** i32 / Option / bool;
///
/// **Output:** number / Option / boolean;
fn parse_type_ident(ident: &str) -> &str {
    match ident {
        "i8" | "i16" | "i32" | "i64" | "i128" | "u8" | "u16" | "u32" | "u64" | "f32" | "f64"
        | "isize" | "usize" => "number",
        "str" | "String" | "char" => "string",
        "bool" => "boolean",
        _ => ident,
    }
}

/// Initialize some Typescript equivalents of
/// core Rust types like Result, Option, etc
fn create_initial_types() -> String {
    let mut output_text = String::new();

    output_text.push_str("type HashSet<T extends number | string> = Record<T, undefined>;");
    output_text.push_str("type HashMap<T extends number | string, U> = Record<T, U>;");
    output_text.push_str("type Vec<T> = Array<T>;");
    output_text.push_str("type Option<T> = T | undefined;");
    output_text.push_str("type Result<T, U> = T | U;");

    output_text
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn handles_type_alias() {
        let mut input_file = File::open("./src/tests/type.rs").unwrap();

        let mut input_file_text = String::new();

        input_file.read_to_string(&mut input_file_text).unwrap();

        let input_syntax: syn::File =
            syn::parse_file(&input_file_text).expect("Unable to parse file");

        let typescript_types = parse_syn_file(input_syntax);

        assert_eq!(r#"export type NumberAlias = number;"#, &typescript_types);
    }

    #[test]
    fn handles_struct() {
        let mut input_file = File::open("./src/tests/struct.rs").unwrap();

        let mut input_file_text = String::new();

        input_file.read_to_string(&mut input_file_text).unwrap();

        let input_syntax: syn::File =
            syn::parse_file(&input_file_text).expect("Unable to parse file");

        let typescript_types = parse_syn_file(input_syntax);

        assert_eq!(
            r#"export interface Person {name:string;age:number;enjoys_coffee:boolean;};"#,
            &typescript_types
        );
    }

    #[test]
    fn handles_enum() {
        let mut input_file = File::open("./src/tests/enum.rs").unwrap();

        let mut input_file_text = String::new();

        input_file.read_to_string(&mut input_file_text).unwrap();

        let input_syntax: syn::File =
            syn::parse_file(&input_file_text).expect("Unable to parse file");

        let typescript_types = parse_syn_file(input_syntax);

        assert_eq!(
            r#"export type Colour =  | { t: "Red" , c: number} | { t: "Green" , c: number} | { t: "Blue" , c: number};"#,
            &typescript_types
        );
    }
}
