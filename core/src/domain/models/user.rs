use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub struct UserDraft {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug)]
pub struct UserPatch {
    pub name: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug)]
pub struct NewUser {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub password: String,
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
    pub email: String,
    pub password: String,
}

impl From<CreateUserCommand> for UserDraft {
    fn from(command: CreateUserCommand) -> Self {
        UserDraft {
            name: command.name,
            email: command.email,
            password: command.password,
        }
    }
}

#[derive(Debug)]
pub struct UpdateUserCommand {
    pub id: Uuid,
    pub name: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub updated_at: DateTime<Utc>,
}

impl From<UpdateUserCommand> for UserPatch {
    fn from(command: UpdateUserCommand) -> Self {
        UserPatch {
            name: command.name,
            email: command.email,
            password: command.password,
            updated_at: command.updated_at,
        }
    }
}

#[derive(Debug)]
pub struct GetAllUsersQuery {}

#[derive(Debug)]
pub struct GetAllUsersQueryResult {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<NewUser> for GetAllUsersQueryResult {
    fn from(entity: NewUser) -> Self {
        GetAllUsersQueryResult {
            id: entity.id,
            name: entity.name,
            email: entity.email,
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
    pub email: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<NewUser> for GetOneUserQueryResult {
    fn from(entity: NewUser) -> Self {
        GetOneUserQueryResult {
            id: entity.id,
            name: entity.name,
            email: entity.email,
            created_at: entity.created_at,
            updated_at: entity.updated_at,
        }
    }
}
