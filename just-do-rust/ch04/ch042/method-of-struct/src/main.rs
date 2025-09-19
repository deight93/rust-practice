struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x: x, y: y }
    }

    fn distance(&self, p: &Point) -> f64 {
        (((p.x - self.x).pow(2) + (p.y - self.y).pow(2)) as f64).sqrt()
    }
}

fn main() {
    // Rust에서는 구조체에 딸린 메서드를 만들 수 있다. 정확하게 얘기하면 '연관함수'와 '메서드'를 만들 수 있다.

    // 연관 함수: 구조체의 인스턴스를 만들지 않고도 사용할 수 있는 함수.
    // <구조체 이름>::<함수명>과 같은 형태로 호출할 수 있다. Java에서 클래스의 정적 메서드와 유사한 거다.
    // 메서드: 구조체의 인스턴스에 의해 호출될 수 있는 메서드

    // Rust에서는 함수(function)와 메서드(method)라는 명칭을 구분해서 사용한다.
    // 모두 fn이라는 키워드로 생성된다. 편이상, 함수외 메서드를 합쳐서 'fn'이라고 함.

    // Rust에서 함수는 인스턴스와 상관 없는 fn이고, 메서드는 구조체 혹은 트레이트의 인스턴스에 의해 호출되는 fn이다.
    let p1 = Point::new(0, 0);
    let p2 = Point::new(3, 4);
    assert_eq!(5.0, p1.distance(&p2));
    println!("p1.distance(&p2) = {}", p1.distance(&p2));

    /*
    - Point라는 구조체를 정의했다. 앞 페이지에서 예를 들었던 튜플 구조체가 아니라,
        일반 구조체로 선언했다. x와 y라는 필드를 가진다.
    - impl Point { ... }라고해서 Point 구조체에 대한 fn 들을 구현한다.
    - fn new(x:i32, y:i32) -> Point는 연관 함수다. 파라미터에 &self라는 변수가 없기 때문이다.
        연관함수는 Point::new(0,0)과 같이 구조체의 이름에 ::를 붙이고 그 뒤에 함수이름을 둬서 호출한다.
        (String 인스턴스를 생성할 때 String::new()라고 해서 비어있는 String 인스턴스를 만드는데, 이게 연관함수다.)
    - fn distance는 파라미터에 &self가 있다. 구조체의 메서드다.
        Point의 인스턴스에 의해서만 호출될 수 있는 것이 메서드이고, &self는 호출한 인스턴스의 참조자 변수이다.
    - new 함수는 let p1 = Point::new(0,0);과 같이 사용된다. 구조체 이름 Point만을 사용해서 호출할 수 있다.
    - distance 메서드는 인스턴스에 의해서만 호출될 수 있다. 그래서, p1.distance(&p2)와 같이 사용된다.
     */

    let p3 = Point::new(-3, 5);
    let max = max_distance(&p1, &p2, &p3);
    println!("max = {}", max);

    /*
    추가된 코드에서 얘기하고 싶은 것은 max_distance 함수에서 파라미터로 받은
    (p1:&Point, p2:&Point, p3:&Point)에 대해서 p1.distance라고 호출하는 부분에 대해서다.

    파라미터로 받은 것이 &Point처럼 레퍼런스로 받았다.
    따라서, p1에 대한 메서드를 호출하려면 (*p1).distance라고 해야할 것 같다.
    문법상으로는 (*p1).distance라고 하는게 맞고, 이렇게 해도 동작한다.

    Rust에서는 object.something() 코드로 메서드를 호출하면, 자동으로 해당 메서드의 시그니처에 맞도록 &, &mut, *를 추가한다. 편리하다.

    따라서, 위 코드에서도 그냥 p1.distance라고 사용한 것이다.
     */
}

fn max_distance(p1: &Point, p2: &Point, p3: &Point) -> f64 {
    let d1 = p1.distance(p2);
    let d2 = p2.distance(p3);
    let d3 = p3.distance(p1);
    // let d1 = (*p1).distance(p2);
    // let d2 = (*p2).distance(p3);
    // let d3 = (*p3).distance(p1);

    let mut max = d1;
    if d2 > max {
        max = d2;
    }
    if d3 > max {
        max = d3;
    }
    max
}
