mod types;
mod lexer;
mod parser;
mod reduction;

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
        let (mut term, id2name) = term.ok().unwrap();

        let mut red = reduction::Reducer::new(id2name);
        let mut output : String = format!("{}", reduction::Formatter::new(&red, &term));

        let mut is_updated = true;
        while is_updated {
            println!("→{}", output);

            let (new_term, new_flag) = red.reduce(term);

            let new_output = format!("{}", reduction::Formatter::new(&red, &new_term));

            if new_flag && new_output == output {
                println!("→{}", new_output);
                println!("infinite loop!");
                return;
            }
            
            output = new_output;
            term = new_term;
            is_updated = new_flag;
        }
    }
}