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
