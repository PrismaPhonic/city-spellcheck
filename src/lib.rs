#[macro_use]
extern crate serde_derive;
extern crate rayon;
extern crate redis;
extern crate sift4;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use sift4::*;
use std::cmp::Ordering;
use std::error::Error;
use std::fs;

/// countries holds a country enum with valid countries
pub mod countries;

/// regions holds enums and implementations to display variants in english friendly string format
pub mod regions;

use self::countries::*;
use self::regions::*;

/// CityData holds all city data, each city is stored vertically across fields at a given index
#[derive(Debug)]
pub struct CityData {
    names: Vec<String>,
    countries: Vec<Country>,
    regions: Vec<Region>,
    latitudes: Vec<f32>,
    longitudes: Vec<f32>,
}

/// City is an abstraction above CityData holding information on a single city (stored at a
/// specific index in CityData)
#[derive(Debug)]
pub struct City<'a> {
    name: &'a str,
    country: Country,
    region: Region,
    latitude: f32,
    longitude: f32,
}

/// Stores a GPS coordinate
#[derive(Debug, Copy, Clone)]
pub struct Coordinate {
    latitude: f32,
    longitude: f32,
}

impl Coordinate {
    /// Instantiate a Coordinate instance with supplied latitude and longitude
    pub fn new(latitude: f32, longitude: f32) -> Coordinate {
        Coordinate {
            latitude,
            longitude,
        }
    }
}

/// FuzzyResult represents a result from our location weighted fuzzy search. Includes a score where
/// 0 is worst and 1 is perfect match.
#[derive(Debug, Serialize, Deserialize)]
pub struct FuzzyResult {
    city: String,
    latitude: f32,
    longitude: f32,
    score: f32,
}

impl FuzzyResult {
    /// Takes in a City instance and score, and instantiates a new FuzzyResult
    /// Which includes human readable formatting of city name, region and country
    pub fn new(city_data: City, score: f32) -> FuzzyResult {
        let City {
            name,
            country,
            region,
            latitude,
            longitude,
        } = city_data;
        let city = format!("{}, {}, {}", name, region, country);
        FuzzyResult {
            city,
            latitude,
            longitude,
            score,
        }
    }
}

impl CityData {
    /// Instantiates a new instance of CityData that is empty. Usually populated with
    /// `populate_from_file` method
    pub fn new() -> Self {
        CityData {
            names: Vec::new(),
            countries: Vec::new(),
            regions: Vec::new(),
            latitudes: Vec::new(),
            longitudes: Vec::new(),
        }
    }

    /// Takes in a geonames file and matches countries and regions to our custom enum variants
    /// It then adds each city in line by line to our CityData instance.
    ///
    /// Note that geonames file comes from: [http://download.geonames.org/export/dump](http://download.geonames.org/export/dump)
    /// Nicer formatted version is available in the github project for this crate under
    /// the data folder:
    ///
    /// [city-spellcheck github](https://github.com/PrismaPhonic/city-spellcheck)
    pub fn populate_from_file(&mut self, filename: &str) -> Result<(), Box<dyn Error>> {
        let buffer = fs::read_to_string(filename)?;
        let mut lines = buffer.lines();

        // skip header line
        lines.next();

        for line in lines {
            if let [name, country, region, latitude, longitude] =
                line.split(',').collect::<Vec<&str>>()[..]
            {
                let latitude: f32 = latitude.parse()?;
                let longitude: f32 = longitude.parse()?;

                let country = match country {
                    "US" => Country::US,
                    "CA" => Country::CA,
                    _ => continue,
                };

                let region = match country {
                    Country::US => CityData::us_match(region),
                    Country::CA => CityData::ca_match(region),
                };

                self.add_city(name, country, region, latitude, longitude);
            };
        }

        Ok(())
    }

    // Matches admin1 codes:
    // http://download.geonames.org/export/dump/admin1CodesASCII.txt
    fn ca_match(region: &str) -> Region {
        match region {
            "01" => Region::Province(CAProvince::AB),
            "02" => Region::Province(CAProvince::BC),
            "03" => Region::Province(CAProvince::MB),
            "04" => Region::Province(CAProvince::NB),
            "05" => Region::Province(CAProvince::NL),
            "07" => Region::Province(CAProvince::NS),
            "08" => Region::Province(CAProvince::ON),
            "09" => Region::Province(CAProvince::PE),
            "10" => Region::Province(CAProvince::QC),
            "11" => Region::Province(CAProvince::SK),
            "12" => Region::Territory(CATerritory::YT),
            "13" => Region::Territory(CATerritory::NT),
            "14" => Region::Territory(CATerritory::NU),
            _ => Region::None,
        }
    }

    fn us_match(region: &str) -> Region {
        match region {
            "AL" => Region::State(USState::AL),
            "AK" => Region::State(USState::AK),
            "AZ" => Region::State(USState::AZ),
            "AR" => Region::State(USState::AR),
            "CA" => Region::State(USState::CA),
            "CO" => Region::State(USState::CO),
            "CT" => Region::State(USState::CT),
            "DE" => Region::State(USState::DE),
            "FL" => Region::State(USState::FL),
            "GA" => Region::State(USState::GA),
            "HI" => Region::State(USState::HI),
            "ID" => Region::State(USState::ID),
            "IL" => Region::State(USState::IL),
            "IN" => Region::State(USState::IN),
            "IA" => Region::State(USState::IA),
            "KS" => Region::State(USState::KS),
            "KY" => Region::State(USState::KY),
            "LA" => Region::State(USState::LA),
            "ME" => Region::State(USState::ME),
            "MD" => Region::State(USState::MD),
            "MA" => Region::State(USState::MA),
            "MI" => Region::State(USState::MI),
            "MN" => Region::State(USState::MN),
            "MS" => Region::State(USState::MS),
            "MO" => Region::State(USState::MO),
            "MT" => Region::State(USState::MT),
            "NE" => Region::State(USState::NE),
            "NV" => Region::State(USState::NV),
            "NH" => Region::State(USState::NH),
            "NJ" => Region::State(USState::NJ),
            "NM" => Region::State(USState::NM),
            "NY" => Region::State(USState::NY),
            "NC" => Region::State(USState::NC),
            "ND" => Region::State(USState::ND),
            "OH" => Region::State(USState::OH),
            "OK" => Region::State(USState::OK),
            "OR" => Region::State(USState::OR),
            "PA" => Region::State(USState::PA),
            "RI" => Region::State(USState::RI),
            "SC" => Region::State(USState::SC),
            "SD" => Region::State(USState::SD),
            "TN" => Region::State(USState::TN),
            "TX" => Region::State(USState::TX),
            "UT" => Region::State(USState::UT),
            "VT" => Region::State(USState::VT),
            "VA" => Region::State(USState::VA),
            "WA" => Region::State(USState::WA),
            "WV" => Region::State(USState::WV),
            "WI" => Region::State(USState::WI),
            "WY" => Region::State(USState::WY),
            _ => Region::None,
        }
    }

    fn add_city(
        &mut self,
        name: &str,
        country: Country,
        region: Region,
        latitude: f32,
        longitude: f32,
    ) {
        self.names.push(name.to_string());
        self.countries.push(country);
        self.regions.push(region);
        self.latitudes.push(latitude);
        self.longitudes.push(longitude);
    }

    /// `get_city` is a function to get a city back from our `CityData` struct.
    /// Helps us keep our data stored in a DoD fashion and provide
    /// a higher level abstraction to retrieve each city based on index
    ///
    /// Supply an index and get back the city at that index.
    ///
    /// ## Example
    ///
    /// ```rust
    ///
    /// let mut cities = city_spellcheck::CityData::new();
    ///
    /// 	cities
    ///     	.populate_from_file("data/cities_canada-usa-filtered.csv")
    ///     	.unwrap();
    ///        
    ///		assert_eq!(
    ///            format!("{:?}", cities.get_city(0)),
    ///            "City { name: \"Abbotsford\", country: CA, region: Province(BC), latitude: 49.05798, longitude: -122.25257 }"
    ///     );
    /// ```
    pub fn get_city(&self, idx: usize) -> City {
        City {
            name: &self.names[idx],
            country: self.countries[idx],
            region: self.regions[idx],
            latitude: self.latitudes[idx],
            longitude: self.longitudes[idx],
        }
    }

    /// `total_score` takes into account location as well as
    /// string distance using sift4 algorithm
    pub fn total_score(&self, term: &str, idx: usize, loc: Option<Coordinate>) -> f32 {
        let city = &self.names[idx];
        let latitude = self.latitudes[idx];
        let longitude = self.longitudes[idx];
        let city_loc = Coordinate {
            latitude,
            longitude,
        };

        let str_dist = sift4(city, term) as f32;
        let mut str_score = if str_dist >= term.len() as f32 {
            0.0
        } else {
            (term.len() as f32 - str_dist) / term.len() as f32
        };

        if str_score == 0.0 {
            return 0.0;
        };

        // penalty if first letters don't match
        if city.chars().next().unwrap() != term.chars().next().unwrap() {
            if str_score < 0.1 {
                str_score = 0.0;
            } else {
                str_score -= 0.1;
            }
        }

        let mut dist_score = str_score;

        if let Some(loc2) = loc {
            let phys_dist = CityData::find_distance_earth(city_loc, loc2);
            dist_score = CityData::dist_score(phys_dist);
        };

        (str_score * 5.0 + dist_score * 3.0) / 8.0
    }

    /// Finds circular distance from two gps coordinates using haversine formula
    pub fn find_distance_earth(loc1: Coordinate, loc2: Coordinate) -> f32 {
        const R: f32 = 6372.8;
        let Coordinate {
            latitude: mut lat1,
            longitude: mut long1,
        } = loc1;
        let Coordinate {
            latitude: mut lat2,
            longitude: long2,
        } = loc2;
        long1 -= long2;
        long1 = long1.to_radians();
        lat1 = lat1.to_radians();
        lat2 = lat2.to_radians();
        let dz: f32 = lat1.sin() - lat2.sin();
        let dx: f32 = long1.cos() * lat1.cos() - lat2.cos();
        let dy: f32 = long1.sin() * lat1.cos();
        ((dx * dx + dy * dy + dz * dz).sqrt() / 2.0).asin() * 2.0 * R
    }

    // Largest city in North America by area is NYC which is 8600 square km
    // or 92 km away - setting a dist of 92 as perfect 1.0
    fn dist_score(dist: f32) -> f32 {
        if dist < 92.0 {
            1.0
        } else {
            92.0 / (dist.powf(2.0) - (91.9 as f32).powf(2.0))
        }
    }

    pub fn search(&self, term: &str, loc: Option<Coordinate>) -> Vec<FuzzyResult> {
        let mut found: Vec<(usize, f32)> = (0..self.names.len())
            .into_par_iter()
            .map(|i| (i, self.total_score(term, i, loc)))
            .filter(|(_, score)| score > &0.5)
            .collect();

        found.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(Ordering::Equal));

        found
            .iter()
            .map(|result| FuzzyResult::new(self.get_city(result.0), result.1))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_citydata_struct_nyc() {
        let mut cities = CityData::new();
        cities.add_city(
            "New York City",
            Country::US,
            Region::State(USState::NY),
            40.7128,
            74.0060,
        );
        assert_eq!(format!("{:?}", cities.get_city(0)), "City { name: \"New York City\", country: US, region: State(NY), latitude: 40.7128, longitude: 74.006 }");
    }

    #[test]
    fn test_citydata_struct_sf() {
        let mut cities = CityData::new();
        cities.add_city(
            "San Francisco",
            Country::US,
            Region::State(USState::CA),
            37.7749,
            122.4194,
        );
        assert_eq!(format!("{:?}", cities.get_city(0)), "City { name: \"San Francisco\", country: US, region: State(CA), latitude: 37.7749, longitude: 122.4194 }");
    }

    #[test]
    fn test_populate_from_file() {
        let mut cities = CityData::new();
        cities
            .populate_from_file("data/cities_canada-usa-filtered.csv")
            .unwrap();
        assert_eq!(
            format!("{:?}", cities.get_city(0)),
            "City { name: \"Abbotsford\", country: CA, region: Province(BC), latitude: 49.05798, longitude: -122.25257 }"
        );
    }

    #[test]
    fn test_str_dist() {
        assert_eq!(sift4("Londo", "London"), 1);
    }

    #[test]
    fn test_phys_dist() {
        let sf = Coordinate {
            latitude: 37.774929,
            longitude: -122.419416,
        };
        let nyc = Coordinate {
            latitude: 40.730610,
            longitude: -73.935242,
        };
        assert_eq!(CityData::find_distance_earth(sf, nyc), 4135.694);
    }

    #[test]
    fn test_dist_score() {
        assert_eq!(CityData::dist_score(4135.694), 0.0000053815274);
    }

    #[test]
    fn test_total_score_no_gps() {
        let mut cities = CityData::new();
        cities
            .populate_from_file("data/cities_canada-usa-filtered.csv")
            .unwrap();
        assert_eq!(cities.total_score("Abbotsfor", 0, None), 0.88888896);
    }

    #[test]
    fn test_search_with_gps() {
        let mut cities = CityData::new();
        cities
            .populate_from_file("data/cities_canada-usa-filtered.csv")
            .unwrap();
        let london = Coordinate {
            latitude: 42.98339,
            longitude: -81.23304,
        };
        let results = cities.search("London", Some(london));
        assert_eq!(
            format!("{:?}", results),
            "[FuzzyResult { city: \"London, ON, CA\", latitude: 42.98339, longitude: -81.23304, score: 1.0 }, FuzzyResult { city: \"London, OH, US\", latitude: 39.88645, longitude: -83.44825, score: 0.6252391 }, FuzzyResult { city: \"London, KY, US\", latitude: 37.12898, longitude: -84.08326, score: 0.6250727 }, FuzzyResult { city: \"Lemont, IL, US\", latitude: 41.67364, longitude: -88.00173, score: 0.52094036 }, FuzzyResult { city: \"Brant, ON, CA\", latitude: 43.1334, longitude: -80.34967, score: 0.5208334 }]"
        );
    }
}
