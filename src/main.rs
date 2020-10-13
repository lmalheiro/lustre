use lustre_lib::object::Object;
use std::sync::Arc;
use lustre_lib::environment::RefEnvironment;
use lustre_lib::evaluator::{operators::initialize_operators, eval};
use lustre_lib::reader::{Reader, tokenizer::Tokenizer};
use lustre_macro_reader;
use tokio::runtime::Runtime;

use std::io::{self, Read, Write};

fn main() {
    let b = io::stdin().bytes();

    let mut rt = Runtime::new().unwrap();
    let tokenizer = Tokenizer::new(b);
    let mut reader = Reader::new(tokenizer);

    let mut environment = RefEnvironment::new();
    initialize_operators(&mut environment);
    
    macro_rules! lustre {
        (for $env:expr; include $tokens:tt) => {
            eval(&lustre_macro_reader::lustre!{$tokens}, $env, &mut rt).unwrap();
        };
    }

    lustre! {
        for environment.clone(); include 
        (def 'fact
            (lambda (n)
                (if (> n 1)
                    (* n (fact (- n 1)))
                    1)))
    }
    lustre! {
        for environment.clone(); include 
        (def 'fib
             (lambda (n)
                 (if (< n 3)
	                 1
	                 (+ (fib (- n 1))
	                    (fib (- n 2))))))
    }

    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let ast = reader.read().unwrap();
        if ast.as_ref().is_some() {
            let result = eval(&ast, environment.clone(), &mut rt).unwrap();
            if let Some(v) = result.as_ref() {
                println!("* {:?}", v);
            } else {
                println!("* None");
            }
        } else {
            break;
        }
    }

}