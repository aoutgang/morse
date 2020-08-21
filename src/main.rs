use std::iter::FromIterator;

fn main() {
    println!("{}", to_morse("abcdefghijklmnopqrstuvwxyz "));
}

fn to_morse(input: &str) -> String {
    let data: Vec<String> = input.chars()
        .map(|x| match x {
            ' ' => String::from("....... "),
            'a' => String::from(".- "),
            'b' => String::from("-... "),
            'c' => String::from("-.-. "),
            'd' => String::from("-.. "),
            'e' => String::from(". "),
            'f' => String::from("..-. "),
            'g' => String::from("--. "),
            'h' => String::from(".... "),
            'i' => String::from(".. "),
            'j' => String::from(".--- "),
            'k' => String::from("-.- "),
            'l' => String::from(".-.. "),
            'm' => String::from("-- "),
            'n' => String::from("-. "),
            'o' => String::from(".- "),
            'p' => String::from(".--. "),
            'q' => String::from("--.- "),
            'r' => String::from(".-. "),
            's' => String::from("... "),
            't' => String::from("- "),
            'u' => String::from("..- "),
            'v' => String::from("...- "),
            'w' => String::from(".-- "),
            'x' => String::from("-..- "),
            'y' => String::from("-.-- "),
            'z' => String::from("--.. "),
            _ => panic!("Invalid character"),
        }).collect();
   
    String::from_iter(data)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn to_morse_test() {
        assert_eq!(to_morse("abcdefghijklmnopqrstuvwxyz "), 
        ".- -... -.-. -.. . ..-. --. .... .. .--- -.- .-.. -- -. .- .--. --.- .-. ... - ..- ...- .-- -..- -.-- --.. ....... ");
    }
}


