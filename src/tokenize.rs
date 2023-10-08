
type SExpres<'a> = (&'a str, Vec<&'a str>);

#[derive(Debug, PartialEq)]
enum Token<'a> {
    Atom   { a : &'a str },
    Symbol { s : &'a str },
    SExpr  { e : SExpres<'a> },
}

fn eval_sexpr(s: &str) ->  SExpres {
    todo!()
}

fn tokenizer(s: &str) -> Token {
    if s.starts_with("(") {
        Token::SExpr { e : eval_sexpr(s)}
    } else if s.starts_with(":") {
        Token::Symbol { s }
    } else {
        Token::Atom { a: s }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_atom() {
        assert_eq!(tokenizer("1"), Token::Atom { a: "1" });
        assert_eq!(tokenizer("this is an atom string"), Token::Atom { a: "this is an atom string" });
    }

    #[test]
    fn test_symbol() {
        assert_eq!(tokenizer(":CC"), Token::Symbol { s: ":CC" });
    }

    #[test]
    fn test_sexpr() {
        todo!()
        // assert_eq!(tokenizer("(1)"), Token::SExpr { e: ("1", Vec::new()) });
        // assert_eq!(tokenizer("(1 2)"), Token::SExpr { e: ("1", vec!["2"]) });
        // assert_eq!(tokenizer("(1 2 3)"), Token::SExpr { e: ("1", vec!["2", "3"]) });
    }
}