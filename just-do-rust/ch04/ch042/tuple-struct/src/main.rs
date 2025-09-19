struct Color(u8, u8, u8);
struct Point(i32, i32);
// 만약 Color를 일반적인 구조체로 선언하면 이런 형태가 될 것이다. 이에 비해 튜플 구조체가 심플하게 사용할 수 있어 좋다.
// struct Color {
//   r:i32,
//   g:i32,
//   b;i32,
// }

fn main() {
    // 구조체인데 필드명이 의미 없는 경우, 필드 이름 없이 정의하는 것이 튜플 구조체
    // 화면 픽셀의 r,g,b 값을 저장하는 데이터 구조, 좌표의 x, y, z 값을 저장하는 구조 등으로 유용하게 사용할 수 있다.

    // 튜플 구조체의 인스턴스 생성과 사용

    let p1 = Point(0, 0);
    let p2 = Point(3, 4);
    let distance = cal_distance(&p1, &p2);
    assert_eq!(distance, 5.0);
}

//cal_distance라는 함수를 만들었다. Point 타입 구조체 2개를 받아서, 두 점 사이의 거리를 계산 후 리턴한다.
// 이때 레퍼런스 타입으로 구조체를 받게 했다. fn cal_distance(p1:&Point, p2:&Point)
fn cal_distance(p1: &Point, p2: &Point) -> f64 {
    (((p2.0 - p1.0).pow(2) + (p2.1 - p1.1).pow(2)) as f64).sqrt()
    // cal_distance 함수내에서 거리를 계산할 때 제곱근 계산이 필요한데,
    // Rust에서는 부동소수점 타입인 f32와 f64에 대해서는 sqrt 메서드를 제공한다.
    // 따라서 as f64로 타입 변환을 한 후 sqrt()를 한다.
}
