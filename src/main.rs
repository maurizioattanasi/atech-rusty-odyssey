fn main() {
    let s = String::from("Hello, world!");

    let first_word = first_word(&s);

    println!("The first word of {} is: {}", s, first_word);

    // let (s2, len) = take_ownership(s);

    // println!("{} ownership has been returned. Len {}", s2, len);

    // let i: i32 = 42;

    // makes_copy(i);

    // println!("{}", i);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' || byte == b'\t' || byte == b'\n' || byte == b',' {
            return &s[0..i];
        }
    }
    return &s[..];
}

// fn take_ownership(s: String) -> (String, usize) {
//     println!("{} has been moved", s);

//     let len = s.len();

//     (s, len)
// }

// fn makes_copy(i: i32) {
//     println!("{} has been copied", i);
// }
