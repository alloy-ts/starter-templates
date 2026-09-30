use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use napi_derive::napi;

diesel::table! {
    posts (id) {
        id -> Integer,
        title -> Text,
        body -> Text,
        published -> Bool,
    }
}

diesel::table! {
    users (id) {
        id -> Integer,
        name -> Text,
        hair_color -> Nullable<Text>,
        banned -> Bool,
    }
}

diesel::table! {
    versions (id) {
        id -> Integer,
        crate_id -> Integer,
        num -> Text,
    }
}

diesel::table! {
    downloads (id) {
        id -> Integer,
        version_id -> Integer,
        #[sql_name = "downloads"]
        download_count -> Integer,
        counted -> Integer,
        date -> Text,
    }
}

#[derive(Queryable, Selectable, Debug, Clone)]
#[diesel(table_name = posts)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Insertable)]
#[diesel(table_name = posts)]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
}

#[derive(Queryable, Selectable, Debug, Clone)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct User {
    pub id: i32,
    pub name: String,
    pub hair_color: Option<String>,
    pub banned: bool,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub name: &'a str,
    pub hair_color: Option<&'a str>,
}

#[derive(Queryable, Selectable, Debug, Clone)]
#[diesel(table_name = downloads)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Download {
    pub id: i32,
    pub version_id: i32,
    pub download_count: i32,
    pub counted: i32,
    pub date: String,
}

#[derive(Insertable)]
#[diesel(table_name = downloads)]
pub struct NewDownload<'a> {
    pub version_id: i32,
    #[diesel(column_name = download_count)]
    pub download_count: i32,
    pub counted: i32,
    pub date: &'a str,
}

#[derive(Queryable, Selectable, Debug, Clone)]
#[diesel(table_name = versions)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Version {
    pub id: i32,
    pub crate_id: i32,
    pub num: String,
}

#[derive(Insertable)]
#[diesel(table_name = versions)]
pub struct NewVersion<'a> {
    pub crate_id: i32,
    pub num: &'a str,
}

#[napi(object)]
pub struct JsPost {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

impl From<Post> for JsPost {
    fn from(p: Post) -> Self {
        Self {
            id: p.id,
            title: p.title,
            body: p.body,
            published: p.published,
        }
    }
}

#[napi(object)]
pub struct JsUser {
    pub id: i32,
    pub name: String,
    pub hair_color: Option<String>,
    pub banned: bool,
}

impl From<User> for JsUser {
    fn from(u: User) -> Self {
        Self {
            id: u.id,
            name: u.name,
            hair_color: u.hair_color,
            banned: u.banned,
        }
    }
}

#[napi(object)]
pub struct JsNewUser {
    pub name: String,
    pub hair_color: Option<String>,
}

#[napi(object)]
pub struct JsDownload {
    pub id: i32,
    pub version_id: i32,
    pub downloads: i32,
    pub counted: i32,
    pub date: String,
}

impl From<Download> for JsDownload {
    fn from(d: Download) -> Self {
        Self {
            id: d.id,
            version_id: d.version_id,
            downloads: d.download_count,
            counted: d.counted,
            date: d.date,
        }
    }
}

#[napi(object)]
pub struct JsVersion {
    pub id: i32,
    pub crate_id: i32,
    pub num: String,
}

impl From<Version> for JsVersion {
    fn from(v: Version) -> Self {
        Self {
            id: v.id,
            crate_id: v.crate_id,
            num: v.num,
        }
    }
}

#[napi]
pub struct DbConnection {
    conn: std::sync::Mutex<SqliteConnection>,
}

#[napi]
impl DbConnection {
    #[napi(constructor)]
    pub fn new(database_url: String) -> napi::Result<Self> {
        let conn = SqliteConnection::establish(&database_url)
            .map_err(|e| napi::Error::from_reason(format!("Error connecting to {database_url}: {e}")))?;
        Ok(Self {
            conn: std::sync::Mutex::new(conn),
        })
    }

    #[napi]
    pub fn setup_tables(&self) -> napi::Result<()> {
        use diesel::connection::SimpleConnection;
        let mut conn = self.conn.lock().unwrap();
        conn.batch_execute(
            "CREATE TABLE IF NOT EXISTS posts (
                id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
                title VARCHAR NOT NULL,
                body TEXT NOT NULL,
                published BOOLEAN NOT NULL DEFAULT 0
            );
            CREATE TABLE IF NOT EXISTS users (
                id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
                name VARCHAR NOT NULL,
                hair_color VARCHAR,
                banned BOOLEAN NOT NULL DEFAULT 0
            );
            CREATE TABLE IF NOT EXISTS versions (
                id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
                crate_id INTEGER NOT NULL,
                num VARCHAR NOT NULL
            );
            CREATE TABLE IF NOT EXISTS downloads (
                id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
                version_id INTEGER NOT NULL,
                downloads INTEGER NOT NULL,
                counted INTEGER NOT NULL,
                date VARCHAR NOT NULL
            );"
        )
        .map_err(|e| napi::Error::from_reason(format!("Error setting up tables: {e}")))?;
        Ok(())
    }

    #[napi]
    pub fn create_post(&self, title: String, body: String) -> napi::Result<JsPost> {
        let mut conn = self.conn.lock().unwrap();
        let new_post = NewPost {
            title: &title,
            body: &body,
        };
        let post: Post = diesel::insert_into(posts::table)
            .values(&new_post)
            .returning(Post::as_returning())
            .get_result(&mut *conn)
            .map_err(|e| napi::Error::from_reason(format!("Error saving post: {e}")))?;
        Ok(post.into())
    }

    #[napi]
    pub fn get_posts(&self, published_only: Option<bool>, limit: Option<i32>) -> napi::Result<Vec<JsPost>> {
        use crate::posts::dsl as p_dsl;
        let mut conn = self.conn.lock().unwrap();
        let mut query = p_dsl::posts.into_boxed();
        if let Some(pub_only) = published_only {
            if pub_only {
                query = query.filter(p_dsl::published.eq(true));
            }
        }
        if let Some(lim) = limit {
            query = query.limit(lim.into());
        }
        let results = query
            .select(Post::as_select())
            .load::<Post>(&mut *conn)
            .map_err(|e| napi::Error::from_reason(format!("Error loading posts: {e}")))?;
        Ok(results.into_iter().map(Into::into).collect())
    }

    #[napi]
    pub fn get_post(&self, post_id: i32) -> napi::Result<Option<JsPost>> {
        use crate::posts::dsl as p_dsl;
        let mut conn = self.conn.lock().unwrap();
        let result = p_dsl::posts
            .find(post_id)
            .select(Post::as_select())
            .first::<Post>(&mut *conn)
            .optional()
            .map_err(|e| napi::Error::from_reason(format!("Error finding post: {e}")))?;
        Ok(result.map(Into::into))
    }

    #[napi]
    pub fn publish_post(&self, post_id: i32) -> napi::Result<JsPost> {
        use crate::posts::dsl as p_dsl;
        let mut conn = self.conn.lock().unwrap();
        let post = diesel::update(p_dsl::posts.find(post_id))
            .set(p_dsl::published.eq(true))
            .returning(Post::as_returning())
            .get_result::<Post>(&mut *conn)
            .map_err(|e| napi::Error::from_reason(format!("Error publishing post: {e}")))?;
        Ok(post.into())
    }

    #[napi]
    pub fn delete_post(&self, target_title_substring: String) -> napi::Result<u32> {
        use crate::posts::dsl as p_dsl;
        let mut conn = self.conn.lock().unwrap();
        let pattern = format!("%{target_title_substring}%");
        let num_deleted = diesel::delete(p_dsl::posts.filter(p_dsl::title.like(pattern)))
            .execute(&mut *conn)
            .map_err(|e| napi::Error::from_reason(format!("Error deleting post: {e}")))?;
        Ok(num_deleted as u32)
    }

    #[napi]
    pub fn create_users(&self, new_users: Vec<JsNewUser>) -> napi::Result<Vec<JsUser>> {
        let mut conn = self.conn.lock().unwrap();
        let mut inserted = Vec::new();
        for u in new_users {
            let nu = NewUser {
                name: &u.name,
                hair_color: u.hair_color.as_deref(),
            };
            let res: User = diesel::insert_into(users::table)
                .values(&nu)
                .returning(User::as_returning())
                .get_result(&mut *conn)
                .map_err(|e| napi::Error::from_reason(format!("Error inserting user: {e}")))?;
            inserted.push(res.into());
        }
        Ok(inserted)
    }

    #[napi]
    pub fn get_users(&self) -> napi::Result<Vec<JsUser>> {
        use crate::users::dsl as u_dsl;
        let mut conn = self.conn.lock().unwrap();
        let results = u_dsl::users
            .select(User::as_select())
            .load::<User>(&mut *conn)
            .map_err(|e| napi::Error::from_reason(format!("Error loading users: {e}")))?;
        Ok(results.into_iter().map(Into::into).collect())
    }

    #[napi]
    pub fn update_user_banned(&self, pattern: String, is_banned: bool) -> napi::Result<u32> {
        use crate::users::dsl as u_dsl;
        let mut conn = self.conn.lock().unwrap();
        let num_updated = diesel::update(u_dsl::users.filter(u_dsl::name.like(pattern)))
            .set(u_dsl::banned.eq(is_banned))
            .execute(&mut *conn)
            .map_err(|e| napi::Error::from_reason(format!("Error updating user: {e}")))?;
        Ok(num_updated as u32)
    }

    #[napi]
    pub fn execute_raw_sql(&self, sql: String) -> napi::Result<u32> {
        use diesel::connection::SimpleConnection;
        let mut conn = self.conn.lock().unwrap();
        conn.batch_execute(&sql)
            .map_err(|e| napi::Error::from_reason(format!("Error executing raw SQL: {e}")))?;
        Ok(1)
    }

    #[napi]
    pub fn create_version(&self, crate_id: i32, num: String) -> napi::Result<JsVersion> {
        let mut conn = self.conn.lock().unwrap();
        let nv = NewVersion { crate_id, num: &num };
        let ver: Version = diesel::insert_into(versions::table)
            .values(&nv)
            .returning(Version::as_returning())
            .get_result(&mut *conn)
            .map_err(|e| napi::Error::from_reason(format!("Error inserting version: {e}")))?;
        Ok(ver.into())
    }

    #[napi]
    pub fn create_download(&self, version_id: i32, downloads: i32, counted: i32, date: String) -> napi::Result<JsDownload> {
        let mut conn = self.conn.lock().unwrap();
        let nd = NewDownload {
            version_id,
            download_count: downloads,
            counted,
            date: &date,
        };
        let dl: Download = diesel::insert_into(downloads::table)
            .values(&nd)
            .returning(Download::as_returning())
            .get_result(&mut *conn)
            .map_err(|e| napi::Error::from_reason(format!("Error inserting download: {e}")))?;
        Ok(dl.into())
    }

    #[napi]
    pub fn get_downloads(&self, version_ids: Vec<i32>) -> napi::Result<Vec<JsDownload>> {
        use crate::downloads::dsl as d_dsl;
        let mut conn = self.conn.lock().unwrap();
        let results = d_dsl::downloads
            .filter(d_dsl::version_id.eq_any(version_ids))
            .order(d_dsl::date.asc())
            .select(Download::as_select())
            .load::<Download>(&mut *conn)
            .map_err(|e| napi::Error::from_reason(format!("Error loading downloads: {e}")))?;
        Ok(results.into_iter().map(Into::into).collect())
    }
}

#[napi]
pub fn establish_connection(database_url: String) -> napi::Result<DbConnection> {
    DbConnection::new(database_url)
}
