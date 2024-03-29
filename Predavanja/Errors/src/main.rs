fn main() {
    println!("Hello, world!");
    println!("{:?}", delii(3,2));
    println!("{}", count(&vec![1.,2.,3.]))
}

#[derive(Debug)]
enum Morebitno<T> {
    Nimam,
    Imam(T),
}

fn deli(x: i32, y: i32) -> Morebitno<i32> {
    if y == 0 {
        Morebitno::Nimam
    } else {
        Morebitno::Imam(x/y)
    }
}

#[derive(Debug)]
enum MorebitnoZRazlogom<T,E> {
    Napaka(E),
    VRedu(T),
}

fn delii(x: i32, y: i32) -> MorebitnoZRazlogom<i32, String> {
    if y == 0 {
        MorebitnoZRazlogom::Napaka("delil si z nič".to_string())
    } else {
        MorebitnoZRazlogom::VRedu(x/y)
    }
}

fn count<T>(list: &[T]) -> i32 {
    let mut count = 0;
    for x in list {
        count += 1
    };
    count  
}