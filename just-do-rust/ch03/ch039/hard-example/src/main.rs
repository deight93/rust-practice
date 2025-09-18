fn main() {
    // 복잡한 예제다. 어떤 Iterator Adapter를 써야할 지 고민해서 결정하고,
    // 여러 개 Iterator Adapter를 사용해야하는 경우
}

#[test]
fn string_test3() {
    //3칸씩 이동 후 3자리씩 짤라내면서, 숫자 정수로 변환하기
    let input: &str = "73167176531330624919225119674426574742355349194934";
    let sum: u32 = input
        .chars()
        .collect::<Vec<char>>()
        .chunks(3)
        .map(|c| c.iter().collect::<String>())
        .filter_map(|s| s.parse::<u32>().ok())
        .sum();
    println!("{}", sum);

    //1칸씩 이동 후 3자리씩 짤라내면서, 숫자 정수로 변환하기
    let input: &str = "73167176531330624919225119674426574742355349194934";
    let sum: u32 = input
        .chars()
        .collect::<Vec<char>>()
        .windows(3)
        .map(|c| c.iter().collect::<String>())
        .filter_map(|s| s.parse::<u32>().ok())
        .sum();
    println!("{}", sum);
}

#[test]
fn complicated_test() {
    // 소수를 구할 때 사용되는 Iterator Adapter 예

    // 체(sieve) 이용해서 소수 구하기
    let sieve = [
        0, 0, 1, 1, 0, 1, 0, 1, 0, 0, 0, 1, 0, 1, 0, 0, 0, 1, 0, 1, 0,
    ]; //prime number sieve
    let v: Vec<u32> = sieve
        .iter()
        .enumerate()
        .filter(|(_, val)| **val == 1)
        .map(|(i, _)| i as u32)
        .collect();
    println!("{:?}", v);

    //10000에서 20000사이의 소수 구한 후, 합 구하기
    let sum: u32 = (10000..=20000).filter(|i| is_prime(*i)).sum();
    println!("{}", sum);
}

#[cfg(test)]
fn is_prime(t: u32) -> bool {
    if t % 2 == 0 {
        return false;
    }

    let sqrt_n = (t as f64).sqrt() as u32;
    let mut i: u32 = 3;
    while i <= sqrt_n {
        if t % i == 0 {
            return false;
        }
        i += 2;
    }
    true
}
