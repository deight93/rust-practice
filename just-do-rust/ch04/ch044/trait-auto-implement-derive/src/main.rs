/*
인스턴스에 대한 복제 메서드인 clone()을 사용하기 위해서
해당 인스턴스 타입인 Point 구조체에 대해 Clone 트레잇을 구현했어야 했다.

impl Clone for Point {
    fn clone(&self) -> Self {
        Self { x: self.x.clone(), y: self.y.clone() }
    }
}
 */

// 구현 자동화가 가능한 트레잇들에 대해 '자동으로 구현할 멤버들을 추출하고 구현하라'고 할 수 있다.
// 프로그래머가 직접 코드를 구현하는 것이 아니고, #[derive(Clone)]라고 Clone에 대해 자동 구현하라고 명시만 하면,
// Rust 컴파일러가 컴파일 과정에서 코드를 자동 생성해서 붙여 넣는다.

// 위에서 예를 든 Point에 대한 Clone 트레잇의 구현은,
// 아래와 같이 Point의 선언부 앞에 #[derive(Clone)]라는 어트리뷰트 한 줄만 써 놓으면 된다.
// 이 어트리뷰트가 있으면 컴파일 타임에 해당 트레잇에 대한 구현코드가 자동 생성되서 같이 컴파일 된다.
// 사용자가 따로 구현할 필요가 없어 편리하다.

/*
자동 구현이 가능한 트레잇은 다음과 같다.

- 비교관련 트레잇들: Eq, PartialEq, Ord, PartialOrd.
- 복제 : Clone
- 항목값의 복사(이동이 아닌): Copy
- 해시 계산: Hash
- 어떤 타입에 대해 비어있는 인스턴스 생성: Default
- {:?}를 사용한 String 생성: Debug

복수의 트레잇을 derive 하는 경우는 ,를 사용하면 된다.
예를 들어 Clone과 Copy를 둘 다 derive하는 경우는, #[derive(Clone, Copy)]
 */

fn main() {
    // Clone 예제
    #[derive(Clone, Debug)]
    struct Point {
        x: i32,
        y: i32,
    }

    let p1 = Point { x: 2, y: 3 };
    let p2 = Point { x: 4, y: 5 };
    let p3 = p1.clone();
    println!("{:?}", p3);
}
