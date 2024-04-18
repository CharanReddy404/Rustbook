struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// Unit-like structs
struct AlwaysEqual;

fn main() {
    let _black = Color(0, 0, 0);
    let _origin = Point(0, 0, 0);

    let _subject = AlwaysEqual;
}
