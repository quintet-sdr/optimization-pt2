use std::{fs::File, io::BufReader};

use color_eyre::Result;
use serde::Deserialize;

use pt2_core::Constraints;

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Test {
    pub objective_function: Vec<f64>,
    pub constraints: Constraints,
    pub initial_point: Vec<f64>,
    #[serde(alias = "epsilon", default = "eps_default")]
    pub eps: usize,
}

pub fn read_tests() -> Result<Box<[Test]>> {
    let tests_file = BufReader::new(File::open("tests.json")?);
    Ok(serde_json::from_reader(tests_file)?)
}

const fn eps_default() -> usize {
    2
}
