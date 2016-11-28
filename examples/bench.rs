use std::collections::HashSet;
use std::collections::BTreeSet;

extern crate vecset;
use vecset::*;

#[macro_use]
extern crate timeit;
extern crate rand;

fn main() {
    for N in vec![8,16,64,128,512,1024].into_iter() {
        println!("N = {} ----------------------------------", N);
        print!("insertion order vecset with capacity\n\t");
        timeit!({
            let mut s = InsOrdVecSet::with_capacity(N);
            for _ in 0..N {
                let i: usize = rand::random();
                s.insert(i);
                s.contains(&i);
            }
        });

        print!("insertion order vecset\n\t");
        timeit!({
            let mut s = InsOrdVecSet::new();
            for _ in 0..N {
                let i: usize = rand::random();
                s.insert(i);
                s.contains(&i);
            }
        });

        print!("ordered vecset with capacity\n\t");
        timeit!({
            let mut s = OrdVecSet::with_capacity(N);
            for _ in 0..N {
                let i: usize = rand::random();
                s.insert(i);
                s.contains(&i);
            }
        });

        print!("ordered vecset\n\t");
        timeit!({
            let mut s = OrdVecSet::new();
            for _ in 0..N {
                let i: usize = rand::random();
                s.insert(i);
                s.contains(&i);
            }
        });

        print!("hashset\n\t");
        timeit!({
            let mut s = HashSet::new();
            for _ in 0..N {
                let i: usize = rand::random();
                s.insert(i);
                s.contains(&i);
            }
        });

        print!("btreeset\n\t");
        timeit!({
            let mut s = BTreeSet::new();
            for _ in 0..N {
                let i: usize = rand::random();
                s.insert(i);
                s.contains(&i);
            }
        });
    }
}
