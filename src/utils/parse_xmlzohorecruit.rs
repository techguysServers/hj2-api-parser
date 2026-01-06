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
    let children = root
        .findnodes("job")
        .map_err(|e| format!("Error finding job nodes: {:?}", e))?;
    let jobs = children
        .iter()
        .map(|job| {
            let mut dictionary = HashMap::new();
            let mut child = job.get_first_child();
            while let Some(current_child) = child {
                let name_attribute = current_child.get_attribute("name");
                if let Some(name) = name_attribute {
                    dictionary.insert(name.to_string(), current_child.get_content().clone());
                }

                child = current_child.get_next_sibling();
            }

            Job {
                id: dictionary
                    .get("Reference Number")
                    .unwrap_or(&String::new())
                    .to_string(),
                schedule: dictionary
                    .get("Type d'emploi")
                    .unwrap_or(dictionary.get("Type d’emploi").unwrap_or(&String::new()))
                    .to_string(),
                category: dictionary
                    .get("Secteur d'activité")
                    .unwrap_or(
                        dictionary
                            .get("Secteur d’activité")
                            .unwrap_or(&String::new()),
                    )
                    .to_string(),
                city: dictionary
                    .get("Ville")
                    .unwrap_or(&String::new())
                    .to_string(),
                province: dictionary
                    .get("État/Province")
                    .unwrap_or(&String::new())
                    .to_string(),
                application_method: "url".to_string(),
                application_destination: dictionary
                    .get("url")
                    .unwrap_or(&"".to_string())
                    .to_string(),
                company: Company {
                    name: dictionary
                        .get("Résidence")
                        .unwrap_or(&String::new())
                        .to_string(),
                    ..Default::default()
                },
                translations: vec![Translation {
                    language: "fr".to_string(),
                    title: dictionary
                        .get("Titre de la publication")
                        .unwrap_or(&String::new())
                        .to_string(),
                    description: dictionary
                        .get("Description du poste")
                        .unwrap_or(&String::new())
                        .to_string(),
                    ..Default::default()
                }],
            }
        })
        .collect();

    Ok(jobs)
}
