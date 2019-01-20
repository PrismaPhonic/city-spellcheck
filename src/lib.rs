/**
 * TODOS:
 * 1. Add enum for Region - states and provinces
*/
extern crate rayon;
extern crate redis;

#[macro_use]
extern crate serde_derive;

extern crate sift4;
use sift4::*;

use std::error::Error;
use std::fs;

use std::cmp::Ordering;
use std::fmt;

use rayon::iter::{IntoParallelIterator, ParallelIterator};

// use redis::Commands;

/// Data-Oriented Design approach
/// Struct of Arrays (SoA)
#[derive(Debug)]
pub struct CityData {
    pub names: Vec<String>,
    pub countries: Vec<Country>,
    pub regions: Vec<Region>,
    pub latitudes: Vec<f32>,
    pub longitudes: Vec<f32>,
}

#[derive(Debug, Copy, Clone)]
pub enum Country {
    US,
    CA,
}

#[derive(Debug, Copy, Clone)]
pub enum Region {
    Province,
    Territory,
    States(State),
}

#[derive(Debug, Copy, Clone)]
pub enum State {
    AL,
    AK,
    AZ,
    AR,
    CA,
    CO,
    CT,
    DE,
    FL,
    GA,
    HI,
    ID,
    IL,
    IN,
    IA,
    KS,
    KY,
    LA,
    ME,
    MD,
    MA,
    MI,
    MN,
    MS,
    MO,
    MT,
    NE,
    NV,
    NH,
    NJ,
    NM,
    NY,
    NC,
    ND,
    OH,
    OK,
    OR,
    PA,
    RI,
    SC,
    SD,
    TN,
    TX,
    UT,
    VT,
    VA,
    WA,
    WV,
    WI,
    WY,
}

impl fmt::Display for Country {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Country::US => write!(f, "US"),
            Country::CA => write!(f, "CA"),
        }
    }
}

impl fmt::Display for Region {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Region::States(State::AL) => write!(f, "AL"),
            Region::States(State::AK) => write!(f, "AK"),
            Region::States(State::AZ) => write!(f, "AZ"),
            Region::States(State::AR) => write!(f, "AR"),
            Region::States(State::CA) => write!(f, "CA"),
            Region::States(State::CO) => write!(f, "CO"),
            Region::States(State::CT) => write!(f, "CT"),
            Region::States(State::DE) => write!(f, "DE"),
            Region::States(State::FL) => write!(f, "FL"),
            Region::States(State::GA) => write!(f, "GA"),
            Region::States(State::HI) => write!(f, "HI"),
            Region::States(State::ID) => write!(f, "ID"),
            Region::States(State::IL) => write!(f, "IL"),
            Region::States(State::IN) => write!(f, "IN"),
            Region::States(State::IA) => write!(f, "IA"),
            Region::States(State::KS) => write!(f, "KS"),
            Region::States(State::KY) => write!(f, "KY"),
            Region::States(State::LA) => write!(f, "LA"),
            Region::States(State::ME) => write!(f, "ME"),
            Region::States(State::MD) => write!(f, "MD"),
            Region::States(State::MA) => write!(f, "MA"),
            Region::States(State::MI) => write!(f, "MI"),
            Region::States(State::MN) => write!(f, "MN"),
            Region::States(State::MS) => write!(f, "MS"),
            Region::States(State::MO) => write!(f, "MO"),
            Region::States(State::MT) => write!(f, "MT"),
            Region::States(State::NE) => write!(f, "NE"),
            Region::States(State::NV) => write!(f, "NV"),
            Region::States(State::NH) => write!(f, "NH"),
            Region::States(State::NJ) => write!(f, "NJ"),
            Region::States(State::NM) => write!(f, "NM"),
            Region::States(State::NY) => write!(f, "NY"),
            Region::States(State::NC) => write!(f, "NC"),
            Region::States(State::ND) => write!(f, "ND"),
            Region::States(State::OH) => write!(f, "OH"),
            Region::States(State::OK) => write!(f, "OK"),
            Region::States(State::OR) => write!(f, "OR"),
            Region::States(State::PA) => write!(f, "PA"),
            Region::States(State::RI) => write!(f, "RI"),
            Region::States(State::SC) => write!(f, "SC"),
            Region::States(State::SD) => write!(f, "SD"),
            Region::States(State::TN) => write!(f, "TN"),
            Region::States(State::TX) => write!(f, "TX"),
            Region::States(State::UT) => write!(f, "UT"),
            Region::States(State::VT) => write!(f, "VT"),
            Region::States(State::VA) => write!(f, "VA"),
            Region::States(State::WA) => write!(f, "WA"),
            Region::States(State::WV) => write!(f, "WV"),
            Region::States(State::WI) => write!(f, "WI"),
            Region::States(State::WY) => write!(f, "WY"),
            _ => write!(f, ""),
        }
    }
}

#[derive(Debug)]
pub struct City<'a> {
    name: &'a str,
    country: Country,
    region: Region,
    latitude: f32,
    longitude: f32,
}

#[derive(Debug, Copy, Clone)]
pub struct Coordinate {
    latitude: f32,
    longitude: f32,
}

impl Coordinate {
    pub fn new(latitude: f32, longitude: f32) -> Coordinate {
        Coordinate {
            latitude,
            longitude,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FuzzyResult {
    city: String,
    latitude: f32,
    longitude: f32,
    score: f32,
}

impl FuzzyResult {
    fn new(city_data: City, score: f32) -> FuzzyResult {
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
    pub fn new() -> Self {
        CityData {
            names: Vec::new(),
            countries: Vec::new(),
            regions: Vec::new(),
            latitudes: Vec::new(),
            longitudes: Vec::new(),
        }
    }

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
                    Country::US => CityData::state_match(region),
                    Country::CA => Region::Territory,
                };

                self.add_city(name, country, region, latitude, longitude);
            };
        }

        Ok(())
    }

    fn state_match(region: &str) -> Region {
        match region {
            "AL" => Region::States(State::AL),
            "AK" => Region::States(State::AK),
            "AZ" => Region::States(State::AZ),
            "AR" => Region::States(State::AR),
            "CA" => Region::States(State::CA),
            "CO" => Region::States(State::CO),
            "CT" => Region::States(State::CT),
            "DE" => Region::States(State::DE),
            "FL" => Region::States(State::FL),
            "GA" => Region::States(State::GA),
            "HI" => Region::States(State::HI),
            "ID" => Region::States(State::ID),
            "IL" => Region::States(State::IL),
            "IN" => Region::States(State::IN),
            "IA" => Region::States(State::IA),
            "KS" => Region::States(State::KS),
            "KY" => Region::States(State::KY),
            "LA" => Region::States(State::LA),
            "ME" => Region::States(State::ME),
            "MD" => Region::States(State::MD),
            "MA" => Region::States(State::MA),
            "MI" => Region::States(State::MI),
            "MN" => Region::States(State::MN),
            "MS" => Region::States(State::MS),
            "MO" => Region::States(State::MO),
            "MT" => Region::States(State::MT),
            "NE" => Region::States(State::NE),
            "NV" => Region::States(State::NV),
            "NH" => Region::States(State::NH),
            "NJ" => Region::States(State::NJ),
            "NM" => Region::States(State::NM),
            "NY" => Region::States(State::NY),
            "NC" => Region::States(State::NC),
            "ND" => Region::States(State::ND),
            "OH" => Region::States(State::OH),
            "OK" => Region::States(State::OK),
            "OR" => Region::States(State::OR),
            "PA" => Region::States(State::PA),
            "RI" => Region::States(State::RI),
            "SC" => Region::States(State::SC),
            "SD" => Region::States(State::SD),
            "TN" => Region::States(State::TN),
            "TX" => Region::States(State::TX),
            "UT" => Region::States(State::UT),
            "VT" => Region::States(State::VT),
            "VA" => Region::States(State::VA),
            "WA" => Region::States(State::WA),
            "WV" => Region::States(State::WV),
            "WI" => Region::States(State::WI),
            "WY" => Region::States(State::WY),
            _ => Region::Territory,
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
    /// string distance using Levenshtein algorithm
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
    // or 92 km away - setting a score of 92 as perfect 1.0
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
            Region::States(State::NY),
            40.7128,
            74.0060,
        );
        assert_eq!(format!("{:?}", cities.get_city(0)), "City { name: \"New York City\", country: US, region: States(NY), latitude: 40.7128, longitude: 74.006 }");
    }

    #[test]
    fn test_citydata_struct_sf() {
        let mut cities = CityData::new();
        cities.add_city(
            "San Francisco",
            Country::US,
            Region::States(State::CA),
            37.7749,
            122.4194,
        );
        assert_eq!(format!("{:?}", cities.get_city(0)), "City { name: \"San Francisco\", country: US, region: States(CA), latitude: 37.7749, longitude: 122.4194 }");
    }

    #[test]
    fn test_populate_from_file() {
        let mut cities = CityData::new();
        cities
            .populate_from_file("data/cities_canada-usa-filtered.csv")
            .unwrap();
        assert_eq!(
            format!("{:?}", cities.get_city(0)),
            "City { name: \"Abbotsford\", country: CA, region: Territory, latitude: 49.05798, longitude: -122.25257 }"
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
            "[FuzzyResult { city: \"London, , CA\", latitude: 42.98339, longitude: -81.23304, score: 1.0 }, FuzzyResult { city: \"London, OH, US\", latitude: 39.88645, longitude: -83.44825, score: 0.6252391 }, FuzzyResult { city: \"London, KY, US\", latitude: 37.12898, longitude: -84.08326, score: 0.6250727 }, FuzzyResult { city: \"Lemont, IL, US\", latitude: 41.67364, longitude: -88.00173, score: 0.52094036 }, FuzzyResult { city: \"Brant, , CA\", latitude: 43.1334, longitude: -80.34967, score: 0.5208334 }]"
        );
    }
}
