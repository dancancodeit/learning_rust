fn main() {
    let s = String::from("Hello World!");

    let first = first_word(&s[..]);

    // s.clear(); // cant mutate because s is being borrowed in previous line

    println!("{first}");
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
