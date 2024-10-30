
fn main() {
    let s = String::from("Hello, world!");

    take_ownership(s);

    let i: i32 = 42;

    makes_copy(i);

    println!("{}", i);
}

fn take_ownership(s: String) {
    println!("{} has been moved", s);
}

fn makes_copy(i: i32) {
    println!("{} has been copied", i);
}

