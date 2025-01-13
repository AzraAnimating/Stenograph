use deadpool_postgres::Pool;

pub async fn setup(pool: Pool) {

    let client = match pool.get().await {
        Ok(client) => client,
        Err(err) => {
            println!("{:?}", err);
            return;
        },
    };
                                                                                                                                                                                                                                                                                                                                                                                                                
    let _ = client.simple_query("
        create table if not exists public.tag (id serial not null constraint tag_pk primary key, name varchar(128) not null);
        create table if not exists public.tag_values (id serial constraint tag_values_pk primary key, tag_id integer constraint tag_values_tag_id_fk references public.tag, value varchar(128) not null);
        create table if not exists public.file (id varchar(36) not null constraint files_pk primary key, name varchar(128) not null, datatype integer not null);
        create table if not exists public.file_tags (id serial not null constraint file_tags_pk primary key, file_id varchar(36) not null constraint file_tags_file_id_fk references public.file, tag_value integer constraint file_tags_file_tags_id_fk references public.file_tags);
    ").await;
    
    

}
