fn main() {
    println!("Hello, world!");
    let mut zaporedje = ArtimeticnoZaporedje::new(1,1);
    println!("{:?}", (zaporedje.next(),zaporedje.next()));
    println!("{:?}", zaporedje);
    println!("{:?}", zaporedje.n_th(7));
    println!("{}", zaporedje.sum(10)+1);
    println!("{:?}", zaporedje.reset());
    println!("{}", zaporedje.current());
    println!("{:?}",zaporedje.vsota(&zaporedje));
    let b = Izraz::Operacija(Box::new(Izraz::Konstanta(1)), BinOperacija::Minus, Box::new(Izraz::Operacija(Box::new(Izraz::Konstanta(2)), BinOperacija::Pow, Box::new(Izraz::Konstanta(3)))));
    println!("{}", b.eval());
    println!("{}", b.collect());
    println!("{}", b.izpis());
}

#[derive(Debug)]
struct ArtimeticnoZaporedje {
    zacetni_clen : u32,
    trenutni_clen : u32,
    razlika : u32,
}

impl ArtimeticnoZaporedje {
    fn new(zacetni_clen: u32, razlika: u32) -> ArtimeticnoZaporedje {
        ArtimeticnoZaporedje {
            zacetni_clen,
            trenutni_clen : 0,
            razlika,
        }
    }

    fn next(&mut self) -> u32 {
        self.trenutni_clen += 1;
        self.zacetni_clen + self.trenutni_clen * self.razlika - self.razlika
    }

    fn n_th(&self, n: u32) -> u32 {
        self.zacetni_clen + n * self.razlika
    }

    fn reset(&mut self) {
        self.trenutni_clen = 0
    }

    fn current(&self) -> u32 {
        self.zacetni_clen + self.trenutni_clen * self.razlika
    }

    fn sum(&mut self, n: u32) -> u32 {
        // n * (self.zacetni_clen) + n * (n+1) * self.razlika / 2
        let mut a = ArtimeticnoZaporedje {
            razlika : self.razlika,
            trenutni_clen : 1,
            zacetni_clen : self.zacetni_clen,
        };
        let mut vsota = 0;
        for indeks in 1..n {
            vsota += a.next()
        };
        vsota
    }

    fn vsota(&self, other : &ArtimeticnoZaporedje) -> ArtimeticnoZaporedje {
        ArtimeticnoZaporedje {
            razlika : self.razlika + other.razlika,
            zacetni_clen : self.zacetni_clen + other.zacetni_clen,
            trenutni_clen : self.trenutni_clen + other.trenutni_clen
        }
    }

    // fn produkt(&self, other : &ArtimeticnoZaporedje) -> ArtimeticnoZaporedje {
    //     ArtimeticnoZaporedje {
    //         zacetni_clen : self.zacetni_clen * other.zacetni_clen
    //         razlika : (self.zacetni_clen + trenutni_clen * self.razlika) * (other.zacetni_clen + other.trenutni_clen * 
    //         trenutni_clen : 0
    //     }
    // }
}

enum UnOperacija {
    UnMinus,
}

enum BinOperacija {
    Plus,
    Minus,
    Times,
    Pow,
}

enum Izraz {
    Konstanta(u32),
    Operacija(Box<Izraz>, BinOperacija, Box<Izraz>),
    UnOperacija(Box<Izraz>, UnOperacija),
}

impl Izraz {
    fn eval(&self) -> i32 {
        match self {
            Izraz::Konstanta(c) => *c as i32,
            Izraz::Operacija(levo, operacija, desno) => match operacija {
                BinOperacija::Plus => levo.eval() + desno.eval(),
                BinOperacija::Minus => levo.eval() - desno.eval(),
                BinOperacija::Times => levo.eval() * desno.eval(),
                BinOperacija::Pow => levo.eval().pow(desno.eval() as u32),
            }
            Izraz::UnOperacija(izraz, operacija) => - izraz.eval()
        }
    }

    fn collect(&self) -> u32 {
        match self {
            Izraz::Konstanta(c) => 1,
            Izraz::Operacija(levo, _, desno) => levo.collect() + desno.collect(),
            Izraz::UnOperacija(izraz, _) => izraz.collect(),
        }
    }

    fn izpis(&self) -> String {
        match self {
            Izraz::Konstanta(c) => format!("{}", *c),
            Izraz::Operacija(levo, operacija, desno) => match operacija {
                BinOperacija::Plus => format!("({} + {})", levo.izpis(), desno.izpis()),
                BinOperacija::Minus => format!("({} - {})", levo.izpis(), desno.izpis()),
                BinOperacija::Times => format!("({} * {})", levo.izpis(), desno.izpis()),
                BinOperacija::Pow => format!("({} ** {})", levo.izpis(), desno.izpis()),
            }
            Izraz::UnOperacija(izraz, operacija) => format!("(-{})", izraz.izpis()),
        }
    }
}