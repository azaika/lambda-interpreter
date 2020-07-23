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
        println!("input recognized! : {}", lambda);
    }
}