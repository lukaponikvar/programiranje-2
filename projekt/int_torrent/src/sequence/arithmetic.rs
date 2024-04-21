use super::models::Sequence;
// Implementirajte artimetično zaporedje

pub struct Arithmetic<T> {
    zacetni_clen : T,
    razlika : T,
}

impl Arithmetic<i64> {
    pub fn new(zacetni_clen: i64, razlika: i64) -> Arithmetic<i64> {
        Arithmetic {
            zacetni_clen,
            razlika,
        }
    }

    pub fn name(&self) -> String {
        format!("Aritmeticno zaporedje z zacetnim clenom {} in razliko {}", self.zacetni_clen, self.razlika)
    }

    pub fn start(&self) -> i64 {
        self.zacetni_clen
    }

    pub fn k_th(&self, k: i64) ->Option<i64> {
        Some(self.zacetni_clen + k * self.razlika)
    }

    pub fn contains(&self, item: i64) -> bool {
        ((item - self.zacetni_clen) / self.razlika)*self.razlika + self.zacetni_clen == item
    }

}
