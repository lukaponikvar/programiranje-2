// kako pišeš teste za sprintane stvari;
// Un iterator?
// Velike črke????



use std::fmt::Display;
// Napišite različne funkcije, ki kot argument sprejmejo zaprtje (closure) in ga pokličejo z različnimi argumenti, ki ustrezajo sledečim ocaml tipom:
// Tip zaprtja naj bo čim bolj splošen (Fn, FnOnce, FnMut).
//  (int -> int) -> int -> int
//  ('a -> 'b) -> 'a -> 'b
//  ('a -> 'a -> 'b) -> 'a -> 'a -> 'b
//  map: ('a -> 'b) -> 'a list -> 'b list  (Uporabite Vec<T> namesto list, predpostavite, da funkcija ne spremeni elementov seznama)
//  map_and_mutate: ('a -> 'b) -> 'a list -> 'b list // Definirajte funkcijo tako, da lahko zaprtje spremeni elemente seznama
//  ponavljaj: int -> ('a -> 'a) -> 'a -> 'a // Ponovi zaprtje n-krat
//  filter: ('a -> bool) -> 'a list -> 'a list // Vrne seznam elementov, ki zadoščajo pogoju - uporabite Vec<T> namesto list in že vgrajeno funkcijo filter

// Vzemite zaporedja iz prejšnjih vaj in naredite nov objekt, ki sprejme zaporedje in ga naredi iterabilnega


// fn main() {
//     let k_dva = |x| {x * 2};
//     println!("{}", apply(k_dva, 10));
//     // ali
//     println!("{}", apply(|x| {x * 2}, 10));
//     // 
//     let neki = 3;
//     println!("{}", apply(|x| {x * neki}, 10));
//     println!("{}", apply_b(|x| {x * neki}, 10));
// }

// fn apply<F: FnOnce(i32) -> i32>(func: F, a: i32) -> i32 {
//     func(a)
// }

// fn apply_b<F: FnOnce(T1) -> T2, T1, T2>(func: F, a: T1) -> T2 {
//     func(a)
// }

// --------------------------------------------------------------------------------------------------------------------------------------
// ITERATORJI
// --------------------------------------------------------------------------------------------------------------------------------------

fn main() {
    naredi_iterator_1(vec![17, 4, 19024]);
    println!("-----");
    naredi_iterator_2(vec![String::from("17"), String::from("4"), String::from("19024")]);
    println!("-----");
    naredi_iterator_7(vec![None, None, None, Some(17), Some(4), Some(19024)]);
    println!("-----");
    naredi_iterator_7(vec![None, None, None, Some(String::from("17")), Some(String::from("4")), Some(String::from("anifdkjaodfij"))]);
}

// Iteratorji

// Napišite funkcijo, ki sprejme vektor XYZ in s pomočjo iteratorja naredi W
// števil in izpiše vsako v svojo vrstico
// nizov in izpiše njihove dolžine
// nizov in vrne vsoto njihovih dolžin
// vektor parov (i32, i32) in vrne vsoto njihovih pozitivnih produktov
// dva vektorja <i32> in vrne vektor, ki vsebuje vsote parov
// dva vektorja <i32> in vrne vsoto poparjenih pozitivni produktov s pomočjo ene izmed prejšnjih nalog
// vektor Option<T> in izpiše vse T-je
// vektor Option<T> in vrne število Some-ov
// odfiltrira števila deljena s 3

fn naredi_iterator_1(vec: Vec<i32>) {
    vec.iter().for_each(|x| println!("{x}"));
}

fn naredi_iterator_2(vec: Vec<String>) {
    vec.iter().for_each(|x| println!("{}", x.len()));
}

fn naredi_iterator_3(vec: Vec<String>) -> usize {
    let x = match vec.iter().map(|x| {x.len()}).reduce(|x,y| x+y) {
        Some(i) => i,
        None => 0,
    };
    x
}

fn naredi_iterator_4(vec: Vec<(i32,i32)>) -> i32 {
    vec.iter().map(|(x,y)| x*y).filter(|x| *x>0).sum()
}

fn naredi_iterator_5(vec1: Vec<i32>, vec2: Vec<i32>) -> Vec<i32> {
    vec1.iter().zip(vec2.iter()).map(|(x, y)| *x + *y).collect()   
}

fn naredi_iterator_6(vec1: Vec<i32>, vec2: Vec<i32>) -> i32 {
    vec1.iter().zip(vec2.iter()).map(|(x, y)| *x * *y).filter(|x| *x>0).sum()
}

fn naredi_iterator_7<T: Display>(vec: Vec<Option<T>>) {
    vec.iter().for_each(|x| match x {Some(x) => println!("{x}"), _ => ()});
}

fn naredi_iterator_8<T>(vec: Vec<Option<T>>) -> i32 {
    vec.iter().map(|x| {match x {Some(_) => 1, None => 0}}).sum()
}

// Dopolnite spodnjo funkcijo, da vrne niz, kjer so vse prve črke posameznih besed velike
// ["Just,", " ", "hello", " ", "world", "!"] -> "Just, Hello World", "!"
// pub fn capitalize_words_string(words: &[&str]) -> String {
//     words.iter().map(|x| &format("{}", (**x).capitalize())).reduce(|x ,y| x ^ y)
// }
// Napišite funkcijo `fakulteta`, ki izračuna fakulteto števila n. Uporabite iteratorje (torej brez lastne for zanke, rekurzije)
// Namig: fold, reduce, `..`...
// fn fakulteta(n: i32) -> i32 {

// }


// -------------------------------------------------------------------------------------------------
// Dodatno:
// Koda vzeta iz googlvih rust vaj:
// Vse se da lepo narediti samo z iteratorji (brez indeksov, brez for zank, brez mutabilnosti)
/*
// Calculate the differences between elements of `values` offset by `offset`,
// wrapping around from the end of `values` to the beginning.
//
// Element `n` of the result is `values[(n+offset)%len] - values[n]`.
fn offset_differences<N>(offset: usize, values: Vec<N>) -> Vec<N>
where
    N: Copy + std::ops::Sub<Output = N>,
{
    unimplemented!()
}

#[test]
fn test_offset_one() {
    assert_eq!(offset_differences(1, vec![1, 3, 5, 7]), vec![2, 2, 2, -6]);
    assert_eq!(offset_differences(1, vec![1, 3, 5]), vec![2, 2, -4]);
    assert_eq!(offset_differences(1, vec![1, 3]), vec![2, -2]);
}

#[test]
fn test_larger_offsets() {
    assert_eq!(offset_differences(2, vec![1, 3, 5, 7]), vec![4, 4, -4, -4]);
    assert_eq!(offset_differences(3, vec![1, 3, 5, 7]), vec![6, -2, -2, -2]);
    assert_eq!(offset_differences(4, vec![1, 3, 5, 7]), vec![0, 0, 0, 0]);
    assert_eq!(offset_differences(5, vec![1, 3, 5, 7]), vec![2, 2, 2, -6]);
}

#[test]
fn test_custom_type() {
    assert_eq!(
        offset_differences(1, vec![1.0, 11.0, 5.0, 0.0]),
        vec![10.0, -6.0, -5.0, 1.0]
    );
}

#[test]
fn test_degenerate_cases() {
    assert_eq!(offset_differences(1, vec![0]), vec![0]);
    assert_eq!(offset_differences(1, vec![1]), vec![0]);
    let empty: Vec<i32> = vec![];
    assert_eq!(offset_differences(1, empty), vec![]);
}



*/



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = main();
        assert_eq!(result, ());
    }

    #[test]
    fn test_naredi_iterator_3() {
        assert_eq!(naredi_iterator_3(vec![String::from("17"), String::from("4"), String::from("19024")]), 8);
        assert_eq!(naredi_iterator_3(vec![String::from(""), String::from(""), String::from("")]), 0);
        assert_eq!(naredi_iterator_3(vec![String::from(""), String::from("Lalalala"), String::from("")]), 8);
    }

    #[test]
    fn test_naredi_iterator_4() {
        assert_eq!(naredi_iterator_4(vec![(17,0), (4,-3), (19024,1)]), 19024);
        assert_eq!(naredi_iterator_4(vec![(17,0), (4,-3), (19024,-112)]), 0);
    }
    #[test]
    fn test_naredi_iterator_5() {
        assert_eq!(naredi_iterator_5(vec![17, 4,-3, 19, 024, 1, 0], vec![17, 4, -3, 19, 024, 1, 7]), vec![34, 8, -6, 38, 48, 2, 7]);
    }

    #[test]
    fn test_naredi_iterator_6() {
        assert_eq!(naredi_iterator_6(vec![1, 4,-3, 19, 1, 0], vec![17, 4, -3, 0, 1, 7]), 43);
    }

    fn test_naredi_iterator_8() {
        assert_eq!(naredi_iterator_8(vec![Some(17), Some(4), Some(19024), None]), 3);
        assert_eq!(
            naredi_iterator_8(
            vec![
                Some(String::from("17")),
                Some(String::from("4")),
                None, 
                None, 
                None, 
                Some(String::from("anifdkjaodfij")), 
                Some(String::from("anifdkjaodfij")), 
                Some(String::from("anifdkjaodfij"))
                ]
            ), 
            5);
    }
}