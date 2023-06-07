fn main() {
    use std::collections::BTreeMap;

    // A table mapping cities to their parks: each value is a vector.
    let mut parks = BTreeMap::new();
    parks.insert("Porland", vec!["Mt. Tabor Park", "Forest Park"]);
    parks.insert("Kyoto", vec!["Tadasu-no-Mori Forest", "Maruyama Koen"]);
    parks.insert("Nashville", vec!["Percy Warner Park", "Dragon Park"]);

    // Build a vector of all parks. `values` gives us an iterator producing
    // vectors, and then `flatten` produces each vector's elements in turn.
    let all_parks: Vec<_> = parks.values().flatten().cloned().collect();

    assert_eq!(all_parks,
        vec!["Tadasu-no-Mori Forest", "Maruyama Koen", "Percy Warner Park",
"Dragon Park", "Mt. Tabor Park", "Forest Park"]);

    assert_eq!(vec![None, Some("day"), None, Some("one")]
        .into_iter()
        .flatten()
        .collect::<Vec<_>>(),
        vec!["day", "one"]);

}
