use std::{collections::HashMap, vec};

use axum::body::Bytes;
use libxml::tree::Document;

use crate::utils::common::{self, Company, Job, Translation};

pub fn parse(file: &Bytes) -> Result<Vec<Job>, common::ParseError> {
    let document = common::validate_against_xsd(file, "xsd-schemas/xml-zohorecruit.xsd");

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
                    current_child.get_attribute("name").unwrap().to_string(),
                    current_child.get_content().clone().to_string(),
                );
                child = current_child.get_next_sibling();
            }

            Job {
                id: dictionary.get("Reference Number").unwrap().to_string(),
                schedule: dictionary.get("Type d'emploi").unwrap().to_string(),
                category: dictionary.get("Secteur d'activité").unwrap().to_string(),
                city: dictionary.get("Ville").unwrap().to_string(),
                province: dictionary.get("État/Province").unwrap().to_string(),
                application_method: "url".to_string(),
                application_destination: dictionary
                    .get("url")
                    .unwrap_or(&"".to_string())
                    .to_string(),
                company: Company {
                    name: dictionary.get("Résidence").unwrap().to_string(),
                    ..Default::default()
                },
                translations: vec![Translation {
                    language: "fr".to_string(),
                    title: dictionary
                        .get("Titre de la publication")
                        .unwrap()
                        .to_string(),
                    description: dictionary.get("Description du poste").unwrap().to_string(),
                    ..Default::default()
                }],
            }
        })
        .collect();

    Ok(jobs)
}
