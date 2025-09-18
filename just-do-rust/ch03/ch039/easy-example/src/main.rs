fn main() {
    // map(), filter(), sum(), collect() 등을 이용해서 작성할 수 있는 간단한 예제
}

#[test]
fn simple_test1() {
    // 1~100까지 합
    let sum: usize = (1..=100).sum();
    println!("sum = {}", sum);

    //1에서10까지의 곱
    let product: usize = (1..=10).product();
    println!("product = {}", product);

    // 1~100에서 짝수의 합
    let sum: usize = (1..=100).filter(|x| x % 2 == 0).sum();
    println!("sum = {}", sum);

    // 1~100에서 3혹은 5의 배수의 합
    let sum: usize = (1..=100).filter(|x| x % 3 == 0 || x % 5 == 0).sum();
    println!("sum = {}", sum);

    // 1~100에서 3혹은 5의 배수의 개수
    let count: usize = (1..=100).filter(|x| x % 3 == 0 || x % 5 == 0).count();
    println!("count = {}", count);
}

#[test]
fn simple_test2() {
    let v: Vec<usize> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    //count(): 짝수 개수
    let count: usize = v.iter().filter(|&x| x % 2 == 0).count();
    println!("count = {}", count);

    //sum(): 벡터의 모든 원소 합
    let sum: usize = v.iter().sum();
    println!("sum = {}", sum);

    //product(): 벡터의 모든 원소 곱
    let product: usize = v.iter().product();
    println!("product = {}", product);

    //filter().product: 원소 중 4보다 작은 값에 대한 곱 : |&x|
    let product: usize = v.iter().filter(|&&x| x < 4).product();
    println!("product = {}", product);

    //take(n).product(): 원소중 앞으로부터 3개를 취해서 곱: take(n)
    let product: usize = v.iter().take(3).product();
    println!("product = {}", product);

    //take_while: filter와 달리, 조건식이 false이면 실행중단
    let product: usize = v.iter().take_while(|&&x| x < 5).product();
    println!("product = {}", product);

    //step_by(n).sum(): 4씩 건너뛰면서 읽어서 합
    let sum: usize = v.iter().step_by(4).sum();
    println!("sum = {}", sum);

    //filter().max(): 3의 배수 중 최댓값
    let max = v.iter().filter(|&&x| x % 3 == 0).max().unwrap();
    println!("max = {}", max);
}
