fn main() {
    println!("hello"); // hello

    let sum = 100;
    println!("a+b={}",sum); // a+b=100
    println!("a+b={sum}");  // a+b=100

    let v = vec![1,2,3];  // vec!도 매크로다. 벡터를 쉽게 만들어주는 매크로다.
    println!("{:?}",v);  //[1, 2, 3]
}
