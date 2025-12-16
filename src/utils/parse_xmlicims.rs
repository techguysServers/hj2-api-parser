use axum::body::Bytes;
use libxml::tree::Document;

use crate::utils::common::{self, Job};

pub fn parse(file: &Bytes) -> Result<Vec<Job>, common::ParseError> {
    let document = common::validate_against_xsd(file, "xsd-schemas/xml-icims.xsd");

    if let Err(errors) = document {
        return Err(common::ParseError {
            message: "File is not valid".to_string(),
            xml_errors: errors,
        });
    }

    match parse_into_jobs(&document.unwrap()) {
        Ok(jobs) => Ok(jobs),
        Err(e) => Err(common::ParseError {
            message: e,
            xml_errors: vec![],
        }),
    }
}

/*
 * In this format language is included in id field as suffix after union trait
 * This means that for our format we need to list unique ids without language suffixes then merge jobs languages
 */
fn parse_into_jobs(document: &Document) -> Result<Vec<Job>, String> {
    let root = document.get_root_element().unwrap();
    let children = root.get_child_elements();
    let jobs = vec![];

    Ok(jobs)
}
