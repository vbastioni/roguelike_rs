#![allow(dead_code)]

#[derive(Debug)]
pub struct Combination(usize, usize);

impl Combination {
    pub fn new_2d(x: usize, y: usize) -> Self {
        (x, y).into()
    }

    pub fn new(sz: usize) -> Self {
        sz.into()
    }
}

impl From<usize> for Combination {
    fn from(u: usize) -> Self {
        Combination(u, u)
    }
}

impl From<(usize, usize)> for Combination {
    fn from(tup: (usize, usize)) -> Self {
        let (x, y) = tup;
        Combination(x, y)
    }
}

impl Combination {
    pub fn combine(&self) -> Vec<(usize, usize)> {
        let mut v = Vec::<(usize, usize)>::with_capacity(self.1);
        for i in 0..self.0 { for j in 0..self.1 {
            v.push((i, j));
        }}
        v
    }

    pub fn consume<A>(&self, f: impl Fn(&usize, &usize) -> A) -> Vec<A> {
        self.combine().iter().map(|(i, j)| f(i, j)).collect::<Vec<A>>()
    }

    /// Comb::map(u, f) is an alias of Comb::new(u).comsume(f)
    pub fn map<A>(u: usize, f: impl Fn(&usize, &usize) -> A) -> Vec<A> {
        Combination::new(u).consume(f)
    }

    pub fn for_each(&self, f: impl Fn(&(usize, usize))) {
        self.combine().iter().for_each(f)
    }
}

#[test]
fn test_little() {
    assert_eq!(Combination::new(2).combine(), vec![(0, 0), (0, 1), (1, 0), (1, 1), ]);
}
