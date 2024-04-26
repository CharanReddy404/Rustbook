// By default, a variable has read/own permissions (RO) on its data. If a variable is annotated with let mut, then it also has the write permission (W). The key idea is that references can temporarily remove these permissions.

// fn main() {
//     let mut v: Vec<i32> = vec![1, 2, 3];
//     let mut num: &i32 = &v[2];
//     println!("{}", *num);
//     v.push(4);
//     num = &v[2];
//     println!("{}", *num);
//     // i cannot access num because the when i push new element to the vec the referance address will be changed

//     println!("{}", v.len());
// }

// fn main() {
//     let x = 2;
//     let mut x_ref = &x;
//     println!("{}--{}--{}", x, x_ref, *x_ref);

//     let y = 3;
//     x_ref = &y;

//     print!("{}", *x_ref);
// }

// mutable references
// fn main() {
//     let mut v: Vec<i32> = vec![1, 2, 3];
//     let num: &mut i32 = &mut v[2];

//     *num += 1;

//     println!("Third  element is {}", *num);
//     println!("Third  element is {}", num);

//     println!("Vector is now {:?}", v);
// }

// fn main() {
//     let mut v: Vec<i32> = vec![1, 2, 3];
//     let num: &mut i32 = &mut v[2];
//     let num2: &i32 = &*num;
//     println!("{} {}", *num, *num2);
// }

// fn main() {
//     let s = String::from("Hello world");
//     let s_ref = &s;
//     // i can not drop it because s_ref holdes the reference of s
//     // drop(s); // this gives an error
//     println!("{}", s_ref);
// }

fn incr(n: &mut i32) {
    *n += 1;
}
fn main() {
    let mut n = 1;
    incr(&mut n);
    println!("{n}");
}
