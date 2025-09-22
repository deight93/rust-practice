// Result는 Option과 더불어 Rust에서 가장 많이 사용되는 열거형이다.
// Option은 "값이 없는 경우"를 위한 열거형이고, Result는 "에러가 발생한 경우"를 위한 열거형이다.
//
// Option: Some, None
// Result: Ok, Err
// Result에서 중점으로 알아야할 사항은 어떻게 Err을 처리하는가다.
// 이것을 알면, Rust에서의 에러처리 방법의 거의 모든 것을 알게되는 것이다.

// 열거형 Result의 구조
// Result는 'Ok'와 Err 두 개의 타입을 원소로 갖는다. Ok에는 T라는 제네릭 탑을 품고있고, Err에는 E라는 제네릭 타입을 품고 있다.
//
/*
    enum Result<T, E> {
        Ok(T),
        Err(E),
}*/
// Ok 타입에는 성공했을 때 실제 리턴하는 값이 들어가게 되고, Err에는 에러 값이 들어가게 된다.

/*
   tip

   제네릭 타입에 대해서 익숙하지 못한 독자을 위해서 잠시 설명하면,
   Result에 있는 <T, E>는 제네릭 타입으로 T와 E라는 기호를 쓰겠다는 의미이고,
   Result라는 것을 사용할 때 Result와 함께 를 같이 사용한다는 것이 아니다.
   "Result라는 열거형에서 제네릭 기호로 T와 E라는 것을 사용할 것이고,
   Ok에 전달되는 무작위 타입을 T로, Err에 전달되는 무작위 타입을 E로 약속한다"는 의미다.
*/

fn main() {
    // 파이썬에는 몫과 나머지를 리턴하는 divmod라는 함수가 있다.
    // Rust에는 없는데, 이 함수를 만들면서 Result에 대해 설명해보고자 한다.

    // 나누어지는 수(n, numerator)와 나누는 수(d, divisor)를 주면 몫(q, quotient)과 나머지(r, remainder)를 리턴
    let (q, r) = divmod(10, 3);
    println!("{} {}", q, r);

    // 그러나 divmod(10,0)을 하게되면 panic, 0으로 나누는 나눗셈 연산은 불가능하기 때문
    // let (q, r) = divmod(10, 0); // panic
    // println!("{} {}", q, r);

    // panic에 빠지지 않고 Err를 리턴하게 해보자. Result 열거형을 사용하는 것
    // 타입에 따라 분기해서 처리하는 것은 match를 이용
    match divmod_result(10, 0) {
        Ok((q, r)) => println!("{} {}", q, r),
        Err(e) => println!("{}", e),
    }
}

fn divmod(n: i32, d: i32) -> (i32, i32) {
    // Result를 적용하지 않고 divmod 함수
    (n / d, n % d)
}

fn divmod_result(n: i32, d: i32) -> Result<(i32, i32), String> {
    // divmod의 리턴값을 Result 타입으로 지정
    // Result 열거형을 사용
    if d == 0 {
        Err("division by zero".to_string())
    } else {
        Ok((n / d, n % d)) // 일반적인 경우는 몫과 나머지를 튜플로 만들어서 Ok에 담아 리턴
    }
    // 이제 이 divmod를 호출하면 그냥 튜플값이 리턴되는 것이 아니라 Result 타입이 리턴된다.
    // 정확하게 얘기하면, Result::Ok 혹은 Result::Err 타입 중 하나가 얻어진다.
    // (Option에서 Some과 None 앞에 Option::를 붙이지 않아도 되었다.
    // Result도 마찬가지다. 그냥 Ok Err를 쓰면 된다. )
}
