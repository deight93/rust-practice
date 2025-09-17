fn main() {
    /*
    // 기본 사용법
    for <변수> in <시작되는 값>..=<마지막 값> {
      ...
    }
     */
    let mut sum = 0;
    // for i in 1..101 {
    // for i in 1..101{ sum += i; }
    for i in 1..=100 {
        sum += i;
    }
    println!("sum = {}", sum);

    // 단일 for 루프에서의 break
    let a = [2, 4, 10, 60, 80];
    let mut sum = 0;
    for i in a {
        if i > 50 {
            break;
        }
        sum += i;
    }
    println!("sum = {}", sum);

    // collection 원소에 대한 for 루프
    let v = vec![1, 2, 3, 4, 5];
    for val in v.iter() {
        println!("{}", val);
    }

    println!("=====");

    for val in &v {
        println!("{}", val);
    }
}

//이중 for 루프에서의 break;
#[test]
fn test6() {
    let mut v: Vec<(i32, i32)> = Vec::new();
    for i in 2..=10 {
        if i >= 4 {
            break;
        }
        for j in 2..=10 {
            if j >= 5 {
                break;
            }
            v.push((i, j));
        }
    }
    println!("{:?}", v); // [(2, 2), (2, 3), (2, 4), (3, 2), (3, 3), (3, 4)]
}

//안 쪽 루프에서 바같에 있는 루프를 벗어나려 할 때
// 자신이 속한 루프가 아닌 더 바같 쪽에 있는 루프를 벗어나려 할 때는 레이블(label)을 이용한다.
#[test]
fn test7() {
    let mut v: Vec<(i32, i32)> = Vec::new();
    'label_i: for i in 2..=10 {
        for j in 2..=10 {
            if j >= 5 {
                break 'label_i;
            }
            v.push((i, j));
        }
    }
    println!("{:?}", v); // [(2, 2), (2, 3), (2, 4)]
}
