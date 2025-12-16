use axum::{Json, body::Bytes, extract::Multipart, http::StatusCode};
use log::{info, warn};
use serde::Serialize;

use crate::utils::{
    common::{Job, XMLError},
    parse_xmlgrandio, parse_xmlhotelleriejobs, parse_xmlpscout, parse_xmltidan,
    parse_xmlzohoquintescense, parse_xmlzohorecruit,
};

#[derive(Serialize)]
pub struct ImportResponse {
    success: bool,
    errors: String,
    xml_errors: Vec<XMLError>,
    jobs: Vec<Job>,
}

pub async fn handler(multipart: Multipart) -> (StatusCode, Json<ImportResponse>) {
    let (format, file) = read_multipart(multipart).await;

    if format.is_none() || file.is_none() {
        warn!(target: "import", "Request to import, format or file is missing");
        return (
            StatusCode::BAD_REQUEST,
            Json(ImportResponse {
                success: false,
                errors: "Format or file is missing".to_string(),
                xml_errors: vec![],
                jobs: vec![],
            }),
        );
    }

    info!(target: "import", "Request to parse an {:?} file", format.clone().unwrap());

    match format.unwrap().as_str() {
        "xml-hotelleriejobs" => {
            let jobs = parse_xmlhotelleriejobs::parse(file.as_ref().unwrap());
            if let Err(errors) = jobs {
                warn!(target: "import", "Error parsing file: {:?} (xml-hotelleriejobs)", errors.message);
                return (
                    StatusCode::BAD_REQUEST,
                    Json(ImportResponse {
                        success: false,
                        errors: errors.message,
                        xml_errors: errors.xml_errors,
                        jobs: vec![],
                    }),
                );
            }

            info!(target: "import", "File parsed successfully");

            return (
                StatusCode::OK,
                Json(ImportResponse {
                    success: true,
                    errors: "".to_string(),
                    xml_errors: vec![],
                    jobs: jobs.unwrap(),
                }),
            );
        }
        "xml-icims" => {
            warn!(target: "import", "icims format is not implemented yet");
            return (
                StatusCode::NOT_IMPLEMENTED,
                Json(ImportResponse {
                    success: false,
                    errors: "icims format is no longer supported".to_string(),
                    xml_errors: vec![],
                    jobs: vec![],
                }),
            );
        }
        "xml-grandio" => {
            let jobs = parse_xmlgrandio::parse(file.as_ref().unwrap());
            if let Err(errors) = jobs {
                warn!(target: "import", "Error parsing fril: {:?} (xml-grandio)", errors.message);
                return (
                    StatusCode::BAD_REQUEST,
                    Json(ImportResponse {
                        success: false,
                        errors: errors.message,
                        xml_errors: errors.xml_errors,
                        jobs: vec![],
                    }),
                );
            }

            info!(target: "import", "File parsed successfully (xml-grandio)");
            return (
                StatusCode::OK,
                Json(ImportResponse {
                    success: true,
                    errors: "".to_string(),
                    xml_errors: vec![],
                    jobs: jobs.unwrap(),
                }),
            );
        }
        "xml-tidan" => {
            let jobs = parse_xmltidan::parse(file.as_ref().unwrap());
            if let Err(errors) = jobs {
                warn!(target: "import", "Error parsing file: {:?} (xml-tidan)", errors.message);
                return (
                    StatusCode::BAD_REQUEST,
                    Json(ImportResponse {
                        success: false,
                        errors: errors.message,
                        xml_errors: errors.xml_errors,
                        jobs: vec![],
                    }),
                );
            }

            info!(target: "import", "File parsed successfully (xml-tidan)");
            return (
                StatusCode::OK,
                Json(ImportResponse {
                    success: true,
                    errors: "".to_string(),
                    xml_errors: vec![],
                    jobs: jobs.unwrap(),
                }),
            );
        }
        "xml-pscout" => {
            let jobs = parse_xmlpscout::parse(file.as_ref().unwrap());
            if let Err(errors) = jobs {
                warn!(target: "import", "Error parsing file: {:?} (xml-pscout)", errors.message);
                return (
                    StatusCode::BAD_REQUEST,
                    Json(ImportResponse {
                        success: false,
                        errors: errors.message,
                        xml_errors: errors.xml_errors,
                        jobs: vec![],
                    }),
                );
            }

            info!(target: "import", "File parsed successfully (xml-pscout)");
            return (
                StatusCode::OK,
                Json(ImportResponse {
                    success: true,
                    errors: "".to_string(),
                    xml_errors: vec![],
                    jobs: jobs.unwrap(),
                }),
            );
        }
        "xml-zohoquintescence" => {
            let jobs = parse_xmlzohoquintescense::parse(file.as_ref().unwrap());
            if let Err(errors) = jobs {
                warn!(target: "import", "Error parsing file: {:?} (xml-zohoquintescence)", errors.message);
                return (
                    StatusCode::BAD_REQUEST,
                    Json(ImportResponse {
                        success: false,
                        errors: errors.message,
                        xml_errors: errors.xml_errors,
                        jobs: vec![],
                    }),
                );
            }

            info!(target: "import", "File parsed successfully (xml-zohoquintescence)");
            return (
                StatusCode::OK,
                Json(ImportResponse {
                    success: true,
                    errors: "".to_string(),
                    xml_errors: vec![],
                    jobs: jobs.unwrap(),
                }),
            );
        }
        "xml-zohorecruit" => {
            let jobs = parse_xmlzohorecruit::parse(file.as_ref().unwrap());
            if let Err(errors) = jobs {
                warn!(target: "import", "Error parsing file: {:?} (xml-zohorecruit)", errors.message);
                return (
                    StatusCode::BAD_REQUEST,
                    Json(ImportResponse {
                        success: false,
                        errors: errors.message,
                        xml_errors: errors.xml_errors,
                        jobs: vec![],
                    }),
                );
            }

            info!(target: "import", "File parsed successfully (xml-zohorecruit)");
            return (
                StatusCode::OK,
                Json(ImportResponse {
                    success: true,
                    errors: "".to_string(),
                    xml_errors: vec![],
                    jobs: jobs.unwrap(),
                }),
            );
        }
        _ => {
            warn!(target: "import", "Format is not supported");
            return (
                StatusCode::BAD_REQUEST,
                Json(ImportResponse {
                    success: false,
                    errors: "Format is not supported".to_string(),
                    xml_errors: vec![],
                    jobs: vec![],
                }),
            );
        }
    }
}

async fn read_multipart(mut multipart: Multipart) -> (Option<String>, Option<Bytes>) {
    let mut format: Option<String> = None;
    let mut file: Option<Bytes> = None;

    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();

        if name == "format" {
            format = Some(field.text().await.unwrap());
        } else if name == "file" {
            file = Some(field.bytes().await.unwrap());
        }
    }

    (format, file)
}
