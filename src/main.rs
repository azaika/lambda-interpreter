mod types;
mod lexer;
mod parser;

extern crate clap;

use clap::{App, Arg};

fn main() {
    let app = App::new("lambda-interpreter")
        .author("Takashi Nakayama <takashi-nakayama@g.ecc.u-tokyo.ac.jp>")
        .about("untyped lambda interpreter")
        .arg(Arg::with_name("LAMBDA")
            .help("set lambda to interpret like '\\x. x'")
            .required(true)
            .index(1)
        );
    
    let match_result = app.get_matches();

    if let Some(lambda) = match_result.value_of("LAMBDA") {
        let tokens = lexer::tokenize(lambda);
        if let Err(e) = &tokens {
            println!("{}", e);
            return;
        }
        let tokens = tokens.ok().unwrap();
        let term = parser::parse(&tokens);
        if let Err(e) = &term {
            println!("{}", e);
            return;
        }
        let term = term.ok().unwrap();

        println!("parsed! : {:?}", term);
    }
}