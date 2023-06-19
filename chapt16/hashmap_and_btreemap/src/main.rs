
fn main() {
    use std::collections::BTreeMap;

    // retain
    let mut map: BTreeMap<i32, i32> = (0..8).map(|x| (x, x*10)).collect();
    // Keep only the elements with even-numbered keys.
    map.retain(|&k, _| k % 2 == 0);
    assert!(map.into_iter().eq(vec![(0,0), (2, 20), (4, 40), (6, 60)]));
    // btree_map.split_off
    let mut a = BTreeMap::new();
    a.insert(1, "a");
    a.insert(2, "b");
    a.insert(3, "c");
    a.insert(17, "d");
    a.insert(41, "e");

    let b = a.split_off(&3);

    assert_eq!(a.len(), 2);
    assert_eq!(b.len(), 3);

    assert_eq!(a[&1], "a");
    assert_eq!(a[&2], "b");

    assert_eq!(b[&3], "c");
    assert_eq!(b[&17], "d");
    assert_eq!(b[&41], "e");

    // or_insert
    let mut map: BTreeMap<&str, usize> = BTreeMap::new();
    map.entry("poneyland").or_insert(12);

    assert_eq!(map["poneyland"], 12);

    // or_insert_with
    let mut map: BTreeMap<&str, String> = BTreeMap::new();
    let s = "hoho".to_string();

    map.entry("poneyland").or_insert_with(|| s);
    assert_eq!(map["poneyland"], "hoho".to_string());

    // or_default
    let mut map: BTreeMap<&str, Option<usize>> = BTreeMap::new();
    map.entry("poneyland").or_default();

    assert_eq!(map["poneyland"], None);

    // and_modify
    let mut map: BTreeMap<&str, usize> = BTreeMap::new();

    map.entry("poneyland")
        .and_modify(|e| { *e += 1 })
        .or_insert(42);
    assert_eq!(map["poneyland"], 42);

    map.entry("poneyland")
        .and_modify(|e| { *e += 1 })
        .or_insert(42);
    assert_eq!(map["poneyland"], 43);

}
