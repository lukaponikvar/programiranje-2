fn main() {
    let x = "abcd";
    let mut matija = Student {
        vpisna : 27224498,
        ime : String::from("Matija"),
        priimek : String::from("Pretnar"),
    };
    let mut se_en_student = Student {
        vpisna : 27225498,
        ime : String::from("Matej"),
        priimek : String::from("Pretnar"),
    };
    matija.vpisna = 272244999;
    println!("{:?}", matija.leto_vpisa());


    fn absolutna(z: Kompleksno) -> f32 {
        (z.0.powi(2) + z.1 * z.1).sqrt()
    }

    let i = Kompleksno(0., 1.);
    println!("{}", i.absolutna_vrednost());

    
}
//matija.ime



#[derive(Debug, PartialEq, PartialOrd)]
struct Student { // Student gre na sklad, je lastnik teh nizov in se ga ne da skopirat
    vpisna : u32,
    ime : String, // referenca na nek podniz v pomnilniku (dva kazalca na začetek in konec veljavnega niza)
    priimek : String,
}

impl Student {
    fn leto_vpisa(&self) -> String {
        let leto = (self.vpisna - 27000000) / 10000;
        format!("20{}/{}", leto, leto +1 )
//        println!("{((272244999 - 27000000) / 10000))}")
    }
}

struct Kompleksno (f32, f32);

impl Kompleksno {
    fn absolutna_vrednost(self) -> f32 {
        (self.0.powi(2) + self.1.powi(2)).sqrt()
    }
}

enum StatusStudenta {
    Prvic(u8),
    Ponovno(u8),
    Obcan,
    Izredni(String),

}

impl StatusStudenta {
    fn letnik(&self) -> Option<u8> {
        match self {
            StatusStudenta::Prvic(l) => Some(*l),
            StatusStudenta::Ponovno(l) => Some(*l),
            _ => None

        }
    }
    fn je_obcan(&self) -> bool {
        match self {
        StatusStudenta::Obcan => true,
        _ => false,
        }
    }
}








    // self ko dajaš v drug tip oseba -> student

// Lastnik vseh svojih polj


// utf 8:
// predstavlja različne simbole z različno stevilo bitov
// če je znak v prvi polovici ga predstaviš z 8 biti, sicer z 16-imi
// kar je blo ascii še vedno dela, vedno vemo kje smo
// ne vemo kok prostora(bytov) bo zasedl 


//ENUMI
// vsote