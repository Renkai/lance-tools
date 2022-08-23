use std::fs;
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

    #[clap(value_parser)]
    file_path: String
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
            use lance_tools::protos::Metadata as MetaPB;
            use prost::Message;
            let mut meta_pb = MetaPB::default();
            meta_pb.manifest_position = 1024;
            println!("metapb: {:?}", meta_pb);
            let mut buf = Vec::new();
            meta_pb.encode(&mut buf);
            let decoded = MetaPB::decode(&*buf);
            println!("decoded: {:?}",decoded);
            use lance_tools::format::metadata::get_schema;

            println!("path: {:?}", args.file_path);
            let mut file = fs::File::open(args.file_path).unwrap();

            let metadata = get_schema(&mut file,0);
            println!("metadata {:?}",metadata)
        }
    }
}
