extern crate derive_more;
extern crate reqwest;
extern crate serde_derive;

use {
    derive_more::Display,
    reqwest::{get, Error as ReqwestError},
    serde_derive::{Deserialize, Serialize},
    std::collections::HashMap,
};

const API_STEM: &'static str = "https://crates.io/api/v1";

const CRATES_ROUTE: &'static str = "crates";

#[derive(Debug, Serialize, Deserialize)]
#[serde(
    tag = "badge_type",
    content = "attributes",
    rename_all = "kebab-case"
)]
pub enum Badge {
    Appveyor {
        id: Option<String>,
        service: Option<String>,
        repository: String,
        project_name: Option<String>,
        branch: Option<String>,
    },
    TravisCi {
        branch: Option<String>,
        repository: String,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Category {
    pub id: String,
    pub category: String,
    pub slug: String,
    pub description: String,
    pub created_at: String,
    pub crates_cnt: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Crate {
    pub id: String,
    pub name: String,
    pub updated_at: String,
    pub versions: Vec<u64>,
    pub keywords: Vec<String>,
    pub categories: Vec<String>,
    pub badges: Vec<Badge>,
    pub created_at: String,
    pub downloads: u64,
    pub recent_downloads: u64,
    pub max_version: String,
    pub description: String,
    pub homepage: Option<String>,
    pub documentation: String,
    pub repository: Option<String>,
    pub links: Links,
    pub exact_match: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Keyword {
    pub id: String,
    pub keyword: String,
    pub created_at: String,
    pub crates_cnt: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Links {
    pub version_downloads: String,
    pub versions: Option<String>,
    pub owners: String,
    pub owner_team: String,
    pub owner_user: String,
    pub reverse_dependencies: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VersionLink {
    pub dependencies: String,
    pub version_downloads: String,
    pub authors: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FullCrateDetails {
    #[serde(rename = "crate")]
    pub crate_: Crate,
    pub versions: Vec<Version>,
    pub keywords: Vec<Keyword>,
    pub categories: Vec<Category>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Version {
    pub id: u64,
    #[serde(rename = "crate")]
    pub crate_: String,
    pub num: String,
    pub dl_path: String,
    pub readme_path: String,
    pub updated_at: String,
    pub created_at: String,
    pub downloads: u64,
    pub features: HashMap<String, Vec<String>>,
    pub yanked: bool,
    pub license: String,
    pub links: VersionLink,
    pub crate_size: Option<u64>,
}

#[derive(Debug, Display)]
pub enum GetCrateError {
    #[display(fmt = "error retrieving crate information: {}", _0)]
    Reqwest(ReqwestError),
    #[display(fmt = "error retrieving crate information: {}", _0)]
    Api(GetCrateErrorResponse),
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
enum GetCrateApiResponse {
    Found(FullCrateDetails),
    Error(GetCrateErrorResponse),
}

#[derive(Debug, Deserialize, Display, Serialize)]
#[display(fmt = "{:?}", errors)]
pub struct GetCrateErrorResponse {
    pub errors: Vec<ErrorDetail>,
}

#[derive(Debug, Deserialize, Display, Serialize)]
#[display(fmt = "{}", detail)]
pub struct ErrorDetail {
    pub detail: String,
}

pub fn get_crate(name: &str) -> Result<Option<FullCrateDetails>, GetCrateError> {
    use self::{GetCrateApiResponse::*, GetCrateError::*};

    match get(&format!("{}/{}/{}", API_STEM, CRATES_ROUTE, name))
        .map_err(Reqwest)?
        .json()
        .map_err(Reqwest)?
    {
        Found(r) => Ok(Some(r)),
        Error(GetCrateErrorResponse { errors }) => {
            if errors.len() == 1 && errors[0].detail == "Not Found" {
                Ok(None)
            } else {
                Err(Api(GetCrateErrorResponse { errors }))
            }
        }
    }
}

pub fn get_latest_version_of_crate(name: &str) -> Result<Option<(String, Version)>, GetCrateError> {
    match get_crate(name) {
        Ok(Some(FullCrateDetails {
            crate_: Crate { id, .. },
            versions,
            ..
        })) => {
            // NOTE: This assumes that versions are sorted in descending order.
            Ok(versions
                .into_iter()
                .filter(|v| !v.yanked)
                .next()
                .map(|v| (id, v)))
        }
        other => other.map(|o| o.map(|_| unreachable!())),
    }
}
