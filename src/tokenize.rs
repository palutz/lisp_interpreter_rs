
type SExpres<'a> = (&'a str, Vec<&'a str>);

#[derive(Debug, PartialEq)]
enum Token<'a> {
    Atom   { a : &'a str },
    Symbol { s : &'a str },
    SExpr  { e : SExpres<'a> },
}

fn eval_sexpr(s: &str) ->  Vec<&str> {
    let inner = s.trim_start_matches("(").trim_end_matches(")");
    let words : Vec<&str> = inner.split_whitespace().collect();
    if words.len() == 0 {
        Vec::new()
    } else  {
        words 
    }
}

fn tokenizer(s: &str) -> Token {
    if s.starts_with("(") {
        Token::SExpr { e : ("uno", eval_sexpr(s))}
    } else if s.starts_with(":") {
        Token::Symbol { s }
    } else {
        Token::Atom { a: s }
    }
}

#[derive(Debug, PartialEq)]
enum TokenType {
    NotDef,
    OpenPar,
    ClosePar,
    Symbol, 
    Atom,
    AtomStr
}

type TokenT<'a>= (TokenType, String);

use TokenType::*;
fn tokenizer2(s: &str) -> Vec<TokenT> {
    let mut tokens : Vec<TokenT> = vec!();
    let mut buffer : TokenT = (NotDef, "".to_string());
    for c in s.chars() {
        match c {
            '\\' => (),
            '(' => { tokens.push((OpenPar, format!("{}",c))); buffer = (NotDef, "".to_string()) },
            ')' => { tokens.push((ClosePar, format!("{}",c))); buffer = (NotDef, "".to_string())} ,
            ':' => buffer = (Symbol, format!("{}",c)),
            ' ' => match buffer.0 {
                        Atom | Symbol => { tokens.push(buffer) ; buffer = (NotDef, "".to_string()) },
                        AtomStr => buffer = (buffer.0, format!("{}{}",buffer.1, c)),
                        _ => (),
                    },
            '"' => if buffer.0 == AtomStr {   // already in an Atom String, so close it
                        tokens.push(buffer);
                        buffer = (NotDef, "".to_string());
                    } else { buffer = (AtomStr, "".to_string()); },
            _   => match buffer.0 {
                        Atom | Symbol | AtomStr => buffer = (buffer.0, format!("{}{}",buffer.1, c)),
                        _ => buffer = (Atom, c.to_string()), 
                    }, 
        }
    }
    if buffer.0 != NotDef {
        tokens.push(buffer);
    }
    // tokens, get index of first open par until index is null,
    //then iterate over slices to return vec of them
    tokens
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
        assert_eq!(tokenizer("()"), Token::SExpr { e: ("", vec!()) });
        assert_eq!(tokenizer("(1)"), Token::SExpr { e: ("1", vec!["1"]) });
        assert_eq!(tokenizer("(1 2)"), Token::SExpr { e: ("1", vec!["1","2"]) });
        assert_eq!(tokenizer("(format t \"Hello, Coding Challenge World World\")"), 
                        Token::SExpr { e: ("format", vec!["format", "t", "\"Hello,","Coding","Challenge","World","World\""]) });
        assert_eq!(tokenizer("(defun hello () \"Hello, Coding Challenge World\")"), 
                        Token::SExpr { e: ("defun", vec!["defun", "hello", "()", "\"Hello,","Coding","Challenge","World\""]) });
    }

    #[test]
    fn test_token2() {
        let s = "(defun hello () \"Hello, Coding Challenge World\")";
        let v2 = tokenizer2(s);
        println!("{:?}", v2);
        assert!(true);
    }
}