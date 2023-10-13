
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

// TODO - refactor with Buffer as a State machine
fn tokenizer2(s: &str) -> Vec<TokenT> {
    use TokenType::*;
    let mut tokens : Vec<TokenT> = vec!();
    let mut buffer : TokenT = (NotDef, "".to_string());
    let s1 = s.replace("\n", "").replace("\t", "");
    for c in s1.chars() {
        // println!("{:?} = {:?}",buffer,c);
        match c {
            '\\' => (),
            '(' => { tokens.push((OpenPar, format!("{}",c))); buffer = (NotDef, "".to_string()) },
            ')' => { if buffer.0 != NotDef { tokens.push(buffer); };
                    tokens.push((ClosePar, format!("{}",c))); 
                    buffer  = (NotDef, "".to_string())} ,
            ':' => buffer = (Symbol, format!("{}",c)),
            ' ' => match buffer.0 {
                        Atom | Symbol => { if buffer.0 != NotDef { tokens.push(buffer); }; buffer = (NotDef, "".to_string()) },
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

#[derive(Debug, PartialEq)]
enum Expr {
    Atom(String),
    List(Vec<Expr>),
}

fn token2treeexpr<'a>(
    tokens: &mut Vec<TokenT>, 
    acc: &'a mut Vec<Expr>) 
{
    while tokens.len() > 0 {
        let t = tokens.remove(0);
        match t.0 {
            TokenType::OpenPar => { 
                    let mut buffer: Vec<Expr> = vec!();
                    token2treeexpr(tokens, &mut buffer);
                    acc.push(Expr::List(buffer));
            },
            TokenType::ClosePar => return,
            _ => acc.push(Expr::Atom(t.1)),
        }
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

    //#[test]
    // fn test_sexpr() {
    //     assert_eq!(tokenizer("()"), Token::SExpr { e: ("", vec!()) });
    //     assert_eq!(tokenizer("(1)"), Token::SExpr { e: ("1", vec!["1"]) });
    //     assert_eq!(tokenizer("(1 2)"), Token::SExpr { e: ("1", vec!["1","2"]) });
    //     assert_eq!(tokenizer("(format t \"Hello, Coding Challenge World World\")"), 
    //                     Token::SExpr { e: ("format", vec!["format", "t", "\"Hello,","Coding","Challenge","World","World\""]) });
    //     assert_eq!(tokenizer("(defun hello () \"Hello, Coding Challenge World\")"), 
    //                     Token::SExpr { e: ("defun", vec!["defun", "hello", "()", "\"Hello,","Coding","Challenge","World\""]) });
    // }

    #[test]
    fn test_token2() {
    use TokenType::*;
        let s = "(defun hello () \n \"Hello, Coding Challenge World\")";
        let v2 = tokenizer2(s);
        //println!("{:?}", v2);
        let expected = Vec::from([
            (OpenPar, "(".to_owned()), 
                (Atom, "defun".to_owned()), (Atom, "hello".to_owned()), 
                (OpenPar, "(".to_owned()), (ClosePar, ")".to_owned()), 
                (AtomStr, "Hello, Coding Challenge World".to_owned()), 
            (ClosePar, ")".to_owned())]);
        assert_eq!(v2, expected);
    }

    #[test]
    fn test_tokentoexpr() {
        let s = &mut tokenizer2("(defun hello () \"Hello, Coding Challenge World\")");
        let mut res :Vec<Expr> = vec!();
        token2treeexpr(s, &mut res);
        let expected = Vec::from(
            [Expr::List(
                Vec::from(
                    [Expr::Atom("defun".to_string()), Expr::Atom("hello".to_string()), 
                    Expr::List(vec!()), 
                    Expr::Atom("Hello, Coding Challenge World".to_string())
                ])
            )]);
        assert_eq!(res, expected);
        // testing the empty list of tokens
        let s1 = &mut tokenizer2("()");
        let mut res1 :Vec<Expr> = vec!();
        token2treeexpr(s1, &mut res1);
        let expected1 = Vec::from([Expr::List(vec!())]);
        assert_eq!(res1, expected1);
        let s2 = &mut tokenizer2("(\t\t\n\t\n)");
        let mut res2 :Vec<Expr> = vec!();
        token2treeexpr(s2, &mut res2);
        let expected2 = Vec::from([Expr::List(vec!())]);
        assert_eq!(res2, expected2);
    }

    #[test]
    fn test_tokentoexpr_fib() {
        use Expr::*;
        let s = &mut tokenizer2(
            "(defun fib (n)
                 (if (< n 2)
                     n
                     (+ (fib (- n 1))
                         (fib (- n 2)))))"
         );
        let mut res :Vec<Expr> = vec!();
        token2treeexpr(s, &mut res);
        let expected = Vec::from(
            [List(
                vec![
                    Atom("defun".to_string()), 
                    Atom("fib".to_string()), 
                    List(vec![Atom("n".to_string())])
                ])
            ]);
        println!("{:?}", res);
        assert!(true);
    }
}