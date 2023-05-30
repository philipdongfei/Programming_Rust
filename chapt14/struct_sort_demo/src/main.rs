#[derive(Debug)]
struct City {
    name: String,
    population: i64,
    country: String,
}

/// Helper function for sorting cities by population.
fn city_population_descending(city: &City) -> i64 {
    -city.population
}

fn sort_cities(cities: &mut Vec<City>) {
    //cities.sort_by_key(city_population_descending); // ok
    cities.sort_by_key(|city| -city.population);
}


fn main() {
    let mut cities: Vec<City> = Vec::new();
    cities.push(City {name:"New York City".to_string(), population:18_937_000, country:"USA".to_string() });
    cities.push(City {name:"San Diego".to_string(), population:3_319_000, country:"USA".to_string()});
    cities.push(City {name:"Miami".to_string(), population:6_265_000, country:"USA".to_string() });
    cities.push(City {name:"Phoenix".to_string(), population:4_717_000, country:"USA".to_string()});
    cities.push(City {name:"Houston".to_string(), population:6_707_000, country:"USA".to_string()});
    sort_cities(&mut cities);
    println!("cities: {:?}", cities);

}
