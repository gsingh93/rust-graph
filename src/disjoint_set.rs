use std::collections::HashMap;
use std::hash::Hash;
use std::fmt::{self, Debug, Formatter};

// TODO: Store references
pub struct DisjointSet<T> {
    sets: HashMap<T, Vec<T>>,
    repr: HashMap<T, T>
}

impl<T: Clone + Eq + Hash> DisjointSet<T> {
    pub fn new() -> DisjointSet<T> {
        DisjointSet { sets: HashMap::new(), repr: HashMap::new() }
    }

    pub fn add_set(&mut self, x: T) {
        self.repr.insert(x.clone(), x.clone());
        self.sets.insert(x.clone(), vec!(x));
    }

    pub fn find(&self, x: &T) -> T {
        self.repr[x].clone()
    }

    pub fn union(&mut self, x: &T, y: &T) {
        let repr_x = self.repr[x].clone();
        let repr_y = self.repr[y].clone();
        let y_set = self.sets.remove(&repr_y).unwrap();
        for i in y_set.iter() {
            self.repr.remove(i);
            self.repr.insert(i.clone(), repr_x.clone());
        }
        let x_set = self.sets.get_mut(&repr_x).unwrap();
        x_set.push_all(&*y_set);
    }
}

impl<T: Eq + Hash + Debug> Debug for DisjointSet<T> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        for (k, v) in self.sets.iter() {
            try!(write!(f, "{:?}: ", k));
            for i in v.iter() {
                try!(write!(f, "{:?}", i));
            }
            try!(write!(f, "\n"))
        }
        Ok(())
    }
}
