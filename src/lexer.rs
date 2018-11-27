#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token {
    Spawn,
    Delete,
    At,
    Name(String),
    Number(u64),
}

fn lex_word(word_str: &str) -> Token {
    if word_str == "spawn" {
        return Token::Spawn;
    } else if word_str == "delete" {
        return Token::Delete;
    } else if word_str == "at" {
        return Token::At;
    } else if let Ok(num) = word_str.parse::<u64>() {
        return Token::Number(num);
    } else {
        return Token::Name(word_str.to_owned());
    }
}

pub fn lex(input: String) -> Vec<Token> {
    input
        .to_ascii_lowercase()
        .replace(',', "")
        .as_str()
        .split_whitespace()
        .map(lex_word)
        .collect()
}

#[cfg(test)]
mod test {
    #[test]
    fn lexes_correctly() {
        use super::Token::*;
        use super::*;
        assert_eq!(
            lex("Spawn commet at 123, 456".to_owned()),
            vec![
                Spawn,
                Name("commet".to_owned()),
                At,
                Number(123),
                Number(456),
            ]
        );
    }
}
