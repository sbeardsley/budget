use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct UserId(Uuid);

impl From<Uuid> for UserId {
    fn from(user: Uuid) -> Self {
        Self(user)
    }
}

impl From<UserId> for Uuid {
    fn from(user: UserId) -> Self {
        user.0
    }
}

#[derive(Debug, Clone)]
pub struct UserName(String);

impl From<String> for UserName {
    fn from(user: String) -> Self {
        Self(user)
    }
}

impl From<UserName> for String {
    fn from(user: UserName) -> Self {
        user.0
    }
}

#[derive(Debug, Clone)]
pub struct User {
    pub id: UserId,
    pub name: UserName,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl User {
    pub fn new(name: String) -> Self {
        let now = Utc::now();
        Self {
            id: UserId(Uuid::now_v7()),
            name: UserName(name),
            created_at: now,
            updated_at: now,
        }
    }
}
