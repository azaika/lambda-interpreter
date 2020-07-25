mod types;
mod lexer;
mod parser;
mod reducer;

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
        )
        .arg(Arg::with_name("eval")
            .help("specify evaluation strategy (leftmost or rightmost)")
            .takes_value(true)
            .short("e")
            .long("eval")
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
            println!("{:?}", tokens);
            println!("{}", e);
            return;
        }
        let (mut term, id2name) = term.ok().unwrap();

        let mut red = reducer::Reducer::new(id2name);
        let mut output : String = format!("{}", reducer::Formatter::new(&red, &term));

        let mut is_updated = true;
        while is_updated {
            println!("→{}", output);

            let (new_term, new_flag) = if let Some(st) = match_result.value_of("eval") {
                if st == "leftmost" {
                    red.reduce_left(term)
                }
                else if st == "rightmost" {
                    red.reduce_right(term)
                }
                else {
                    println!("error : invalid evaluation strategy.");
                    return;
                }
            }
            else {
                red.reduce_left(term)
            };

            let new_output = format!("{}", reducer::Formatter::new(&red, &new_term));

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