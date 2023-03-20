use std::collections::HashMap;
use std::hash::Hash;

// Disjoint Set Definition
pub struct DisjointSet<T>
where
    T: Clone + Eq + Hash,
{
    sets: HashMap<T, Vec<T>>,
    representatives: HashMap<T, T>,
}

impl<T> DisjointSet<T>
where
    T: Clone + Eq + Hash, // Got error coz hashmap value needs these traits
{
    // Associate Function
    pub fn new() -> Self {
        Self {
            sets: HashMap::new(),
            representatives: HashMap::new(),
        }
    }

    // Insert a value to the set
    pub fn set_insert(&mut self, val: T) {
        self.sets.insert(val.clone(), vec![val.clone()]);
        self.representatives.insert(val.clone(), val.clone());
    }

    // Find parent of the value
    pub fn find(&self, val: &T) -> T {
        self.representatives.get(val).unwrap().clone()
    }

    // Union function for two nodes
    pub fn union(&mut self, a: &T, b: &T) {
        let repa = self.representatives.get(a).unwrap().clone();
        let repb = self.representatives.get(b).unwrap().clone();
        let setb = self.sets.remove(&repb).unwrap(); // get all from set of the second value

        for i in setb.iter() {
            self.representatives.remove(i); // remove them from their group
            self.representatives.insert(i.clone(), repa.clone()); // and add them to the first group
        }

        let seta = self.sets.get_mut(&repa).unwrap();

        // Now all elements from the second set will be added to first and thus union is complete
        for i in &setb {
            seta.push(i.clone());
        }
    }
}
