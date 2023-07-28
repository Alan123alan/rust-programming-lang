fn slice_first_word(s: &str)-> &str{
    for (index, char) in s.chars().enumerate(){
	if char == ' '{
            return &s[..index];
        }
    }
    return &s[..];
}

fn main() {
    let my_str = String::from("Hello, world!");
    let first_word = slice_first_word(&my_str);
    println!("{}", first_word);
}
