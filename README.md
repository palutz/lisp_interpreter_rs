# Lisp interpreter in Rust

![Rust Version][rustc-image]
[![crates.io][crate-image]][crate-link]
[![Documentation][docs-image]][docs-link]
[![Dependency Status][deps-image]][deps-link]

## Code Challenges #30 

### Step 1
In this step your goal is to tokenise the expressions in a string. My personal approach to this would be to use test-driven development (TDD) to build tests for some example Lisp, i.e.:

*1* this will give you the atom with a value of 1.

*"Hello, Coding Challenges”* This is a string atom.

*:CC* this should be an atom (which will evaluate to the symbol :CC).

*(format t "Hello, Coding Challenge World World")* this is an s-expression which should tokenise as a list comprising of a symbol followed by a list.

*(defun hello () "Hello, Coding Challenge World").*

Be sure to add your own test cases too.

### Step 2
In this step your goal is to turn our tokenised string into an abstract syntax tree. By it’s nature, Lisp’s s-expressions can easily be turned into a binary tree. You could then represent that as an actual tree, or as a list, where some entries in the list are themselves lists.

For example consider the Lisp code: 
*(defun doublen (n) (* n 2))* you could represent that as a tree or as the list: *['defun', 'doublen', ['n'], ['*', 'n', 2]].*

Again, I’d suggest building some test cases. Here are some classic examples to use:

```
(defun fib (n)
  (if (< n 2)
      n
      (+ (fib (- n 1))
         (fib (- n 2)))))

(defun fact (n) 
  (if (<= n 1) 
    1 
    (* n (fact (- n 1)))))
```

### Step 3
In this step your goal is to be able to evaluate an AST and execute it. To be able to do that you’ll need to have some way to look up the symbols that have been tokenised. For example when your evaluation function encounters the *+* operator it will need to know to evaluate and add together the two arguments.

To be able to do this your code will need to be able to look up the symbol in a mapping from variable name to value. This should include support for standard functions, like addition, subtraction and so on as well as user-defined variables.

You can create nested mappings to support local variables - look look up the variable in the innermost mapping then work outwards to access the global scope.

For this challenge I’d suggest you support the *mathematical operators*, *if*, *defun*, *format* and the *comparison operators*.

You goal for this step is to be able to run some simple code:

```
(+ 1 2)
(* 2 3)
(defun doublen (n) (* 2 n))
(doublen 4)
```

### Step 4
In this step your goal is to implement a REPL (Read-Evaluate-Print Loop). Essentially a prompt that allows the user to enter and expression and immediately see it read, evaluated and the output printed. Like Python if your familiar with it.

Once done you should be able to run the code above through it:
```
cclisp> (+ 1 2)
3
cclisp>  (* 2 3)
6
cclisp> (defun doublen (n) (* 2 n))
DOUBLEN
cclisp> (doublen 4)  
8
```

[//]: # (badges)

[rustc-image]: https://img.shields.io/badge/rustc-1.60+-blue.svg
[crate-image]: https://img.shields.io/crates/v/{{project-name}}.svg
[crate-link]: https://crates.io/crates/{{project-name}}
[docs-image]: https://docs.rs/{{project-name}}/badge.svg
[docs-link]: https://docs.rs/{{project-name}}
[deps-image]: https://deps.rs/repo/github/palutz/lisp_interpreter_rs/status.svg
[deps-link]: https://deps.rs/repo/github/palutz/lisp_interpreter_rs
