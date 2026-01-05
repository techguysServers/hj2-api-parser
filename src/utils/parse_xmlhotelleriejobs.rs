use std::collections::HashMap;

use axum::body::Bytes;
use libxml::tree::Document;

use crate::utils::common::{Company, Job, Translation};

use crate::utils::common;

pub fn parse(file: &Bytes) -> Result<Vec<Job>, common::ParseError> {
    let document = common::validate_against_xsd(file, "xsd-schemas/xml-hotelleriejobs.xsd");
    if let Err(errors) = document {
        return Err(common::ParseError {
            message: "File is not valid".to_string(),
            xml_errors: errors,
        });
    }

    match parse_into_jobs(&document.unwrap()) {
        Ok(jobs) => {
            return Ok(jobs);
        }
        Err(e) => {
            return Err(common::ParseError {
                message: e,
                xml_errors: vec![],
            });
        }
    }
}

fn parse_into_jobs(document: &Document) -> Result<Vec<Job>, String> {
    let root = document.get_root_element().unwrap();

    // Get <job> nodes
    let children = root
        .findnodes("job")
        .map_err(|e| format!("Error finding job nodes: {:?}", e))?;
    let jobs = children
        .iter()
        .map(|job| {
            // Populate a dictionary, keyed by nodes names
            // This will be used to populate the Job struct with unique fields only as the repeated fields are overwritten
            let mut dictionary = HashMap::new();
            let mut child = job.get_first_child();
            while let Some(current_child) = child {
                dictionary.insert(
                    current_child.get_name().to_string(),
                    current_child.get_content().clone().to_string(),
                );
                child = current_child.get_next_sibling();
            }

            let titles = job.findnodes("title").unwrap();
            let descriptions = job.findnodes("description").unwrap();
            let requirements = job.findnodes("requirements").unwrap();

            // Build translations by language
            let mut translations: Vec<Translation> = Vec::new();

            for title_node in titles {
                let lang = title_node.get_attribute("lang").unwrap_or("en".to_string());
                let title_content = title_node.get_content().to_string();

                // Find existing translation with this language or create new one
                let translation = translations
                    .iter_mut()
                    .find(|t| t.language == lang.to_string());
                if let Some(existing_translation) = translation {
                    existing_translation.title = title_content;
                } else {
                    translations.push(Translation {
                        language: lang,
                        title: title_content,
                        description: String::new(),
                        requirements: String::new(),
                    });
                }
            }

            for desc_node in descriptions {
                let lang = desc_node.get_attribute("lang").unwrap_or("en".to_string());
                let desc_content = desc_node.get_content().to_string();

                // Find existing translation with this language or create new one
                let translation = translations.iter_mut().find(|t| t.language == lang);
                if let Some(existing_translation) = translation {
                    existing_translation.description = desc_content;
                } else {
                    translations.push(Translation {
                        language: lang,
                        title: String::new(),
                        description: desc_content,
                        requirements: String::new(),
                    });
                }
            }

            for req_node in requirements {
                let lang = req_node.get_attribute("lang").unwrap_or("en".to_string());
                let req_content = req_node.get_content().to_string();

                // Find existing translation with this language or create new one
                let translation = translations.iter_mut().find(|t| t.language == lang);
                if let Some(existing_translation) = translation {
                    existing_translation.requirements = req_content;
                } else {
                    translations.push(Translation {
                        language: lang,
                        title: String::new(),
                        description: String::new(),
                        requirements: req_content,
                    });
                }
            }

            Job {
                id: dictionary
                    .get("unique_id")
                    .unwrap_or(&String::new())
                    .to_string(),
                schedule: dictionary
                    .get("schedule")
                    .unwrap_or(&String::new())
                    .to_string(),
                category: dictionary
                    .get("category")
                    .unwrap_or(&String::new())
                    .to_string(),
                city: dictionary.get("city").unwrap_or(&String::new()).to_string(),
                province: dictionary
                    .get("province")
                    .unwrap_or(&String::new())
                    .to_string(),
                application_method: dictionary
                    .get("application_method")
                    .unwrap_or(&String::new())
                    .to_string(),
                application_destination: dictionary
                    .get("application_destination")
                    .unwrap_or(&String::new())
                    .to_string(),
                company: Company {
                    id: dictionary
                        .get("company_id")
                        .unwrap_or(&String::new())
                        .to_string(),
                    name: dictionary
                        .get("company")
                        .unwrap_or(&String::new())
                        .to_string(),
                    city: dictionary
                        .get("company_city")
                        .unwrap_or(&String::new())
                        .to_string(),
                    postal_code: dictionary
                        .get("company_postal_code")
                        .unwrap_or(&String::new())
                        .to_string(),
                    logo_url: dictionary
                        .get("company_logo_url")
                        .unwrap_or(&String::new())
                        .to_string(),
                },
                translations: translations,
            }
        })
        .collect();

    Ok(jobs)
}
