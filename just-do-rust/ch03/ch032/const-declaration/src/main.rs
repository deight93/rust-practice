//const <상수명>: <타입> = <값>
const MAX: u32 = 1000; //함수 밖에서 선언. 모든 함수에서 사용가능, 상수명은 대문자 사용
//const MAX_VEC:Vec<&str> = vec!["a","b"];  //에러. 상수의 값으로 vec! 매크로 사용할 수 없음

fn main() {
    const MIN: u32 = 1;
    println!("{}, {}", MAX, MIN);
    sub_fn();

    const MAX_FLOAT: f64 = 1000.123;
    const MAX_STR: &str = "Max String";
    const MAX_ARR: [i32; 4] = [1, 2, 3, 4];
    println!("{}, {}, {:?}", MAX_FLOAT, MAX_STR, MAX_ARR);
}

fn sub_fn() {
    println!("{}", MAX);
}
