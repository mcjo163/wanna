use std::fmt::Display;
use std::fs;
use std::path::Path;

use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};

#[derive(Eq, PartialEq)]
pub enum Similarity {
    Exact,
    Similar,
    Different,
}

impl Similarity {
    pub fn between(s1: &str, s2: &str) -> Self {
        match edit_distance::edit_distance(&s1.to_lowercase(), &s2.to_lowercase()) {
            0 => Self::Exact,
            1..=2 => Self::Similar,
            _ => Self::Different,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Thing {
    pub verb: String,
    pub name: String,
}

impl Thing {
    pub fn new(verb: &str, name: &str) -> Self {
        let (verb, name) = (verb.into(), name.into());
        Self { verb, name }
    }
}

impl Display for Thing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.verb, self.name)
    }
}

pub struct Things(Vec<Thing>);

impl Things {
    pub fn from_file(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let data = fs::read_to_string(path)?;
        let things = serde_json::from_str(&data)?;

        Ok(Things(things))
    }

    pub fn save_to_file(&self, path: impl AsRef<Path>) -> anyhow::Result<()> {
        fs::write(
            path,
            serde_json::to_string_pretty(&self.0)
                .expect("failed to serialize")
                .as_bytes(),
        )?;
        Ok(())
    }

    pub fn find_existing_thing(&self, name: &str) -> Option<&Thing> {
        self.0.iter().find(|t| &t.name == name)
    }

    pub fn find_similar_candidate(&self, name: &str) -> Option<&Thing> {
        self.0
            .iter()
            .find(|t| Similarity::between(&t.name, name) == Similarity::Similar)
    }

    pub fn get_something(&self) -> Option<&Thing> {
        self.0.choose(&mut rand::thread_rng())
    }

    pub fn get_something_for_verb(&self, verb: &str) -> Option<&Thing> {
        self.0
            .iter()
            .filter(|t| t.verb == verb)
            .collect::<Vec<_>>()
            .choose(&mut rand::thread_rng())
            .map(|&t| t)
    }

    pub fn add_something(&mut self, thing: Thing) {
        self.0.push(thing);
    }
}
