fn main() {
    // https://github.com/awslabs/aws-sdk-rust.git ;  cd aws-sdk-rust
    // cargo +nightly rustdoc -p aws-sdk-ec2 -- -Zunstable-options -wjson
    let rustdoc_json = "/home/martin/src/aws-sdk-rust/target/doc/aws_sdk_ec2.json";
    let public_api = public_api::Builder::from_rustdoc_json(rustdoc_json)
        .sorted(false)
        .build()
        .unwrap();
    println!("{}", public_api);
}
