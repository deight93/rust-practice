struct Point {
    x: i32,
    y: i32,
}

/*
1) 연관함수의 선언 : 구조체에서 선언되는 함수는 '연관 함수'라는 이름으로도 불린다.
일반 함수와 구분하기 위한 이름일 뿐이다. 구조체에 '연관'된 함수라는 의미다.

2) 메서드의 선언: 일반 함수와 동일하게 fn을 사용해서 선언한다.
단, &self 파라미터가 필수로 존재한다. fn distance(&self, p:&Point) -> f64

3) 연관 함수의 사용: 함수이기에 인스턴스에 의해 호출되지 않고 구조체 이름과 함께 호출된다.
Point::new(3,4)와 같이 구조체 이름에 ::를 붙여서 사용한다.

4) 메서드의 사용: 인스턴스 이름에 .을 붙이고 그 다음에 메서드 이름이 나온다. p1.distance(&p2)
 */

impl Point {
    fn new(x: i32, y: i32) -> Point {
        //1. 연관 함수
        Point { x: x, y: y }
    }

    fn distance(&self, p: &Point) -> f64 {
        //2. 메서드
        (((p.x - self.x).pow(2) + (p.y - self.y).pow(2)) as f64).sqrt()
    }
}

fn main() {
    let p1 = Point::new(0, 0);
    let p2 = Point::new(3, 4); //3. 연관함수의 사용
    assert_eq!(5.0, p1.distance(&p2)); //4. 메서드의 사용
}
