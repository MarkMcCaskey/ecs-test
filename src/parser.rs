use lexer::Token::*;
use lexer::*;
use nalgebra::Point2;

#[derive(Debug, PartialEq)]
pub enum Ast {
    SpawnCommand { name: String, location: Point2<f32> },
    DeleteCommand { name: String },
}

fn parse_spawn(token_stream: &Vec<Token>) -> Result<Ast, &'static str> {
    if token_stream.len() != 5 {
        return Err("Wrong number of tokens");
    }

    if token_stream[0] == Spawn && token_stream[2] == At {
        if let (Name(name), Number(x), Number(y)) = (
            token_stream[1].clone(),
            token_stream[3].clone(),
            token_stream[4].clone(),
        ) {
            return Ok(Ast::SpawnCommand {
                name: name,
                location: Point2::new(x as f32, y as f32),
            });
        }
    }
    return Err("Invalid spawn command");
}

pub fn tokens_to_ast(token_stream: Vec<Token>) -> Result<Ast, &'static str> {
    parse_spawn(&token_stream)
}

pub fn parse_string(input: String) -> Result<Ast, &'static str> {
    tokens_to_ast(lex(input))
}

#[cfg(test)]
mod test {
    use nalgebra::Point2;
    use parser::*;

    #[test]
    fn end_to_end_parsing() {
        assert_eq!(
            parse_string("spawn commet at 123, 789".to_owned()),
            Ok(Ast::SpawnCommand {
                name: "commet".to_owned(),
                location: Point2::new(123., 789.),
            })
        );
    }
}
