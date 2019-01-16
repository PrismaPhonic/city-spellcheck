#[macro_use]
extern crate criterion;

use criterion::Criterion;

use city_spellcheck::*;
use distance::*;

fn benchmark_populate_from_file(c: &mut Criterion) {
    let mut cities = CityData::new();
    c.bench_function("load csv", move |b| {
        b.iter(|| {
            cities
                .populate_from_file("data/cities_canada-usa-filtered.csv")
                .unwrap()
        })
    });
}

fn benchmark_phys_dist(c: &mut Criterion) {
    let sf = Coordinate::new(37.774929, -122.419416);
    let nyc = Coordinate::new(40.730610, -73.935242);
    c.bench_function("phys dist", move |b| {
        b.iter(|| CityData::find_distance_earth(sf, nyc))
    });
}

fn benchmark_levenshtein_dist(c: &mut Criterion) {
    c.bench_function("levenshtein dist", move |b| {
        b.iter(|| {
            damerau_levenshtein("London", "Applesauce");
        })
    });
}

fn benchmark_sift3_dist(c: &mut Criterion) {
    c.bench_function("sift3 dist", move |b| {
        b.iter(|| {
            sift3("Francisco", "San Francisco");
        })
    });
}

fn benchmark_sift4_dist(c: &mut Criterion) {
    c.bench_function("sift4 dist", move |b| {
        b.iter(|| {
            city_spellcheck::sift4("Francisco", "San Francisco");
        })
    });
}

fn benchmark_total_score(c: &mut Criterion) {
    let mut cities = CityData::new();
    cities
        .populate_from_file("data/cities_canada-usa-filtered.csv")
        .unwrap();
    c.bench_function("total score", move |b| {
        b.iter(|| {
            cities.total_score("Abbotsfor", 0, None);
        })
    });
}

fn benchmark_total_score_gps(c: &mut Criterion) {
    let mut cities = CityData::new();
    cities
        .populate_from_file("data/cities_canada-usa-filtered.csv")
        .unwrap();
    c.bench_function("total score with gps", move |b| {
        b.iter(|| {
            let london = Coordinate::new(42.98339, -81.23304);
            cities.total_score("Londo", 10, Some(london));
        })
    });
}

fn benchmark_search_with_gps(c: &mut Criterion) {
    let mut cities = CityData::new();
    cities
        .populate_from_file("data/cities_canada-usa-filtered.csv")
        .unwrap();
    c.bench_function("search", move |b| {
        b.iter(|| {
            let london = Coordinate::new(42.98339, -81.23304);
            let results = cities.search("London", Some(london));
        })
    });
}

criterion_group!(
    benches,
    benchmark_populate_from_file,
    benchmark_phys_dist,
    benchmark_levenshtein_dist,
    benchmark_sift3_dist,
    benchmark_sift4_dist,
    benchmark_search_with_gps,
    benchmark_total_score,
    benchmark_total_score_gps,
);
criterion_main!(benches);

