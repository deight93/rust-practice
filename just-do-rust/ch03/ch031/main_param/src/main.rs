use std::env;  // Rust에서 기본으로 제공하는 std::env라는 라이브러리를 사용하겠다는 의미

fn main() {
    let args: Vec<String> = env::args().collect();

    let src_file = &args[1];
    let tgt_file = &args[2];

    println!("Source File: {}", src_file);
    println!("Target File: {}", tgt_file);
}
