# rust-diesel-actix-test
sandbox for a simple rust web application


## Rough edges of Diesel
Some things everyone does wrong when using Diesel for the first time

### Diesel & Diesel CLI compilation
You need to have the client lib for your DBMS installed: for mysql you need to install package 'libmysqlclient-dev' (LINUX)

### What the f**k is Diesel CLI?
The documentation is not very specific about his one: 
* handle mirgrations (=setups and cleanups for database schemas)
* code genereation: generate `schema.rs` from up.sql of migration

Setup diesel-cli (will generate diesel.toml)
```
diesel setup --database-url [url]
```

Generate new migration
```
diesel migration generate [name] --database-url [url]
```

Execute migration and generate source
```
diesel migration run --database-url [url]
```

Execute migration and generate source
```
diesel migration redo --database-url [url]
```

### Diesel up.sql and down.sql
* up.sql -- setup for database "do changes"
* down.sql -- reset for database "undo changes"

### Diesel up.sql table definition and struct definition
Types and their order must match EXACTLY. 
An `Integer` in up.sql is an `i32` and NOT an `i8` or something else.