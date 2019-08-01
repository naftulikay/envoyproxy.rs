use glob;

use std::path::PathBuf;

fn main() {
    let protos_to_build: Vec<PathBuf> = glob::glob("proto/envoy/api/v2/**/*.proto")
        .unwrap()
        .filter_map(|p| p.ok())
        .map(|p| p.into())
        .collect();

    prost_build::compile_protos(&protos_to_build.as_slice(), &[PathBuf::from("proto/")]).unwrap();
}
