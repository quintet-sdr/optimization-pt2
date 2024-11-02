use std::fmt::{self, Display, Formatter};
use std::fs::File;
use std::io::BufReader;

use color_eyre::Result;
use serde::Deserialize;

use pt2_core::Constraints;

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Test {
    pub name: Box<str>,
    pub objective_function: Vec<f64>,
    pub constraints: Constraints,
    pub initial_point: Vec<f64>,
    #[serde(alias = "epsilon", default = "eps_default")]
    pub eps: usize,
}

impl Display for Test {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

pub fn read_tests() -> Result<Vec<Test>> {
    let tests_file = BufReader::new(File::open("tests.json")?);
    Ok(serde_json::from_reader(tests_file)?)
}

fn name_default() -> Box<str> {
    Box::from("Unnamed")
}

const fn eps_default() -> usize {
    2
}
