// This module implements configuration related stuff.
// Copyright (c) 2014 by Shipeng Feng.
// Licensed under the BSD License, see LICENSE for more details.

use std::os;
use std::io::File;
use std::collections::TreeMap;
use serialize::json::{Object, Json};


/// We provide ways to fill it from JSON files:
///
/// ```ignore
/// app.config.from_jsonfile("yourconfig.json")
/// ```
///
/// You can also load configurations from an environment variable
/// pointing to a file:
///
/// ```ignore
/// app.config.from_envvar("YOURAPPLICATION_SETTINGS")
/// ```
///
/// In this case, you have to set this environment variable to the file
/// you want to use.  On Linux and OS X you can use the export statement:
///
/// ```bash
/// export YOURAPPLICATION_SETTINGS="/path/to/config/file"
/// ```
///
#[deriving(Clone)]
pub struct Config {
    config: Object,
}

impl Config {
    /// Create a `Config` object.
    pub fn new() -> Config {
        let json_object: Object = TreeMap::new();
        Config {
            config: json_object,
        }
    }

    /// Set a value for the key.
    pub fn set(&mut self, key: &str, value: Json) {
        self.config.insert(key.to_string(), value);
    }

    /// Returns a reference to the value corresponding to the key.
    pub fn get(&self, key: &str) -> Option<&Json> {
        self.config.get(&key.to_string())
    }

    /// Loads a configuration from an environment variable pointing to
    /// a JSON configuration file.
    pub fn from_envvar(&mut self, variable_name: &str) {
        match os::getenv(variable_name) {
            Some(value) => self.from_jsonfile(value.as_slice()),
            None => panic!("The environment variable {} is not set.", variable_name),
        }
    }

    /// Updates the values in the config from a JSON file.
    pub fn from_jsonfile(&mut self, filepath: &str) {
        let path = Path::new(filepath);
        let mut file = File::open(&path).unwrap();
        let content = file.read_to_string().unwrap();
        let object: Json = from_str(content.as_slice()).unwrap();
        match object {
            Json::Object(object) => { self.from_object(object); },
            _ => { panic!("The configuration file is not an JSON object."); }
        }
    }

    /// Updates the values from the given `Object`.
    pub fn from_object(&mut self, object: Object) {
        for (key, value) in object.iter() {
            self.set(key.as_slice(), value.clone());
        }
    }
}