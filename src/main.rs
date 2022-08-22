use std::fs::Metadata;
use clap::{arg, Parser};


#[derive(clap::ValueEnum, Clone, Debug)]
enum Method {
    Inspect,
    Show,
    Lab,
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
        Method::Lab => {
            use lance_tools::protos::Metadata;
            use prost::Message;
            let mut metadata = Metadata::default();
            metadata.manifest_position = 1024;
            println!("metadata: {:?}", metadata);
            let mut buf = Vec::new();
            metadata.encode(&mut buf);
            let decoded = Metadata::decode(&*buf);
            println!("decoded: {:?}",decoded)
        }
    }
}
