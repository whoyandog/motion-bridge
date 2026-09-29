fn main() {
    println!("cargo:rerun-if-changed=../proto/schema.proto");

    prost_build::compile_protos(&["../proto/schema.proto"], &["../proto"])
        .expect("Не удалось скомпилировать protobuf");
}
