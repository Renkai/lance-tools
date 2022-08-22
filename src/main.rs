use clap::{arg, Parser};


#[derive(clap::ValueEnum, Clone, Debug)]
enum Method {
    Inspect,
    Show,
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(value_enum)]
    method: Method,
}

fn main() {
    let args = Args::parse();

    match args.method {
        Method::Inspect => {
            println!("inspect!")
        }
        Method::Show => {
            println!("show!");
            todo!()
        }
    }
}
