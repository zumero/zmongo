use rustc_serialize::json::{Json, Object};
use std::fs::File;

use super::responses::Responses;
use super::outcome::Outcome;

pub struct Phase {
    pub operation: Responses,
    pub outcome: Outcome,
}

impl Phase {
    fn from_json(object: &Object) -> Result<Phase, String> {
        let operation = val_or_err!(object.get("responses"),
                                    Some(&Json::Array(ref array)) => try!(Responses::from_json(array)),
                                    "No `responses` array found.");

        let outcome = val_or_err!(object.get("outcome"),
                                  Some(&Json::Object(ref obj)) => try!(Outcome::from_json(obj)),
                                  "No `outcome` object found.");

        Ok(Phase{ operation: operation, outcome: outcome })
    }
}

pub struct Suite {
    pub uri: String,
    pub phases: Vec<Phase>,
}

fn get_phases(object: &Object) -> Result<Vec<Phase>, String> {
    let array = val_or_err!(object.get("phases"),
                            Some(&Json::Array(ref array)) => array.clone(),
                            "No `phases` array found");

    let mut phases = vec![];

    for json in array {
        let obj = val_or_err!(json,
                              Json::Object(ref obj) => obj.clone(),
                              "`phases` array must only contain objects");

        let phase = match Phase::from_json(&obj) {
            Ok(phase) => phase,
            Err(s) => return Err(s)
        };

        phases.push(phase);
    }

    Ok(phases)
}

pub trait SuiteContainer: Sized {
    fn from_file(path: &str) -> Result<Self, String>;
    fn get_suite(&self) -> Result<Suite, String>;
}

impl SuiteContainer for Json {
    fn from_file(path: &str) -> Result<Json, String> {
        let mut file = File::open(path).ok().expect(&format!("Unable to open file: {}", path));

        Ok(Json::from_reader(&mut file).ok().expect(&format!("Invalid JSON file: {}", path)))
    }

    fn get_suite(&self) -> Result<Suite, String> {
        let object = val_or_err!(self,
                                 &Json::Object(ref object) => object.clone(),
                                 "`get_suite` requires a JSON object");

        let uri = val_or_err!(object.get("uri"),
                              Some(&Json::String(ref s)) => s.clone(),
                              "`get_suite` requires a connection uri");


        let phases = try!(get_phases(&object));
        Ok(Suite { uri: uri, phases: phases })
    }
}
