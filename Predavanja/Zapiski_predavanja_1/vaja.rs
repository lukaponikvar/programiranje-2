fn main() {
    let mut a = Box::new(5);
    povecaj(&mut a);
    let b = &&&&&Box::new(5);
    let c = sestej(&a, &b);
    println!("{} + {} = {}", *a, *b, c);
}

fn sestej (x: &Box<i32>, y: &Box<i32>) -> i32{
    return **x + **y 
}

fn povecaj(x: &mut Box<i32>) {
    **x += 1
}