use std::{
    collections::{HashMap, HashSet},
    error::Error,
    str::FromStr,
};

use cedar_policy::{Authorizer, Context, Entities, Entity, EntityId, EntityTypeName, EntityUid, PolicySet, Request, RestrictedExpression};

use super::models::{ActionEntityInput, ActionVerb, AuthorizationDecision, CreateEntityInput, OwnerId, ResourceEntityInput, ResourceType, UserEntityInput, UserId};

fn create_entity(input: CreateEntityInput) -> Result<Entity, Box<dyn Error + Send + Sync>> {
    
    let (entity_id, entity_type) = match input.clone() {
        CreateEntityInput::User(input) => (
            EntityId::from_str(input.user_id.as_str())?,
            EntityTypeName::from_str("User")?,
        ),
        CreateEntityInput::Action(input) => (
            EntityId::from_str(input.verb.to_string().as_str())?,
            EntityTypeName::from_str("Action")?,
        ),
        CreateEntityInput::Resource(input) => (
            EntityId::from_str(input.resource_type.to_string().as_str())?,
            EntityTypeName::from_str("Resource")?,
        ),
    };

    let entity_uid = EntityUid::from_type_name_and_id(entity_type, entity_id);

    let attributes = match input {
        CreateEntityInput::User(input) => {
            let mut attributes = HashMap::new();
            attributes.insert(
                "id".to_string(),
                RestrictedExpression::from_str(format!("\"{}\"", input.user_id.as_str()).as_str())
                    .expect("restr exp parsing error"),
            );
            attributes
        }
        CreateEntityInput::Action(_) => HashMap::new(),
        CreateEntityInput::Resource(input) => {
            let mut attributes = HashMap::new();
            attributes.insert(
                "owner_id".to_string(),
                RestrictedExpression::from_str(format!("\"{}\"", input.owner_id.as_str()).as_str())
                    .expect("restr exp parsing error"),
            );
            attributes
        }
    };

    Ok(Entity::new(entity_uid, attributes, HashSet::new())?)
}

fn get_policy_set() -> Result<PolicySet, Box<dyn Error + Send + Sync>> {
    let policy_src = r#"
    permit(
        principal is User,
        action == Action::"GetReportInfo",
        resource == Resource::"ReportData"
    );
    permit(
        principal is User,
        action == Action::"SetReportInfo",
        resource == Resource::"ReportData"
    ) when {
        resource has owner_id && resource.owner_id == principal.id
    };
    permit(
        principal is User,
        action == Action::"GenerateS3Url",
        resource == Resource::"S3Object"
    ) when {
        resource has owner_id && resource.owner_id == principal.id
    };"#;

    Ok(policy_src.parse()?)
}

pub(crate) fn authorize(
    user_id: UserId,
    action_verb: ActionVerb,
    resource_type: ResourceType,
    owner_id: OwnerId,
) -> Result<AuthorizationDecision, Box<dyn Error + Send + Sync>> {

    let policy = get_policy_set()?;

    let user_entity_input = UserEntityInput::new(user_id);

    let action_entity_input = ActionEntityInput::new(action_verb);

    let resource_entity_input = ResourceEntityInput::new(resource_type, owner_id);

    let principal = create_entity(CreateEntityInput::User(user_entity_input))?;

    let action = create_entity(CreateEntityInput::Action(action_entity_input))?;

    let resource = create_entity(CreateEntityInput::Resource(resource_entity_input))?;

    let request = Request::new(
        principal.uid(),
        action.uid(),
        resource.uid(),
        Context::empty(),
        None,
    )?;

    let entities = Entities::from_entities([principal, action, resource], None)?;

    let authorizer = Authorizer::new();

    let answer = authorizer.is_authorized(&request, &policy, &entities);

    let decision = match &answer.decision() {
        cedar_policy::Decision::Allow => {
            println!("Allowed: {:?}", &answer);
            AuthorizationDecision::Allow
        },
        cedar_policy::Decision::Deny => {
            println!("Deny: {:?}", &answer);
            AuthorizationDecision::Deny
        },
    };
    
    Ok(decision)
}