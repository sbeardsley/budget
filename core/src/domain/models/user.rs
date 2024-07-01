use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug)]
pub struct UserDraft {
    pub name: String,
}

#[derive(Debug)]
pub struct UserPatch {
    pub name: Option<String>,
}

#[derive(Debug)]
pub struct NewUser {
    pub id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug)]
pub struct DeleteUser {
    pub id: Uuid,
}

#[derive(Debug)]
pub struct CreateUserCommand {
    pub name: String,
}

impl From<CreateUserCommand> for UserDraft {
    fn from(command: CreateUserCommand) -> Self {
        UserDraft { name: command.name }
    }
}

#[derive(Debug)]
pub struct UpdateUserCommand {
    pub id: Uuid,
    pub name: Option<String>,
}

impl From<UpdateUserCommand> for UserPatch {
    fn from(command: UpdateUserCommand) -> Self {
        UserPatch { name: command.name }
    }
}

#[derive(Debug)]
pub struct GetAllUsersQuery {}

#[derive(Debug)]
pub struct GetAllUsersQueryResult {
    pub id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<NewUser> for GetAllUsersQueryResult {
    fn from(entity: NewUser) -> Self {
        GetAllUsersQueryResult {
            id: entity.id,
            name: entity.name,
            created_at: entity.created_at,
            updated_at: entity.updated_at,
        }
    }
}

#[derive(Debug)]
pub struct GetOneUserQuery {
    pub user_id: Uuid,
}

#[derive(Debug)]
pub struct GetOneUserQueryResult {
    pub id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<NewUser> for GetOneUserQueryResult {
    fn from(entity: NewUser) -> Self {
        GetOneUserQueryResult {
            id: entity.id,
            name: entity.name,
            created_at: entity.created_at,
            updated_at: entity.updated_at,
        }
    }
}
