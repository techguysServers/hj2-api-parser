use axum::body::Bytes;
use libxml::parser;
use libxml::schemas;
use libxml::tree::Document;
use serde::Serialize;

#[derive(Serialize)]
pub struct Company {
    pub id: String,
    pub name: String,
    pub city: String,
    pub postal_code: String,
    pub logo_url: String,
}

impl Default for Company {
    fn default() -> Self {
        Self {
            id: String::new(),
            name: String::new(),
            city: String::new(),
            postal_code: String::new(),
            logo_url: String::new(),
        }
    }
}

#[derive(Serialize)]
pub struct Translation {
    pub language: String,
    pub title: String,
    pub description: String,
    pub requirements: String,
}

impl Default for Translation {
    fn default() -> Self {
        Self {
            language: String::new(),
            title: String::new(),
            description: String::new(),
            requirements: String::new(),
        }
    }
}

#[derive(Serialize)]
pub struct Job {
    pub id: String,
    pub schedule: String,
    pub category: String,
    pub city: String,
    pub province: String,
    pub application_method: String,
    pub application_destination: String,
    pub company: Company,
    pub translations: Vec<Translation>,
}

#[derive(Serialize, Debug)]
pub struct XMLError {
    pub line: i32,
    pub column: i32,
    pub message: String,
    pub level: String,
    pub domain: String,
    pub code: i32,
}

#[derive(Debug)]
pub struct ParseError {
    pub message: String,
    pub xml_errors: Vec<XMLError>,
}

pub fn validate_against_xsd(file: &Bytes, xsd_path: &str) -> Result<Document, Vec<XMLError>> {
    let mut schema_parser = schemas::SchemaParserContext::from_file(xsd_path);
    let mut schema_validation =
        schemas::SchemaValidationContext::from_parser(&mut schema_parser).unwrap();

    let parser = parser::Parser::default();
    let document = parser.parse_string(file).unwrap();

    match schema_validation.validate_document(&document) {
        Ok(_) => Ok(document),
        Err(e) => {
            println!("Error: {:?}", e);

            let errors = e
                .iter()
                .map(|e| XMLError {
                    line: e.line.unwrap_or(0),
                    column: e.col.unwrap_or(0),
                    message: e.message.clone().unwrap_or_default(),
                    level: format!("{:?}", e.level),
                    domain: e.domain.to_string(),
                    code: e.code,
                })
                .collect();

            Err(errors)
        }
    }
}
