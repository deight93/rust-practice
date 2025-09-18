fn main() {
    // 조금 복잡한 사용예
}

#[test]
fn middle_example1() {
    //[1..100]에서 10의 배수값들에 대해서 10으로 나눈 후 합
    let sum: i32 = (1..=100).filter(|&x| x % 10 == 0).map(|x| x / 10).sum();
    println!("{}", sum);

    //[1..100]에서, 10보다 작은 수에서, 3의 배수 합
    let sum: i32 = (1..=100)
        .take_while(|&x| x < 10)
        .filter(|&x| x % 3 == 0)
        .sum();
    println!("{}", sum);
}

#[test]
fn string_test1() {
    //map().filter().map(): 문자열 중 숫자로된 문자열만 골라내서, 해당 문자열을 숫자로 변환해서 벡터로 만듦
    let a = ["1", "two", "NaN", "four", "5"];
    let v: Vec<i32> = a
        .iter()
        .map(|s| s.parse::<i32>())
        .filter(|x| x.is_ok())
        .map(|x| x.unwrap())
        .collect();
    println!("{:?}", v);

    //filter_map(): 위 예제와 동일
    let a = ["1", "two", "NaN", "four", "5"];
    let v: Vec<i32> = a.iter().filter_map(|x| x.parse::<i32>().ok()).collect();
    println!("{:?}", v);

    //문자 배열을 십진수 숫자 배열로 변환
    let a = ['1', '2', '3', 'f', '5'];
    let v: Vec<_> = a.iter().filter_map(|c| c.to_digit(10)).collect();
    println!("{:?}", v);
}

#[test]
fn string_test2() {
    //앞 5개 문자만 짤라내서, 숫자로 변환
    let input: &str = "73167176531330624919225119674426574742355349194934";
    let v: Vec<u32> = input
        .chars()
        .take(5)
        .filter_map(|c| c.to_digit(10))
        .collect();
    println!("{:?}", v);

    //5번째 숫자부터 5개 문자를 짤라내서, 숫자로 변환
    let input: &str = "73167176531330624919225119674426574742355349194934";
    // let v: Vec<u32> = input
    //     .chars()
    //     .skip(4)
    //     .take(5)
    //     .filter_map(|c| c.to_digit(10))
    //     .collect();
    let v: Vec<u32> = input[4..9].chars().filter_map(|c| c.to_digit(10)).collect();
    println!("{:?}", v);

    //숫자로 모두 변환 후 짝수만 더하기
    let input: &str = "73167176531330624919225119674426574742355349194934";
    let sum: u32 = input
        .chars()
        .filter_map(|c| c.to_digit(10))
        .filter(|&x| x % 2 == 0)
        .sum();
    println!("{}", sum);
}
