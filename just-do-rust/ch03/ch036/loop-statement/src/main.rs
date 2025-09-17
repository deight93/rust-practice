fn main() {
    let mut i = 1;
    loop {
        if i > 10 {
            break;
        }
        println!("i = {}", i);
        i += 1;
    }
    // 사실 위 코드는 loop의 예제로는 적절치 않다.
    // loop 구문 안에서 if i>10을 제일 먼저 체크하기 때문에 while 구문을 쓰는 것과 차이가 없다.

    let p = max_factor(10);
    println!("max factor of 10 is {}", p)
}

fn max_factor(mut n: u64) -> u64 {
    let mut p = 2;
    loop {
        let (q, r) = (n / p, n % p);

        if q == 1 {
            break;
        }

        if r == 0 {
            n = q;
        } else {
            p += 1;
        }
    }
    return n;
}
