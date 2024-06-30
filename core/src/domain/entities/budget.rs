use anyhow::anyhow;
use chrono::{DateTime, Utc};
use rust_decimal::{prelude::FromPrimitive, Decimal};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct BudgetId(Uuid);

impl From<Uuid> for BudgetId {
    fn from(id: Uuid) -> Self {
        Self(id)
    }
}

impl From<BudgetId> for Uuid {
    fn from(id: BudgetId) -> Self {
        id.0
    }
}

#[derive(Debug, Clone)]
pub struct BudgetName(String);

impl From<String> for BudgetName {
    fn from(name: String) -> Self {
        Self(name)
    }
}

impl From<BudgetName> for String {
    fn from(name: BudgetName) -> Self {
        name.0
    }
}

#[derive(Debug, Clone)]
pub struct BudgetDescription(String);

impl From<String> for BudgetDescription {
    fn from(desc: String) -> Self {
        Self(desc)
    }
}

impl From<BudgetDescription> for String {
    fn from(desc: BudgetDescription) -> Self {
        desc.0
    }
}

#[derive(Debug, Clone)]
pub struct BudgetTotal(Decimal);

impl TryFrom<f64> for BudgetTotal {
    type Error = anyhow::Error;
    fn try_from(total: f64) -> Result<BudgetTotal, anyhow::Error> {
        let f64_total = Decimal::from_f64(total);
        match f64_total {
            Some(total) => Ok(Self(total)),
            None => Err(anyhow!("Failed to parse f64 into Decimal")),
        }
    }
}

impl TryFrom<BudgetTotal> for f64 {
    type Error = anyhow::Error;

    fn try_from(value: BudgetTotal) -> Result<Self, anyhow::Error> {
        match value.0.to_string().parse::<f64>() {
            Ok(total) => Ok(total),
            Err(_) => Err(anyhow!("Failed to parse Decimal into f64")),
        }
    }
}

#[derive(Debug, Clone)]
pub struct BudgetCurrency(String);

impl From<String> for BudgetCurrency {
    fn from(currency: String) -> Self {
        Self(currency)
    }
}

impl From<BudgetCurrency> for String {
    fn from(currency: BudgetCurrency) -> Self {
        currency.0
    }
}

#[derive(Debug, Clone)]
pub struct BudgetUser(Uuid);

impl From<Uuid> for BudgetUser {
    fn from(user: Uuid) -> Self {
        Self(user)
    }
}

impl From<BudgetUser> for Uuid {
    fn from(user: BudgetUser) -> Self {
        user.0
    }
}

#[derive(Debug, Clone)]
pub struct Budget {
    pub id: BudgetId,
    pub name: BudgetName,
    pub description: BudgetDescription,
    pub total: BudgetTotal,
    pub currency: BudgetCurrency,
    pub user_id: BudgetUser,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Budget {
    pub fn new(
        id: Uuid,
        name: String,
        description: String,
        total: f64,
        currency: String,
        user_id: Uuid,
    ) -> Result<Self, anyhow::Error> {
        let now = Utc::now();
        let total = BudgetTotal::try_from(total)?;
        Ok(Self {
            id: BudgetId::from(id),
            name: BudgetName::from(name),
            description: BudgetDescription::from(description),
            total: total,
            currency: BudgetCurrency::from(currency),
            user_id: BudgetUser::from(user_id),
            created_at: now,
            updated_at: now,
        })
    }
}
