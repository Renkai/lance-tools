// get docs from
// https://docs.rs/prost-build/latest/prost_build/

use std::io::Result;

fn main() -> Result<()> {
    prost_build::Config::new()
        // .type_attribute(".", "#[derive(Debug)]")
        .compile_protos(&["lance/protos/format.proto"], &["lance/protos/"])?;
    Ok(())
}
