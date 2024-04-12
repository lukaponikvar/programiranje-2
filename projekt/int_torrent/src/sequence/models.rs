pub trait Sequence<T> {
    fn name(&self) -> String;
    fn start(&self) -> T;

    // To pustimo do naslednjič, ko se bomo natančneje poučili o Rustovih traitih in izposojanju
    // fn current_index(&self) -> usize;
    // fn current(&self) -> Option<T>;

    // fn next(&mut self) -> Option<T>;
    // fn k_next(&mut self, k: usize) -> Option<T>;

    fn k_th(&self, k: usize) -> Option<T>;

    fn contains(&self, item: T) -> bool;

    // fn contains(&self, item: T) -> bool {
    //     let mut k = 0;
    //     loop{
    //         if self.k_th(k) == item {
    //             return True
    //         }
    //     k += 1
    //     } 
    // };
}
