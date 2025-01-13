use core::panic;

use crate::models::*;
use deadpool_postgres::Pool;
use diesel::{prelude::*, Connection, ExpressionMethods, PgConnection};
use serenity::all::colours::branding::RED;

use crate::structs::configuration::DatabaseConfiguration;

pub fn connect(database_config: DatabaseConfiguration) -> PgConnection {
    let database_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        &database_config.username,
        &database_config.password,
        &database_config.hostname,
        &database_config.port,
        &database_config.database
    );
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Failed to connect to database!"))
}

pub fn get_file(file_id: &str, conn: &mut PgConnection) -> Result<Option<File>, Error> {
    use crate::schema::file::dsl::*;

    let files = match file
        .filter(id.eq(file_id))
        .select(File::as_select())
        .load(conn)
    {
        Ok(data) => data,
        Err(err) => return Err(err),
    };

    if files.len() > 1 {
        return Ok(Some(file));
    }

    return Ok(None);
}

pub async fn setup(pool: Pool) {
    let client = match pool.get().await {
        Ok(client) => client,
        Err(err) => {
            println!("{:?}", err);
            return;
        }
    };

    let _ = client.simple_query("
        create table if not exists public.tag (id serial not null constraint tag_pk primary key, name varchar(128) not null);
        create table if not exists public.tag_values (id serial constraint tag_values_pk primary key, tag_id integer constraint tag_values_tag_id_fk references public.tag, value varchar(128) not null);
        create table if not exists public.file (id varchar(36) not null constraint files_pk primary key, name varchar(128) not null, datatype integer not null);
        create table if not exists public.file_tags (id serial not null constraint file_tags_pk primary key, file_id varchar(36) not null constraint file_tags_file_id_fk references public.file, tag_value integer constraint file_tags_file_tags_id_fk references public.file_tags);
    ").await;
}
