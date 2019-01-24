# City Spellcheck 
[![Build
Status](https://travis-ci.org/PrismaPhonic/city-spellcheck.svg?branch=master)](https://travis-ci.org/PrismaPhonic/city-spellcheck)
[![crates.io](http://meritbadge.herokuapp.com/city_spellcheck)](https://crates.io/crates/city_spellcheck)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)
[![Released API docs](https://docs.rs/city-spellcheck/badge.svg)](https://docs.rs/city-spellcheck)

This crate provides a library for spell correction of city names
using a fuzzy search scoring system that has optional weighting for
distance.

What that means is that if you supply your current GPS coordinates, then the
spelling correction suggested results takes your current location heavily into
account when scoring each potential match.

Currently only supports USA and Canada, working on expanding to other countries ASAP.

## Setup

To use this library just add `city-spellcheck` to your `Cargo.toml` file:

```toml
[dependencies]
city-spellcheck = "0.1.0"
```

Now you can use it:

```rust
use city_spellcheck::*;
```

To take a look at a very simple RESTful API (with only one route) that uses this library,
check out the [City-Spellcheck Web Api](https://github.com/PrismaPhonic/city-spellcheck-web-api)

## Example Use Case

```rust
use city_spellcheck::*;

let mut cities = CityData::new();
cities
    .populate_from_file("data/cities_canada-usa-filtered.csv")
    .unwrap();
let london = Coordinate::new(42.98339, -81.23304);

let results = cities.search("London", Some(london));
assert_eq!(
    format!("{:?}", results),
    "[FuzzyResult { city: \"London, ON, CA\", latitude: 42.98339, longitude: -81.23304, score: 1.0 }, FuzzyResult { city: \"London, OH, US\", latitude: 39.88645, longitude: -83.44825, score: 0.6252391 }, FuzzyResult { city: \"London, KY, US\", latitude: 37.12898, longitude: -84.08326, score: 0.6250727 }, FuzzyResult { city: \"Lemont, IL, US\", latitude: 41.67364, longitude: -88.00173, score: 0.52094036 }, FuzzyResult { city: \"Brant, ON, CA\", latitude: 43.1334, longitude: -80.34967, score: 0.5208334 }]");
```

Please explore the [documentation](https://docs.rs/city_spellcheck) to learn more. Nearly all useful methods are on the CityData
struct.
