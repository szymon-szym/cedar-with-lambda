use std::fmt;

use serde::{Deserialize, Serialize};

// input types

#[derive(Clone, Debug)]
pub struct UserId(pub String);

impl UserId {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug)]
pub struct OwnerId(pub String);

impl OwnerId {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}


#[derive(Serialize, Deserialize, Clone)]
pub enum ActionVerb {
    GetReportInfo,
    SetReportInfo,
    GenerateS3Url
}


impl fmt::Display for ActionVerb {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let str = match self {
            ActionVerb::GetReportInfo => "GetReportInfo",
            ActionVerb::SetReportInfo => "SetReportInfo",
            ActionVerb::GenerateS3Url => "GenerateS3Url"
        };
        write!(f, "{}", str)
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub enum ResourceType {
    S3Object,
    ReportData
}

impl fmt::Display for ResourceType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let str = match self {
            ResourceType::S3Object => "S3Object",
            ResourceType::ReportData => "ReportData"
        };

        write!(f, "{}", str)
    }
}

// output

#[derive(Debug)]
pub enum AuthorizationDecision {
    Allow,
    Deny
}

// internal types

#[derive(Clone)]
pub struct UserEntityInput {
    pub user_id: UserId
}

impl UserEntityInput {
    pub fn new(user_id: UserId) -> Self {
        Self { user_id }
    }
}

#[derive(Clone)]
pub struct ActionEntityInput {
    pub verb: ActionVerb
}

impl ActionEntityInput {
    pub fn new(verb: ActionVerb) -> Self {
        Self { verb }
    }
}

#[derive(Clone)]
pub struct ResourceEntityInput {
    pub resource_type: ResourceType,
    pub owner_id: OwnerId
}

impl ResourceEntityInput {
    pub fn new(resource_type: ResourceType, owner_id: OwnerId) -> Self {
        Self { resource_type, owner_id }
    }
}

#[derive(Clone)]
pub enum CreateEntityInput {
    User(UserEntityInput),
    Action(ActionEntityInput),
    Resource(ResourceEntityInput)
}

