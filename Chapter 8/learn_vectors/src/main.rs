enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}

fn _old4() {
    let v = vec![100, 32, 57];
    for n_ref in &v {
        let n_plus_one: i32 = *n_ref + 1;
        println!("{}--{}", n_ref, n_plus_one);
    }

    let mut v = vec![100, 32, 57];
    for n_ref in &mut v {
        *n_ref += 50;
    }
    println!("v : {:?}", v);
}

fn _old3() {
    let mut v = vec![1, 2, 3, 4, 5];
    let _first = &v[0];

    v.push(6);

    // println!("{}", _first);
}

fn _old2() {
    let v = vec![1, 2, 3, 4, 5];

    let does_not_exist = &v[100];
    println!("&v[100] {}", does_not_exist);
    let does_not_exist = v.get(100);
    println!("v.get(100) {:?}", does_not_exist);
}

fn _old1() {
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);

    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element : {third:?}"),
    }
}

fn _old() {
    // let v = vec![1, 2, 3];
    let mut v: Vec<i32> = Vec::new();

    v.push(4);
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
}
