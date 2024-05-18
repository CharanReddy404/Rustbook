use add_one;
fn main() {
    let num = 10;
    println!(
        "Hello, world! {} pluse on is {}!",
        num,
        add_one::add_one(num)
    );
}
