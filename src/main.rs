fn first_word_len(s: &str) -> usize {
    for (i, c) in s.chars().enumerate() {
        if c == ' ' {
            return i;
        }
    }
    s.len()
}

fn main() {
    let s = "hello world";
    println!("{}", first_word_len(s));
}