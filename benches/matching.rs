#![feature(test)]

extern crate test;
extern crate fragment;

use test::Bencher;
use fragment::matching;

#[bench]
fn bench_find(b: &mut Bencher) {
    let haystack = vec![
        "src/fragment.rs".to_string(),
        "lib/fragments.rs".to_string(),
    ];
    b.iter(|| matching::find("frag", &haystack, 2));
}
