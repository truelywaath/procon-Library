use proconio::{input, fastout};
use proconio::marker::{Bytes, Chars, Usize1};
use itertools::Itertools;
use regex::Regex;

struct UnionFind {
    n: usize,
    par: Vec<usize>,
}

impl UnionFind {
    fn init(n: usize) -> UnionFind {
        let mut par = Vec::new();
        for i in 0..n {
            par.push(i);
        }
        UnionFind {
            n: n,
            par: par,
        }
    }

    fn root(&mut self, x: usize) -> usize {
        if self.par[x] == x { return x; }
        else {
            self.par[x] = self.root(self.par[x]);
            return self.par[x];
        }
    }

    fn same(&mut self, x: usize, y: usize) -> bool {
        self.root(x) == self.root(y)
    }

    fn unite(&mut self, x: usize, y: usize) {
        let x = self.root(x);
        let y = self.root(y);
        if x == y { return; }
        self.par[x] = y;
    } 
}
