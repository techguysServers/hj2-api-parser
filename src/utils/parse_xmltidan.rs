use std::collections::HashMap;

use axum::body::Bytes;
use libxml::tree::Document;

use crate::utils::common::{self, Company, Job, Translation};

pub fn parse(file: &Bytes) -> Result<Vec<Job>, common::ParseError> {
    let document = common::validate_against_xsd(file, "xsd-schemas/xml-tidan.xsd");

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

fn parse_into_jobs(document: &Document) -> Result<Vec<Job>, String> {
    let root = document.get_root_element().unwrap();
    let children = root.get_child_elements();
    let jobs = children
        .iter()
        .map(|job| {
            let mut dictionary = HashMap::new();
            let mut child = job.get_first_child();
            while let Some(current_child) = child {
                dictionary.insert(
                    current_child.get_name().to_string(),
                    current_child.get_content().clone().to_string(),
                );
                child = current_child.get_next_sibling();
            }

            Job {
                id: dictionary.get("businessProcessId").unwrap().to_string(),
                schedule: dictionary.get("typedemploi").unwrap().to_string(),
                category: dictionary.get("categorie1").unwrap().to_string()
                    + ","
                    + dictionary.get("categorie2").unwrap()
                    + ","
                    + dictionary.get("categorie3").unwrap(),
                city: dictionary.get("ville").unwrap().to_string(),
                province: dictionary.get("province").unwrap().to_string(),
                application_method: "url".to_string(),
                application_destination: dictionary.get("applyUrl").unwrap().to_string(),
                company: Company {
                    name: dictionary.get("company").unwrap().to_string(),
                    ..Default::default()
                },
                translations: vec![Translation {
                    language: dictionary.get("jobCode").unwrap().to_string(),
                    title: dictionary.get("title").unwrap().to_string(),
                    description: dictionary.get("description").unwrap().to_string(),
                    requirements: (dictionary.get("education").unwrap().to_string()
                        + "\n"
                        + dictionary.get("experience").unwrap())
                    .to_string(),
                }],
            }
        })
        .collect();

    Ok(jobs)
}
