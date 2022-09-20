use lynxlang::env::Env;
use lynxlang::evaluator::Evaluator;
use lynxlang::parser::Parser;

use std::cell::RefCell;
use std::io;
use std::io::Write;
use std::process;
use std::rc::Rc;

pub fn repl() -> io::Result<()> {
    let stdin = io::stdin();
    // let mut stderr = io::stderr();
    let mut stdout = io::stdout();

    let mut input = String::new();
    // let mut env = eldiro::Env::default();

    loop {
        writeln!(stdout, ">>> Welcome to the Lynx Programming Language - REPL")?;
        writeln!(
            stdout,
            r#"Type "help", "copyright", "credits" or "license" for more information."#
        )?;
        write!(stdout, ">>> ")?;
        stdout.flush()?;

        stdin.read_line(&mut input)?;

        let value = input.trim();

        let program = Parser::get(value).parse_program();

        let mut evaluator = Evaluator::new(Rc::new(RefCell::new(Env::new())));

        evaluator.builtin();

        if value == ".exit" {
            writeln!(stdout, "exit");
            process::exit(0);
        }

        match evaluator.eval_program(program) {
            Some(value) => writeln!(stdout, "{:?}", value),
            None => writeln!(stdout, "Null"),
        };

        input.clear();
    }
}
