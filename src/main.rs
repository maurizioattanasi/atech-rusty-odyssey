
fn main() {
    let s = String::from("Hello, world!");

    let (s2, len) = take_ownership(s);

    println!("{} ownership has been returned. Len {}", s2, len);

    let i: i32 = 42;

    makes_copy(i);

    println!("{}", i);
}

fn take_ownership(s: String) -> (String, usize) {
    println!("{} has been moved", s);

    let len = s.len();
    
    (s, len)
}

fn makes_copy(i: i32) {
    println!("{} has been copied", i);
}

