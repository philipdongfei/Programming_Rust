fn main() {
    let args: Vec<String> = std::env::args().collect();

    use std::collections::{HashSet, BTreeSet, LinkedList, HashMap, BTreeMap};

    let args: HashSet<String> = std::env::args().collect();
    let args: BTreeSet<String> = std::env::args().collect();
    let args: LinkedList<String> = std::env::args().collect();

    // Collecting a map requires (key, value) pairs, so for this example,
    // zip the sequence of strings with a sequence of integers.
    let args: HashMap<String, usize> = std::env::args().zip(0..).collect();
    let args: BTreeMap<String, usize> = std::env::args().zip(0..).collect();

    // The return type of collect is its type parameter, so the first two calls are equivalent to
    // the following
    let args = std::env::args().collect::<Vec<String>>();
    let args = std::env::args().collect::<HashSet<String>>(); 
}
