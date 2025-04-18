use std::{collections::HashMap, usize, vec};

use deadpool_postgres::{GenericClient, Manager, Object, Pool};
use uuid::Uuid;
use crate::{get_client, structs::tag::{NamedTag, NamedTagValue, NamedValueTag}};

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
        create table if not exists public.file_tags (id serial not null constraint file_tags_pk primary key, file_id varchar(36) not null constraint file_tags_file_id_fk references public.file, tag_value_id integer constraint file_tags_file_tags_id_fk references public.file_tags);
    ").await;
}

pub async fn add_file(pool: &Pool, file_name: String, filetype: i32) -> Result<String, String> {
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

pub async fn add_file_tag(pool: &Pool, file_id: &str, tag_value_id: i32) -> Result<i32, String> {

    let client = get_client!(pool);

    let _ = match client.execute("insert into file_tags (file_id, tag_value_id) values ($1::TEXT, $2::INT);", &[&file_id, &tag_value_id]).await {
        Ok(_) => {},
        Err(err) => {
            return Err(err.to_string())
        },
    };

    let potential_rows = match client.query("select id from tag_values where file_id = $1::TEXT AND tag_value_id = $2::INT;", &[&file_id, &tag_value_id]).await {
        Ok(file) => file,
        Err(err) => {
            return Err(err.to_string());
        },
    };

    if potential_rows.is_empty() {
        return Err("No such tag value!".to_string());
    }

    Ok(potential_rows[0].get(0))
}


pub async fn create_tag(pool: &Pool, tag_name: String) -> Result<i32, String> {

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


pub async fn create_tag_value(pool: &Pool, tag_id: i32, value: &str) -> Result<i32, String> {

    let client = get_client!(pool);

    let _ = match client.execute("insert into tag_values (tag_id, value) values ($1::INT, $2::TEXT);", &[&tag_id, &value]).await {
        Ok(_) => {},
        Err(err) => {
            return Err(err.to_string());
        },
    };


    let potential_rows = match client.query("select id from tag_values where value = $1::TEXT AND tag_id = $2::INT;", &[&value, &tag_id]).await {
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

pub async fn get_all_tags(pool: &Pool) -> Result<Vec<NamedTag>, String> {

    let client = get_client!(pool);

    let potential_rows = match client.query("select tag.id, tag.name, tag_values.id, tag_values.value from tag join tag_values on tag.id = tag_values.tag_id;", &[]).await {
        Ok(rows) => rows,
        Err(err) => {
            return Err(err.to_string())
        },
    };

    if potential_rows.is_empty() {
        return Err("No tags!".to_string());
    }

    let mut tags_map: HashMap<i32, NamedTag> = HashMap::default();

    for row in potential_rows {
        
        let id: i32 = row.get(0);
        let name: String = row.get(1);
        let value_id: i32 = row.get(2);
        let value: String = row.get(3);        

        match tags_map.get_mut(&id) {
            Some(tag) => {
                tag.values.push(NamedTagValue { id: value_id, value })
            },
            None => {
                tags_map.insert(id.clone(), NamedTag { id, name, values: vec![NamedTagValue { id: value_id, value }] });
            },
        }
    }

    let mut tags: Vec<NamedTag> = vec![];

    for (_, tag) in tags_map {
        tags.push(tag);
    }

    Ok(tags)
}

pub async fn get_files_with_tag(pool: &Pool, tags: Vec<i32>) {
    
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





