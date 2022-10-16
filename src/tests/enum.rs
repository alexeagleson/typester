#[serde(tag = "t", content = "c")]
enum Colour {
    Red(i32),
    Green(i32),
    Blue(i32),
}