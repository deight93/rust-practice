/*
클로저의 선언은 다음과 같은 형태를 띈다.
let <변수명> = | <파라미터> | <표현식> ;
클로저의 사용은 변수명을 함수처럼 사용하면 된다.
 */
fn main() {
    // 1. 클로저 "|x| x+1"를 변수 add_one에 할당해서 함수처럼 사용가능
    let add_one = |x: i32| x + 1;
    println!("{}", add_one(2)); //3

    // 2. 함수와 달리 파라미터의 타입지정 안해도 됨. 자동 추정
    let add_one = |x| x + 1;
    println!("{}", add_one(2)); //3

    // 3. 파라미터가 없어도 된다.
    let print_hello = || println!("hello");
    print_hello(); //hello

    // 4. 바디는 {}로 감쌀 수 있다. 파라미터를 여러개 사용 가능
    let divmod = |x: i32, y: i32| {
        let q = x / y;
        let r = x % y;
        return (q, r);
    };
    println!("{:?}", divmod(10, 3)); //(3,1)

    /*
    앞에서 예로 든 클러저의 선언과 사용은 이렇게 사용될 수 있다는 것이지 이렇게 사용할 일은 별로 없다.
    함수와 다름없기 때문에 굳이 클로저를 쓰기보단 익숙한 함수를 정의해서 사용하는 것이 나을 것이다.
    함수가 아닌, 클로저를 꼭 써야만 하는 경우는 외부 변수를 함수의 내부에서 써야하는 경우다.
     */

    let num = 100;
    let add_num = |x| x + num;
    println!("{}", add_num(5)); //105

    let v = vec![1, 2, 3];
    assert_eq!(1, get_val(&v, 3));
    assert_eq!(3, get_val(&v, 2));

    let v: Vec<i32> = Vec::new();
    assert_eq!(0, get_val(&v, 1));

    let v: Vec<i32> = (1..=10000)
        .into_iter()
        .filter(|x| x % 3 == 0 || x % 5 == 0)
        .collect();
    assert_eq!(4667, v.len());
}

fn get_val(v: &Vec<i32>, idx: usize) -> i32 {
    let val = v
        .get(idx)
        .unwrap_or_else(|| if v.get(0).is_some() { &v[0] } else { &0 });
    return *val;

    /*
    let val = match v.get(idx) {
        Some(x) => x,
        None => {
            if v.get(0).is_some() {&v[0]}
            else {&0}
        },
    };
    return *val;
     */
}
