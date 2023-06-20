use std::hash::{Hash, Hasher};
use std::collections::{HashSet, HashMap};
use fnv::{FnvHashMap, FnvHashSet};

/// The ID number for an object in the British Museum's collection.
#[derive(Clone, PartialEq, Eq, Hash)]
enum MuseumNumber {
    One,Two,Three,Four,Five,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum TimeUnit {
    Seconds, Minutes, Hours, Days, Months, Years,
}

enum RoughTime {
    InThePast(TimeUnit, u32),
    JustNow,
    InTheFuture(TimeUnit, u32),
}

enum Culture {
   Low,Popular,Folk,Global,
}

struct Artifact {
    id: MuseumNumber, 
    name: String,
    cultures: Vec<Culture>,
    date: RoughTime,
}

impl PartialEq for Artifact {
    fn eq(&self, other: &Artifact) -> bool {
        self.id == other.id
    }
}

impl Eq for Artifact {

}

impl Hash for Artifact {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        // Delegate hashing to the MuseumNumber.
        self.id.hash(hasher);
    }
}


fn main() {
    let mut collection = HashSet::<Artifact>::new();
    collection.insert(Artifact {
        id: MuseumNumber::One,
        name: "one".to_string(),
        cultures: Vec::<Culture>::new(),
        date: RoughTime::InThePast(TimeUnit::Years, 100),
    });
    collection.insert(Artifact {
        id: MuseumNumber::One,
        name: "two".to_string(),
        cultures: Vec::<Culture>::new(),
        date: RoughTime::InThePast(TimeUnit::Years, 1000),
    });
    collection.insert(Artifact {
        id: MuseumNumber::Two,
        name: "two".to_string(),
        cultures: Vec::<Culture>::new(),
        date: RoughTime::InThePast(TimeUnit::Years, 2000),
    });
    for artifact in &collection {
        println!("{:?}", artifact.name);
    }

    // Using a Custom Hashing Algorithm
    let mut fnvmap = FnvHashMap::default();
    fnvmap.insert(1, "one");
    fnvmap.insert(2, "two");
    let mut fnvset = FnvHashSet::default();
    fnvset.insert(1);
    fnvset.insert(2);

}
