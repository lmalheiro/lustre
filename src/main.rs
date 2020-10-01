use lustre_lib::object::Object;
use std::sync::Arc;
use lustre_lib::environment::Environment;
use lustre_lib::evaluator::{operators::initialize_operators, Evaluator};
use lustre_lib::reader::{Reader, tokenizer::Tokenizer};
use lustre_macro_reader;

use std::io::{self, Read, Write};

fn main() {
    let b = io::stdin().bytes();

    let tokenizer = Tokenizer::new(b);
    let mut reader = Reader::new(tokenizer);

    let mut environment = Environment::new();
    initialize_operators(&mut environment);
    let mut evaluator = Evaluator::new(&mut environment);

    macro_rules! lustre {
        (for $evaluator:ident include $tokens:tt) => {
            $evaluator.eval(&lustre_macro_reader::lustre!{$tokens}).unwrap();
        };
    }

    lustre! {
        for evaluator include 
        (def 'fact
            (lambda (n)
                (if (> n 1)
                    (* n (fact (- n 1)))
                    1)))
    }
    lustre! {
        for evaluator include 
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
            let result = evaluator.eval(&ast).unwrap();
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