fn main() {
    print!("{}", process(String::from("hello MATMEX")));
}

fn process(_text: String) -> String {
    let first_word = _text.split_whitespace().next().unwrap_or("").to_string();
    let mut text = _text;
    text.push_str(" rust");
    let mut_l = &mut text;
    let immut_l = &*mut_l;
    format!("{}\n{}\n{}\n", first_word, mut_l, immut_l)
}

#[cfg(test)]
#[path = "../tests/unit/process.rs"]
mod process_tests;
