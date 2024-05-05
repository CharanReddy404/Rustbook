fn main() {
    let hello = "Здравствуйте";
    let _s = &hello[0..4];
    println!("{}", hello);
    for c in hello.chars() {
        println!("{c}");
    }

    for b in hello.bytes() {
        println!("{b}");
    }
}
fn _old4() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;

    println!("{s}");

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");
    println!("{s}");
}

fn _old3() {
    // let mut s = String::from("lo");
    // s.push('l');
    // println!("{}", s);

    let s1 = String::from("hello, ");
    let s2 = String::from("world ");
    let s22 = String::from(" booooo");
    let s3 = s1 + &s2 + &s22;
    // println!("{}", s1);
    println!("{}", s2);
    println!("{}", s22);
    println!("{}", s3);
}

fn _old2() {
    let mut s1 = String::from("foo");
    println!("{}", s1);
    let s2 = "bar";
    s1.push_str(s2);
    println!("{}", s1);
    println!("{}", s2);
}

fn _old() {
    // let mut s = String::new();
    let data = "initial contents";
    let _s = data.to_string();

    let _s = "initial contents".to_string();
}
