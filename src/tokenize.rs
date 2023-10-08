
type SExpres = (&'a str, Vec<&'a str>);

enum Token<'a> {
    Atom   { a : &'a str },
    Symbol { s : &'a str },
    SExpr  { e : SExpres },
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