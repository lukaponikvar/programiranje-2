use core::panic;

/// Skupaj preverite in pokomentirajte kvize iz [učbenika](https://rust-book.cs.brown.edu/ch03-00-common-programming-concepts.html)

/// ------------------------------------------------------------------------------------------------

/// Napišite funkcijo `fib`, ki sprejme začetna člena fibbonacijevega zaporedja, število `n` in vrne `n`-ti člen zaporedja

fn fib(a0: u32, a1: u32, n: u32) -> u32 {                          //panic!("Not implemented");
    match n {
        0 => a0,
        1 => a1,
        _ => fib(a1, a0 + a1, n-1)
    }
}

/// ------------------------------------------------------------------------------------------------

/// Napišite funkcijo `je_prestopno`, ki za podano leto preveri, ali je prestopno

fn je_prestopno (n: u32) -> bool {
    if n%400 == 0 {
        true
    } else if n%100 == 0 {
        false
    } else if n%4 == 0 {
        true
    } else {false}
}

/// ------------------------------------------------------------------------------------------------

/// Napišite funkcijo `je_veljaven_datum(datum: Date) -> bool`, ki preveri, ali je datum veljaven

// Dan, mesec, leto

type Date = (u32, u32, u32);

fn je_veljaven_datum(datum: Date) -> bool {
    let (dan, mesec, leto) = datum;
    let seznam: [u32; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    if mesec > 12 {false}
    else if mesec == 2 && je_prestopno(leto) {
        dan <= 29
    } else {
        dan <= seznam[(mesec-1) as usize]
    }
}

/// ------------------------------------------------------------------------------------------------

/// Napišite funkcijo `iteracija(mut start: u32, fun: fn(u32) -> u32, cond: fn(u32) -> bool) -> u32`, ki sprejme iteracijsko funkcijo, zaustavitveni pogoj in začetno vrednost.
/// Iteracijsko funkcijo zaporedoma uporablja, dokler za rezultat ne velja zaustavitveni pogoj, in vrne prvi rezultat, ki zadošča zaustavitvenemu pogoju.

fn iteracija(start: u32, fun: fn(u32) -> u32, cond: fn(u32) -> bool) -> u32 {
    if cond(start) {
        start
    } else{
        iteracija(fun(start), fun, cond)
    }
}

/// ------------------------------------------------------------------------------------------------

/// Napišite funkcijo, ki izračuna ničlo zvezne funkcije s pomočjo bisekcije.
/// Postopek bisekcije je sledeč:
/// 1. Izberemo interval [a, b], kjer je f(a) * f(b) < 0
/// 2. Izračunamo sredino intervala c = (a + b) / 2
/// 3. Če je |f(c)| < prec ali je dolžina intervala manjša od določene natančnosti, vrnemo c
/// 4. Če ni, izberemo nov interval [a, b] glede na predznak f(c)
/// 5. Ponavljamo korake 2-4

fn bisekcija(a: f64, b: f64, fun: fn(f64) -> f64, prec: f64) -> f64 {
    let c = (a + b)/ 2.0;
    println!("{}", c);
    if (b-a < prec) || (fun(c).abs() < prec) {
        c
    } else if  (fun(a) * fun(c)) > 0.0 {
        bisekcija(c, b, fun, prec)
    } else {
        bisekcija(a, c, fun, prec)
    }
}

/// ------------------------------------------------------------------------------------------------

/// Popravite igro ugibanja iz prejšnje naloge, da bo delovala sledeče
/// Uporabnika sprašujemo po novi številki, vse dokler so števila, ki jih vpisuje del nekega aritmetičnega zaporedja
/// Če uporabnik vpiše neveljavno število to ni napaka, program za pogoj aritmetičnega zaporedja upošteva samo veljavno vpisana števila.

fn guessing_game() {
    fn ugibaj() -> u32{
        println!("Vpiši število");
        let mut ugib = String::new();
        let ugib= loop {
            match std::io::stdin().read_line(&mut ugib) {
                Ok(niz) => ugib.clone(),
                Err(_) => continue,                    
            };

            match ugib.trim().parse() {
                Ok(num) => return num,
                Err(_) => continue,
            };
        };
    }
    println!("Igraš igro Aritmetična_Zaporedja.\nIgra poteka tako, da vpisuješ nova števila dokler ta tvorijo aritmetično zaporedje, če se zmotiš, se igra konča.");
    let mut prvi = ugibaj();
    let mut drugi = ugibaj();
    let razlika = drugi - prvi;
    loop {
        let novi = ugibaj();
        if novi -drugi == razlika {
            prvi = drugi;
            drugi = novi
        } else { break }
    };
    println!("Igra se je končala! Razlika je bila {razlika}.")
}

/// ------------------------------------------------------------------------------------------------
/// Napišite funkcijo `fn mat_mul(a: [[u32; 2]; 2], b: [[u32; 2]; 2]) -> [[u32; 2]; 2]`, ki matriki `a` in `b` zmnoži in vrne rezultat

fn mat_mul(a: [[u32; 2]; 2], b: [[u32; 2]; 2]) -> [[u32; 2]; 2] {
    let [[a11, a12], [a21, a22]] = a;
    let [[b11, b12], [b21, b22]] = b;
    return [[a11*b11 + b21 * a12, a11*b12 + b22* a12], [a21*b11 + b21 * a22, a21*b12 + b22* a22]]
}

/// ------------------------------------------------------------------------------------------------
/// Napišite funkcijo `ordered`, ki sprejme tabelo števil in vrne `true`, če so števila urejena (padajoče ali naraščajoče) in `false` sicer.

fn ordered(arr: &[u32]) -> bool {
    let dolzina = arr.len();
    if dolzina <= 2 {
        return true
    } else if arr[0] < arr[1] {
        for indeks in 1..dolzina-1 {
            if arr[*&indeks]>arr[*&indeks+1] {
                return false
            }
        }
    } else {
        for indeks in 1..dolzina-1 {
            if arr[*&indeks]>arr[*&indeks+1] {
                return false
            }
        }
    }
    return true
}

/// ------------------------------------------------------------------------------------------------
/// Hitro potenciranje
/// Napišite funkcijo `fn pow(mut x: u32, mut n: u32) -> u32`, ki izračuna `x` na potenco `n` v času O(log n)
/// Hitro potenciranje izgleda tako:
/// 1. Če je `n` sodo, potem je `x^n = (x^(n/2))^2`
/// 2. Če je `n` liho, potem je `x^n = (x^2)^(n/2)`
/// 3. Če je `n = 0`, potem je `x^n = 1`

fn pow(x: u32, n: u32) -> u32 {
    if n == 0 {
        return 1
    } else if n == 1{
        return x
    } else if n == 2 {
        return x*x
    } else if n % 2 == 0 {
        return pow(pow(x, n/2), 2)
    } else {
        return x * pow(pow(x, (n-1)/2), 2)
    }
}

/// ------------------------------------------------------------------------------------------------
/// Prepišite hitro potenciranje v iterativno obliko

/// ------------------------------------------------------------------------------------------------
/// Hitro potenciranje deluje tudi, če nas zanima samo ostanek po deljenju z nekim številom `m`
/// Napišite funkcijo `fn pow_mod(mut x: u32, mut n: u32, m: u32) -> u32`, ki izračuna `x` na potenco `n` in vrne ostanek po deljenju z `m`
/// Postopek je enak, le da pri vsakem izračunu vrnemo ostanek pri deljenju z `m`

fn pow_mod(x: u32, n: u32, m: u32) -> u32 {
    if n == 0 {
        return 1 % m
    } else if n == 1{
        return x % m
    } else if n == 2 {
        return (x*x) % m
    } else if n % 2 == 0 {
        return pow(pow(x, n/2) % m, 2)
    } else {
        return x * pow(pow(x, (n-1)/2) % m, 2) % m
    }
}

/// ------------------------------------------------------------------------------------------------
/// Urejanje z izbiranjem
/// Napišite funkcijo `fn selection_sort(arr: &mut [u32])`, ki uredi tabelo `arr` z uporabo algoritma urejanja z izbiranjem

fn selection_sort(arr: &mut[u32]) -> &[u32] {
    let l = arr.len();
    if l <= 1 {
        return arr
    }
    fn minimum (ar: &[u32]) -> usize {
        let li = ar.len();
        let mut m = ar[0];
        let mut indeks = 0;
        for i in 0..li {
            if ar[i] < m {
                m = ar[i];
                indeks = i;
            }
        } 
        indeks
    }
    for i in 0..(l-1) {
        let novi = &arr[i..=l-1];
        let indeks = minimum(novi) + i;

        let a = arr[i];
        let b = arr[indeks as usize];
        arr[i] = b;
        arr[indeks as usize] = a;
    }
    println!("{:?}", arr);
    arr
}

/// ------------------------------------------------------------------------------------------------
/// Napišite program, ki izpiše piramido višine `n` iz zvezdic

fn pyramid(n: u32) {
    panic!("Not implemented");
}

/// ------------------------------------------------------------------------------------------------
/// Napišite program, ki izpiše piramido črk angleške abecede višine `n`, lahkom predpostavite, da bo n največ 26.
///      A
///    A B A
///   A B C B A
/// A B C D C B A

fn main() {
    guessing_game();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = main();
        assert_eq!(result, ());
    }

    #[test]
    fn test_fib() {
        assert_eq!(fib(0, 1, 0), 0);
        assert_eq!(fib(0, 1, 1), 1);
        assert_eq!(fib(0, 1, 2), 1);
        assert_eq!(fib(0, 1, 3), 2);
        assert_eq!(fib(0, 1, 4), 3);
        assert_eq!(fib(0, 1, 5), 5);
        assert_eq!(fib(0, 1, 6), 8);
        assert_eq!(fib(0, 1, 7), 13);
        assert_eq!(fib(0, 1, 8), 21);
        assert_eq!(fib(0, 1, 9), 34);
        assert_eq!(fib(0, 1, 10), 55);
    }
        #[test]
    fn test_je_prestopno() {
        assert_eq!(je_prestopno(400), true);
        assert_eq!(je_prestopno(100), false);
        assert_eq!(je_prestopno(40), true);
        assert_eq!(je_prestopno(99), false);
    }

    #[test]
    fn test_je_veljaven_datum() {
        assert_eq!(je_veljaven_datum((1,1,1)), true);
        assert_eq!(je_veljaven_datum((32,1,1)), false);
        assert_eq!(je_veljaven_datum((1,13,1)), false);
        assert_eq!(je_veljaven_datum((1,1,212112121)), true);
        assert_eq!(je_veljaven_datum((29,2,1)), false);
        assert_eq!(je_veljaven_datum((29,2,2024)), true);
        assert_eq!(je_veljaven_datum((17,8,1999)), true);
        assert_eq!(je_veljaven_datum((111,111,111)), false);
    }

    fn manjsi_od7(stevilo: u32) -> bool {
        stevilo < 7
    }

    fn pristej1(a: u32) -> u32 {
        a+1
    }

    fn odstej1(a: u32) -> u32 {
        a-1
    }

    #[test]
    fn test_iteracija() {
        assert_eq!(iteracija(0, pristej1, manjsi_od7), 0);
        assert_eq!(iteracija(9, odstej1, manjsi_od7), 6);

    }

    fn identiteta(x:f64) -> f64 {
        x
    }

    fn skor_identiteta(x:f64) -> f64 {
        x-2.0
    }

    #[test]
    fn test_bisekcija() {
        assert_eq!(bisekcija(-1.0, 1.0, identiteta, 0.001), 0.0);
        assert_eq!(bisekcija(-10.0, 10.0, identiteta, 0.00001), 0.0);
        more_asserts::assert_le!(((bisekcija(-4.0, 4.0, skor_identiteta, 0.00001))-2.0).abs(), 0.000010001);

    }

    #[test]
    fn test_mat_mul() {
        assert_eq!(mat_mul([[1, 0], [0, 1]], [[3,4], [11,873154]]), [[3, 4], [11, 873154]]);
        assert_eq!(mat_mul([[123, 234], [2, 7]], [[1, 0], [0, 1]]), [[123, 234], [2, 7]]);
    }
    
    #[test]
    fn test_ordered() {
        assert_eq!(ordered(&[1,2,3]), true);
        assert_eq!(ordered(&[4,3,3]), true);
        assert_eq!(ordered(&[1,2,3,2]), false);
    }
    
    #[test]
    fn test_pow() {
        assert_eq!(pow(2, 13), 8192);
        assert_eq!(pow(3, 8), 6561);
        assert_eq!(pow(2, 27), 134217728);
    }

    #[test]
    fn test_pow_mod() {
        assert_eq!(pow_mod(2, 13, 10), 2);
        assert_eq!(pow_mod(3, 8, 9), 0);
        assert_eq!(pow_mod(2, 27, 8), 0);
        assert_eq!(pow_mod(2, 27, 1), 0);
    }
    
    #[test]
    fn test_sort() {
        assert_eq!(selection_sort(&mut[1]), [1]);
        assert_eq!(selection_sort(&mut[1, 2]), [1, 2]);
        assert_eq!(selection_sort(&mut[2, 1]), [1, 2]);
        assert_eq!(selection_sort(&mut[4, 5, 2, 9, 22, 1]), [1, 2, 4, 5, 9, 22]);
    }
}






// Kako napišeš test za igro??