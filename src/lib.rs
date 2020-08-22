use std::iter::FromIterator;

pub fn to_morse(input: &str) -> String {
    let data: Vec<&str> = input.chars()
        .map(|x| match x {
            ' ' => "....... ",
            'a' => ".- ",
            'b' => "-... ",
            'c' => "-.-. ",
            'd' => "-.. ",
            'e' => ". ",
            'f' => "..-. ",
            'g' => "--. ",
            'h' => ".... ",
            'i' => ".. ",
            'j' => ".--- ",
            'k' => "-.- ",
            'l' => ".-.. ",
            'm' => "-- ",
            'n' => "-. ",
            'o' => ".- ",
            'p' => ".--. ",
            'q' => "--.- ",
            'r' => ".-. ",
            's' => "... ",
            't' => "- ",
            'u' => "..- ",
            'v' => "...- ",
            'w' => ".-- ",
            'x' => "-..- ",
            'y' => "-.-- ",
            'z' => "--.. ",
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


