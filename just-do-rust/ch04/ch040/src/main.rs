fn main() {
    // Rust에서는 소유권 개념 때문에 함수에 파라미터로 보낼 때 그냥 보낼지 참조로 보내야할 지를 결정하는 것이,
    // Rust 초보자에게는 어렵고, 이와 비슷하게, 기존 프로그래밍 상식으로는 유추하기 힘든 Rust의 독특한 특성들이 있다.
}

#[test]
fn test() {
    let strs: [String; 2] = ["Jeff".to_string(), "Park".to_string()];
    let concated_strs = concat(&strs);

    for s in strs.into_iter() {
        println!("{}", s);
    }
    println!("concated string: {}", concated_strs);
}

#[cfg(test)]
fn concat(strs: &[String; 2]) -> String {
    let mut new_strs = String::new();
    for s in strs.into_iter() {
        new_strs.push_str(s.as_str());
    }
    new_strs
}
