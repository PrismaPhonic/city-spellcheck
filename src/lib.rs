#[macro_use]
extern crate serde_derive;
extern crate rayon;
extern crate redis;
extern crate sift4;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use sift4::*;
use std::cmp::Ordering;
use std::error::Error;
use std::fmt;
use std::fs;

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
    Province(CAProvince),
    Territory(CATerritory),
    State(USState),
    None,
}

#[derive(Debug, Copy, Clone)]
pub enum CAProvince {
    ON,
    QC,
    NS,
    NB,
    MB,
    BC,
    PE,
    SK,
    AB,
    NL,
}

#[derive(Debug, Copy, Clone)]
pub enum CATerritory {
    NT,
    NU,
    YT,
}

#[derive(Debug, Copy, Clone)]
pub enum USState {
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
            Region::State(USState::AL) => write!(f, "AL"),
            Region::State(USState::AK) => write!(f, "AK"),
            Region::State(USState::AZ) => write!(f, "AZ"),
            Region::State(USState::AR) => write!(f, "AR"),
            Region::State(USState::CA) => write!(f, "CA"),
            Region::State(USState::CO) => write!(f, "CO"),
            Region::State(USState::CT) => write!(f, "CT"),
            Region::State(USState::DE) => write!(f, "DE"),
            Region::State(USState::FL) => write!(f, "FL"),
            Region::State(USState::GA) => write!(f, "GA"),
            Region::State(USState::HI) => write!(f, "HI"),
            Region::State(USState::ID) => write!(f, "ID"),
            Region::State(USState::IL) => write!(f, "IL"),
            Region::State(USState::IN) => write!(f, "IN"),
            Region::State(USState::IA) => write!(f, "IA"),
            Region::State(USState::KS) => write!(f, "KS"),
            Region::State(USState::KY) => write!(f, "KY"),
            Region::State(USState::LA) => write!(f, "LA"),
            Region::State(USState::ME) => write!(f, "ME"),
            Region::State(USState::MD) => write!(f, "MD"),
            Region::State(USState::MA) => write!(f, "MA"),
            Region::State(USState::MI) => write!(f, "MI"),
            Region::State(USState::MN) => write!(f, "MN"),
            Region::State(USState::MS) => write!(f, "MS"),
            Region::State(USState::MO) => write!(f, "MO"),
            Region::State(USState::MT) => write!(f, "MT"),
            Region::State(USState::NE) => write!(f, "NE"),
            Region::State(USState::NV) => write!(f, "NV"),
            Region::State(USState::NH) => write!(f, "NH"),
            Region::State(USState::NJ) => write!(f, "NJ"),
            Region::State(USState::NM) => write!(f, "NM"),
            Region::State(USState::NY) => write!(f, "NY"),
            Region::State(USState::NC) => write!(f, "NC"),
            Region::State(USState::ND) => write!(f, "ND"),
            Region::State(USState::OH) => write!(f, "OH"),
            Region::State(USState::OK) => write!(f, "OK"),
            Region::State(USState::OR) => write!(f, "OR"),
            Region::State(USState::PA) => write!(f, "PA"),
            Region::State(USState::RI) => write!(f, "RI"),
            Region::State(USState::SC) => write!(f, "SC"),
            Region::State(USState::SD) => write!(f, "SD"),
            Region::State(USState::TN) => write!(f, "TN"),
            Region::State(USState::TX) => write!(f, "TX"),
            Region::State(USState::UT) => write!(f, "UT"),
            Region::State(USState::VT) => write!(f, "VT"),
            Region::State(USState::VA) => write!(f, "VA"),
            Region::State(USState::WA) => write!(f, "WA"),
            Region::State(USState::WV) => write!(f, "WV"),
            Region::State(USState::WI) => write!(f, "WI"),
            Region::State(USState::WY) => write!(f, "WY"),
            Region::Province(CAProvince::AB) => write!(f, "AB"),
            Region::Province(CAProvince::BC) => write!(f, "BC"),
            Region::Province(CAProvince::MB) => write!(f, "MB"),
            Region::Province(CAProvince::NB) => write!(f, "NB"),
            Region::Province(CAProvince::NL) => write!(f, "NL"),
            Region::Province(CAProvince::NS) => write!(f, "NS"),
            Region::Province(CAProvince::ON) => write!(f, "ON"),
            Region::Province(CAProvince::PE) => write!(f, "PE"),
            Region::Province(CAProvince::QC) => write!(f, "QC"),
            Region::Province(CAProvince::SK) => write!(f, "SK"),
            Region::Territory(CATerritory::NT) => write!(f, "NT"),
            Region::Territory(CATerritory::NU) => write!(f, "NU"),
            Region::Territory(CATerritory::YT) => write!(f, "YT"),
            Region::None => write!(f, ""),
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
            // we never hit the catch all - better way to write this?
            _ => Region::State(USState::CA),
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
            Region::State(USState::NY),
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
            Region::State(USState::CA),
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
