use crate::sequence::models::Sequence;


pub struct Constant<T> {
    vrednost: T,
}

impl Constant<i64> {
    pub fn new(vrednost: i64) -> Constant<i64> {
        Constant{
            vrednost,
        }
    }

    pub fn name(&self) -> String {
        let niz = format!("Konstantno zaporedje {}", self.vrednost);
        niz
    }

    pub fn start(&self) -> i64 {
        self.vrednost
    }

    pub fn k_th(&self, k: u32) -> Option<i64> {
        Some(self.vrednost)
    }

    pub fn contains(&self, item: i64) -> bool {
        self.vrednost == item
    }
}