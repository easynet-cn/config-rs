#![cfg(feature = "yaml")]

extern crate config;
extern crate float_cmp;
extern crate serde;

use linked_hash_map::LinkedHashMap;
use std::path::PathBuf;

use self::config::*;
use self::float_cmp::ApproxEqUlps;

#[derive(Debug, Deserialize)]
struct Place {
    name: String,
    longitude: f64,
    latitude: f64,
    favorite: bool,
    telephone: Option<String>,
    reviews: u64,
    creator: LinkedHashMap<String, Value>,
    rating: Option<f32>,
}

#[derive(Debug, Deserialize)]
struct Settings {
    debug: f64,
    production: Option<String>,
    place: Place,
    #[serde(rename = "arr")]
    elements: Vec<String>,
}

fn make() -> Config {
    let mut c = Config::default();
    c.merge(File::new("tests/Settings", FileFormat::Yaml))
        .unwrap();

    c
}

#[test]
fn test_file() {
    let c = make();

    // Deserialize the entire file as single struct
    let s: Settings = c.try_into().unwrap();

    assert!(s.debug.approx_eq_ulps(&1.0, 2));
    assert_eq!(s.production, Some("false".to_string()));
    assert_eq!(s.place.name, "Torre di Pisa");
    assert!(s.place.longitude.approx_eq_ulps(&43.7224985, 2));
    assert!(s.place.latitude.approx_eq_ulps(&10.3970522, 2));
    assert_eq!(s.place.favorite, false);
    assert_eq!(s.place.reviews, 3866);
    assert_eq!(s.place.rating, Some(4.5));
    assert_eq!(s.place.telephone, None);
    assert_eq!(s.elements.len(), 10);
    assert_eq!(s.elements[3], "4".to_string());
    assert_eq!(
        s.place
            .creator
            .into_iter()
            .collect::<Vec<(String, config::Value)>>(),
        vec![
            ("name".to_string(), "John Smith".into()),
            ("username".into(), "jsmith".into()),
            ("email".into(), "jsmith@localhost".into()),
        ]
    );
}

#[test]
fn test_error_parse() {
    let mut c = Config::default();
    let res = c.merge(File::new("tests/Settings-invalid", FileFormat::Yaml));

    let path_with_extension: PathBuf = ["tests", "Settings-invalid.yaml"].iter().collect();

    assert!(res.is_err());
    assert_eq!(
        res.unwrap_err().to_string(),
        format!(
            "while parsing a block mapping, did not find expected key at \
         line 2 column 1 in {}",
            path_with_extension.display()
        )
    );
}
