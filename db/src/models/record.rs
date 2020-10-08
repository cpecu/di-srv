use std::rc::Weak;
use sqlx::{FromRow, types::chrono::{DateTime, Utc}, prelude::*, Postgres, postgres::PgRow};
use serde::{Serialize, Deserialize};
use crate::{Db, 
    models::{Model, User, Status, Visibility, Priority, Item, Group,
        link::{LinkedTo, Link},
    },
};

//TODO add validation so that user/record name combo is unique
#[serde(rename_all="camelCase")]
#[derive(Serialize, Deserialize, FromRow, Clone)]
pub struct Record {
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<i32>,
    pub uid: i32,
    pub name: String,
    #[serde(default="Status::default")]
    pub status: Status,
    #[serde(default="Visibility::default")]
    pub visibility: Visibility,
    #[serde(default="Utc::now")]
    pub created_at: DateTime<Utc>,
}

impl Record {

    pub fn new<T, U>(uid: T, name: U) -> Self 
    where T: Into<i32>, U: Into<String> {
        Self { 
            name: name.into(),  
            uid: uid.into(),
            ..Self::default()
        }
    }

    pub fn create<T, U, V, W>
        (uid: T, name: U, status: V, visibility: W) -> Self    
        where T: Into<i32>, U: Into<String>, 
              V: Into<Status>, W: Into<Visibility> {
        Self { 
            name: name.into(),
            uid: uid.into(),
            status: status.into(),
            visibility: visibility.into(),
            ..Self::default()
        }
    }

    fn build() -> () {}

    pub fn set_name<T>(self, name: T) -> Self where T: Into<String> {
        Self { name: name.into(), ..self }
    }

    pub fn set_status<T>(self, status: T) -> Self where T: Into<Status> {
        Self { status: status.into(), ..self }
    }

    pub fn set_visibility<T>(self, visibility: T) -> Self where T: Into<Visibility> {
        Self { visibility: visibility.into(), ..self }
    }

    pub async fn update_visibility<T>(
        self, db: &Db, visibility: T, id: i32,
    ) -> sqlx::Result<Self> where T: Into<Visibility>{
        let vis = visibility.into();
        sqlx::query("UPDATE Records SET visibility=$1 WHERE id=$2")
                .bind(&vis)
                .bind(id)
                .execute(&db.pool).await?;
        Ok ( Self { visibility: vis, ..self } )
    }

    pub async fn update_status<T>(
        self, db: &Db, status: T, id: i32,
    ) -> sqlx::Result<Self> where T: Into<Status>{
        let stat = status.into();
        sqlx::query("UPDATE Records SET visibility=$1 WHERE id=$2")
                .bind(&stat)
                .bind(id)
                .execute(&db.pool).await?;
        Ok ( Self { status: stat, ..self } )
    }

    // implemented in model trait -- remove?
    pub async fn get_by_id(db: &Db, id: i32) -> sqlx::Result<Vec<Self>> {
        let res: Vec<Record> = sqlx::query_as::<Postgres, Record>(
            "SELECT * FROM Records WHERE id=$1")
            .bind(id)
            .fetch_all(&db.pool).await?;
        Ok(res)
    }

    pub async fn insert(self, db: &Db) -> sqlx::Result<Self> {
        let res: i32 = sqlx::query(
            "INSERT INTO Records (uid, name, status, visibility, created_at)
             VALUES ($1, $2, $3, $4, $5) RETURNING id")
            .bind(&self.uid)
            .bind(&self.name)
            .bind(&self.status)
            .bind(&self.visibility)
            .bind(&self.created_at)
            .fetch_one(&db.pool).await?
            .get("id");
        Ok( Self { id: Some(res), ..self })
    }

    // implemented in linkedto trait -- remove?
    pub async fn get_linked_users(db: &Db, rid: i32) -> sqlx::Result<Vec<User>> {
        let res = sqlx::query_as::<Postgres, User>
            ("SELECT u.id, u.username, u.email, u.created_at
              FROM Users u INNER JOIN UserRecordLinks ur ON u.id = ur.uid
                   INNER JOIN Records r on ur.rid = r.id
                   AND r.id = $1")
            .bind(rid)
            .fetch_all(&db.pool)
            .await?;
        Ok(res)
    }

    pub async fn add_new_item<T: Into<String>>
        (db: &Db, rid: i32, item_name: T) -> sqlx::Result<Item> 
    {
        let item = Item::new(rid, item_name.into()).insert(db).await?;
        let link = Link::new(Some(rid), item.id).insert::<Record, Item>(db).await?;
        Ok(item)
    }

    pub async fn add_existing_item(db: &Db, rid: i32, iid: i32) -> sqlx::Result<i32> 
    {
        let link = Link::new(Some(rid), Some(iid)).insert::<Record, Item>(db).await?;
        Ok(link)
    }

    pub async fn delete_by_id(db: &Db, id:  i32) -> sqlx::Result<i32> {
        let res = sqlx::query(
            "DELETE FROM Records WHERE id=$1 RETURNING id")
            .bind(id)
            .fetch_one(&db.pool).await?;
        Ok( res.get("id") )
    }

    pub async fn update_by_id(db: &Db, id: i32, record: Record) -> sqlx::Result<i32> {
        let res = sqlx::query(
            "DELETE FROM Records WHERE id=$1 RETURNING id")
            .bind(id)
            .fetch_one(&db.pool).await?;
        Ok( res.get("id") )
    }
}

impl Default for Record {
    fn default() -> Self {
        Self { 
            id: None, 
            uid: -1, 
            name: String::new(), 
            status: Status::Active.into(),
            visibility: Visibility::Private.into(),
            created_at: Utc::now(),
        } 
    }
}

impl From<Option<i32>> for Record {
    fn from(uid: Option<i32>) -> Self {
        Record { uid: uid.unwrap(), ..Record::default() }
    }
}

impl From<User> for Record {
    fn from(user: User) -> Self {
        Record { uid: user.id.unwrap(), ..Record::default() }
    }
}

impl From<&'static PgRow> for Record {
    fn from(row: &'static PgRow) -> Self {
        Record::from_row(row).unwrap()
    }
}

#[async_trait::async_trait]
impl Model for Record {
    fn table() -> String { String::from("Records") }
    fn foreign_id() -> String { String::from("rid") }
    fn id(self) -> i32 { self.id.unwrap() }
    fn fields() ->  Vec<String> { 
        let fields = vec!["id", "uid", "name", "status", "visibility", "created_at"];
        fields.into_iter()
            .map(|field| field.to_string())
            .collect::<Vec<String>>()
    }
}

impl LinkedTo<User> for Record {  }
impl LinkedTo<Item> for Record {  }
impl LinkedTo<Group> for Record {  }
