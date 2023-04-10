use std::collections::HashMap;
type Table = HashMap<String, Vec<String>>;

// &T pronounced "ref T"
fn show(table: &Table) {
    for (artist, works) in table {
        println!("works by {}:", artist);
        for work in works {
            println!("  {}", work);
        }
    }
}
// &mut T pronounced "ref mute T"
fn sort_works(table: &mut Table) {
    for (_artist, works) in table {
        works.sort();
    }
}


fn main() {
    let mut table = Table::new();
    table.insert("Gesualdo".to_string(),
        vec!["many madrigals".to_string(),
        "Tenebrae Responsoria".to_string()]);
    table.insert("Caravaggio".to_string(),
        vec!["The Musicians".to_string(),
        "The Calling of St. Matthew".to_string()]);
    table.insert("Cellini".to_string(),
        vec!["Perseus with the head of Medusa".to_string(),
        "a salt cellar".to_string()]);
    show(&table);
    assert_eq!(table["Gesualdo"][0], "many madrigals");
    println!("\nsort works:");
    sort_works(&mut table);
    show(&table);

    
}
