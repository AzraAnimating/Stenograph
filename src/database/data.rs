use deadpool_postgres::{GenericClient, Pool};
use tokio_postgres::row;
use uuid::Uuid;
use crate::get_client;

pub async fn setup(pool: Pool) {

    let client = match pool.get().await {
        Ok(client) => client,
        Err(err) => {
            panic!("Failed to setup Database!: {:?}", err);
        },
    };
                                                                                                                                                                                                                                                                                                                                                                                                                
    let _ = client.simple_query("
        create table if not exists public.tag (id serial not null constraint tag_pk primary key, name varchar(128) not null);
        create table if not exists public.tag_values (id serial constraint tag_values_pk primary key, tag_id integer constraint tag_values_tag_id_fk references public.tag, value varchar(128) not null);
        create table if not exists public.file (id varchar(36) not null constraint files_pk primary key, name varchar(128) not null, datatype integer not null);
        create table if not exists public.file_tags (id serial not null constraint file_tags_pk primary key, file_id varchar(36) not null constraint file_tags_file_id_fk references public.file, tag_value integer constraint file_tags_file_tags_id_fk references public.file_tags);
    ").await;
}

pub async fn add_file(pool: Pool, file_name: String, filetype: i32) -> Result<String, String> {
    let client = get_client!(pool);

    let mut uuid = Uuid::new_v4().to_string();
    let mut found = false;


    for _ in 0..10 {
        let potential_row = match client.query("select id from file where id = $1::TEXT;", &[&uuid]).await {
            Ok(file) => file,
            Err(err) => {
                return Err(err.to_string());
            },
        };

        if potential_row.is_empty() {
            found = true;
            break;
        }

        uuid = Uuid::new_v4().to_string();
    }

    if !found {
        return Err("No free UUID found".to_string());
    }

    match client.execute("insert into file values ($1::TEXT, $2::TEXT, $3::INT);", &[&uuid, &file_name, &filetype]).await {
        Ok(_) => {},
        Err(err) => {
            return Err(err.to_string());
        },
    };

    Ok(uuid)
}


pub async fn create_tag(pool: Pool, tag_name: String) -> Result<i32, String> {
    let client = get_client!(pool);

    let potential_rows = match client.query("select id from tag where name = $1::TEXT;", &[&tag_name]).await {
        Ok(file) => file,
        Err(err) => {
            return Err(err.to_string());
        },
    };

    if !potential_rows.is_empty() {
        return Ok(potential_rows[0].get(0));
    }
    
    let _ = match client.execute("insert into tag (name) values ($1::TEXT);", &[&tag_name]).await {
        Ok(_) => {},
        Err(err) => {
            return Err(err.to_string())
        },
    };

    let potential_rows = match client.query("select id from tag where name = $1::TEXT;", &[&tag_name]).await {
        Ok(file) => file,
        Err(err) => {
            return Err(err.to_string());
        },
    };

    if potential_rows.is_empty() {
        return Err("No such tag!".to_string());
    }

    Ok(potential_rows[0].get(0))
}


#[macro_export]
macro_rules! get_client {
    ($pool:expr) => {
      match $pool.get().await {
          Ok(client) => client,
          Err(err) => {
              return Err(err.to_string());
          },
      }
    };
}
