trait GeographicalEntity {
    fn search(&self, query: &str) -> bool;
}

struct Country {
    name: String,
    provinces: Vec<Box<dyn GeographicalEntity>>,
}

impl Country {
    fn new(name: &str) -> Country {
        Country {
            name: name.to_string(),
            provinces: Vec::new(),
        }
    }

    fn add_province(&mut self, province: Box<dyn GeographicalEntity>) {
        self.provinces.push(province);
    }
}

impl GeographicalEntity for Country {
    fn search(&self, query: &str) -> bool {
        let mut found = false;
        println!("Searching for {} in country {}", query, self.name);

        if self.name.to_lowercase() == query.to_lowercase() {
            println!("Match found in country: {}", self.name);
            found = true;
        }

        for province in &self.provinces {
            if province.search(query) {
                found = true;
            }
        }

        found
    }
}

struct Province {
    name: String,
    cities: Vec<Box<dyn GeographicalEntity>>,
}

impl Province {
    fn new(name: &str) -> Province {
        Province {
            name: name.to_string(),
            cities: Vec::new(),
        }
    }

    fn add_city(&mut self, city: Box<dyn GeographicalEntity>) {
        self.cities.push(city);
    }
}

impl GeographicalEntity for Province {
    fn search(&self, query: &str) -> bool {
        let mut found = false;
        println!("Searching for {} in province {}", query, self.name);

        if self.name.to_lowercase() == query.to_lowercase() {
            println!("Match found in province: {}", self.name);
            found = true;
        }

        for city in &self.cities {
            if city.search(query) {
                found = true;
            }
        }

        found
    }
}

struct City {
    name: String,
}

impl City {
    fn new(name: &str) -> City {
        City {
            name: name.to_string(),
        }
    }
}

impl GeographicalEntity for City {
    fn search(&self, query: &str) -> bool {
        println!("Searching for {} in city {}", query, self.name);

        if self.name.to_lowercase() == query.to_lowercase() {
            println!("Match found in city: {}", self.name);
            return true;
        }

        false
    }
}

fn main() {
    let mut country = Country::new("Country1");

    let mut province1 = Province::new("Province1");
    province1.add_city(Box::new(City::new("City1")));
    province1.add_city(Box::new(City::new("City2")));

    let mut province2 = Province::new("Province2");
    province2.add_city(Box::new(City::new("City3")));
    province2.add_city(Box::new(City::new("City4")));

    country.add_province(Box::new(province1));
    country.add_province(Box::new(province2));

    let query = "City3";
    if country.search(query) {
        println!("Search completed: Match found for {}", query);
    } else {
        println!("Search completed: No match found for {}", query);
    }
}

