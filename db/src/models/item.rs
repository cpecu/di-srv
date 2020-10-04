use serde::{Serialize, Deserialize};
use sqlx::{FromRow, types::chrono::{DateTime, Utc}, prelude::*};
use crate::{
    db::Db,
    models::{Model, user::User, record::Record, Status, Visibility, Priority, link::{RecordItemLink, ItemFieldLink},
}};

#[serde(rename_all="camelCase")]
#[derive(Serialize, Deserialize, FromRow, Clone, PartialEq)]
pub struct Item {
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<i32>,
    pub uid: i32,
    pub name: String,
    pub status: i32,
    pub visibility: i32,
    #[serde(default="Utc::now")]
    pub created_at: DateTime<Utc>,
}

impl Item {

    pub fn new(uid: i32, name: String) -> Self {
        Self { uid, name, ..Self::default() }
    } 

    pub fn private(&mut self, visibility: Visibility) -> Self {
        Self { visibility: visibility as i32, ..self.to_owned() }
    }

    pub fn with_status(&mut self, status: Status) -> Self {
        Self { status: status as i32, ..self.to_owned() }
    }

    pub async fn insert(self, db: &Db) -> sqlx::Result<u32> {
        let res = sqlx::query(
            "INSERT INTO Items (uid, name, status, visibility, created_at)
             VALUES ($1, $2, $3, $4, $5) RETURNING id")
            .bind(&self.uid)
            .bind(&self.name)
            .bind(&self.status)
            .bind(&self.visibility)
            .bind(&self.created_at)
            .execute(&db.pool).await?;
        Ok(res.rows_affected() as u32)
    }

}

impl Default for Item {
    fn default() -> Self {
        Self {
            id: None,
            uid: -1,
            name: String::new(),
            status: Status::Active as i32,
            visibility: Visibility::Private as i32,
            created_at: Utc::now(),
        }
    }
}

impl From<Record> for Item {
    fn from(record: Record) -> Self {
        Self { uid: record.uid, ..Self::default() }
    }
}

impl From<User> for Item {
    fn from(user: User) -> Self {
        Self { uid: user.id.expect("User ID not set"), ..Self::default() }
    }
}


#[async_trait::async_trait]
impl Model for Item {
    fn table() -> String { String::from("Items") }
}
