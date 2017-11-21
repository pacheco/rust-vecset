use std::collections::HashSet;
use std::collections::BTreeSet;

extern crate fnv;

extern crate vecset;
use vecset::*;

#[macro_use]
extern crate timeit;
extern crate rand;

fn main() {
    for n in vec![8,16,64,128,512,1024].into_iter() {
        println!("n = {} ----------------------------------", n);
        print!("insertion order vecset with capacity\n\t");
        timeit!({
            let mut s = InsOrdVecSet::with_capacity(n);
            for _ in 0..n {
                let i: usize = rand::random();
                s.insert(i);
                s.contains(&i);
            }
        });

        print!("insertion order vecset\n\t");
        timeit!({
            let mut s = InsOrdVecSet::new();
            for _ in 0..n {
                let i: usize = rand::random();
                s.insert(i);
                s.contains(&i);
            }
        });

        print!("ordered vecset with capacity\n\t");
        timeit!({
            let mut s = OrdVecSet::with_capacity(n);
            for _ in 0..n {
                let i: usize = rand::random();
                s.insert(i);
                s.contains(&i);
            }
        });

        print!("ordered vecset\n\t");
        timeit!({
            let mut s = OrdVecSet::new();
            for _ in 0..n {
                let i: usize = rand::random();
                s.insert(i);
                s.contains(&i);
            }
        });

        print!("hashset\n\t");
        timeit!({
            let mut s = HashSet::with_capacity(n);
            for _ in 0..n {
                let i: usize = rand::random();
                s.insert(i);
                s.contains(&i);
            }
        });

        print!("fnv hashset\n\t");
        timeit!({
            let mut s = fnv::FnvHashSet::with_capacity_and_hasher(n, Default::default());
            for _ in 0..n {
                let i: usize = rand::random();
                s.insert(i);
                s.contains(&i);
            }
        });

        print!("btreeset\n\t");
        timeit!({
            let mut s = BTreeSet::new();
            for _ in 0..n {
                let i: usize = rand::random();
                s.insert(i);
                s.contains(&i);
            }
        });
    }
}
