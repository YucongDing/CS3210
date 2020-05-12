pub fn capitalize_first(input: &str) -> String {
    let mut c = input.chars();
    match c.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + c.as_str(),
    }
}


fn main() {
    let words = vec!["hello", " ", "world"];
    let capitalized_words = words.iter().map(|s| capitalize_first(s)).collect::<String>();
    println!("{:?}", capitalized_words);

    let s = "Peter,27";
    println!("{:?}", s.split())
}
