fn main() {
    /*
    튜플
     */
    let p: (&str, u32) = ("Lee", 20);
    println!("name:{}, age={}", p.0, p.1); //name:Lee, age=20
    println!("{:?}", p); //("Lee", 20)

    let info = get_info();
    println!("age:{}, height={}", info.0, info.1); //age:20, height=60

    /*
    배열
     */
    //1. 배열의 선언과 사용
    let a = [1, 2, 3, 4, 5];
    println!("{}", a[0]); //1
    println!("{:?}", a); //[1, 2, 3, 4, 5]
    println!("{:?}", &a[0..2]); //[1,2]
    println!("{:?}", &a[3..]); //[4,5]

    //2. [값; 크기]로 여러 개의 값 지정
    let b = [1; 5];
    println!("{:?}", b); //[1, 1, 1, 1, 1]

    //3. 배열 선언 후 나중에 값 지정
    let b1: [i32; 5];
    b1 = [1; 5];
    println!("{:?}", b1); //[1, 1, 1, 1, 1]

    //4. 에러
    //let b2 = [1, 2.0]; //다른 데이터 타입

    // let x = [1,2,3];
    // println!("{}", x[3]); // 범위 밖 데이터 액세스

    //5. 벡터의 사용
    let mut v = vec![1, 2, 3];
    v.insert(3, 4);
    println!("{:?}", v); //[1,2,3,4]

    /*
    Rust에서는 배열 대신에 그냥 벡터를 사용하면 된다.
    Rust의 벡터는 크기를 변경할 수 있고, 원소를 더 추가할 수도 있으며, 정렬 등 편리한 메서드들도 사용할 수 있다.
    또한, 배열을 사용하는 것 대비 성능 저하도 거의 없다. 따라서, 굳이 배열을 사용할 필요 없다.
     */
}

fn get_info() -> (i32, f64) {
    let age = 20;
    let height = 60.5;

    return (age, height);
}
