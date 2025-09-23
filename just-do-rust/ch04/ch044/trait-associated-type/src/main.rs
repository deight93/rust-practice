// 트레잇이 함유하는 항목이 [타입, 상수, 메서드]임
// 타입 부분이 '연관 타입(Associated Type) 이다.
// 트레잇 안에 정의된 타입이고, 트레잇과 '연관'되어 있어서 연관 타입

// 1. 연관 타입 사용 이유
/*
왜 사용하는 걸까?

Rust 라이브러리 중 연관 타입을 사용한 트레잇을 살펴보면, 왜 사용되었는지를 아는데 도움이 될 것이다.
Rust에 보면 Iterator라는 트레잇이 있다. 정의는 다음과 같이 되어 있다. type Item이라고 되어 있는 것이 연관 타입이다.

pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
    //...

- next 메서드를 보면 'Self::Item' 타입의 값을 Option 형태로 리턴한다. 여기서 Self는 이 트레잇을 구현한 객체의 타입이다.
 */

trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
    // Iterator 트레잇에 정의된 next 메서드의 리턴은 Option(Self::Item)이다.
    // 여기서 Self::Item은 트레잇을 정의할 때는 결정되지 않는 타입이고, 이 트레잇을 구현할 때 결정되는 타입
}

struct Counter {
    count: u32,
}

impl Iterator for Counter {
    type Item = u32; // Counter 구조체용으로 Iter 트레잇을 구현할 때 type Item = u32로 Item 타입을 결정
    // 이와 같이, 연관 타입을 사용함으로써 트렛잇에 있는 메서드의 리턴 타입을
    // 구현 측에 위임할 수 있게 되었다. 트레잇을 유연하게 사용할 수 있게 만든 것이다.

    fn next(&mut self) -> Option<Self::Item> {
        if self.count > 100 {
            return None;
        } else {
            self.count += 1;
            return Some(self.count);
        }
    }
}

// 2. 연관 타입이 여러 개인 경우
struct Node {
    id: i32,
    name: String,
    value: i32,
}

struct Edge {
    start: Node,
    ene: Node,
}

trait Graph {
    type N;
    type E;

    fn get_value(&self, n: &Self::N) -> i32;
    fn edges(&self, n: &Self::N) -> Vec<Self::E>;
}

struct MyGraph {
    // nodes: Vec<Node>,
    // edges: Vec<Edge>,
}

impl Graph for MyGraph {
    type N = Node;
    type E = Edge;

    fn get_value(&self, n: &Self::N) -> i32 {
        return n.value;
    }

    fn edges(&self, e: &Self::N) -> Vec<Self::E> {
        todo!()
    }
    // MyGraph 구조체에 대해 Graph라는 트레잇을 구현한다.
    // 다른 그래프 구조가 있으면 그것도 Graph라는 트레잇을 구현하게 될 것이다.
    // 이렇게 하면, 어떠한 그래프 구조라도 Graph라는 공통된 특징을 가지게 될 것이다.
}

fn distance<G: Graph>(g: &G, s: &G::N, e: &G::N) -> i32 {
    // 어떤 그래프에서 두 노드 사이의 '거리'를 계산하는 함수
    g.get_value(e) - g.get_value(s)

    // Graph 트레잇을 구현한 모든 그래프를 사용할 수 있다. 위 쪽에서 MyGraph라는 구조체가 Graph를 impl한 바 있다.
    // 이 MyGraph를 이 함수의 파라미터로 보낼 수 있다.
    // 혹은, 다른 YouGraph라는 것을 만들었으면,
    // 이 구조체가 Graph를 impl 하기만 하면, 이 distance 함수에 이용될 수 있다.

    // Node에 대해서도 마찬가지로 추상화 되어 있다. 여
    // 기서는 G::N만 사용해서 Graph 안에 정의된 연관 타입을 사용했다.
    // 즉, 결국은 MyGraph안에서 타입 지정한 type N = Node;가 사용되는 것으로,
    // Graph를 정의할 시점에는 Node라는 구조체에 대한 정보가 없어도 된다.

    // 또 주의할만한 것은, distance 메서드를 정의할 때 G::E를 사용하지 않았다는 점이다.
    // 당연한 이야기일 수 있으나, 만약 '연관 타입'을 사용하지 않고
    // '제네릭' 같은 것을 썼으면 G::E도 사용해야 한다. (이 부분은 아래에서 좀 더 자세하게 설명)
}

// 3. '연관 타입'과 '제네릭' 비교
trait GraphGeneric<N, E> {
    //fn
    fn get_value(&self, n: N) -> i32;
    fn edges(&self, n: N) -> Vec<E>;
}
// 만약 Graph 트레잇에서 사용한 '연관 타입' type N; type E;를 다음과 같이 제네릭으로 선언했다고 해보자.
// 이렇게 해도 N과 E에 대해서 추상화가 되었다.
// 즉, 어떠한 Node와 Edge 타입으로도 Graph를 만들 수 있다. 연관 타입을 사용한 것과 유사한 효과

fn distance_generic<N, E, G: GraphGeneric<N, E>>(g: &G, s: N, e: N) -> i32 {
    // 제네릭으로 를 선언한 경우의 distance 함수 모습
    g.get_value(e) - g.get_value(s)
    // distance 함수의 파라미터로 Graph와 Node만이 필요함에도 불구하고 E를 같이 선언해야한다는 점

    // distance 함수에 제네릭 타입 E를 사용해야하는 이유는, GraphGeneric 트레잇에 제네릭 타입 가 사용되었기 때문

    // 결국, N, E를 연관 타입으로 사용하지 않고 제네릭으로 하게되면
    // 이처럼 N 타입만 필요한 경우에도 E 타입을 선언해야하는 불합리함이 생긴다.
    // 즉, N과 E에 대한 결합도가 높아진 것으로, 좋은 아키텍처가 아니게 된다.

    // 따라서, 이러한 형태의 트레잇에는, 트레잇 안에 '연관 타입'으로 타입을
    // 선언해서 사용하는 것이 '제네릭' 타입으로 하는 것보다 구조상 더 좋다.
}

fn main() {
    let counter = Counter { count: 12 };
    println!("{}", counter.count);
}
