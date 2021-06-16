use std::env;
use std::path::PathBuf;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    tonic_build::configure()
        .build_client(false)
        .file_descriptor_set_path(out_dir.join("user_svc.bin"))
        .compile(&["../proto/user_svc.proto"], &["../proto"])
        .unwrap();
}
