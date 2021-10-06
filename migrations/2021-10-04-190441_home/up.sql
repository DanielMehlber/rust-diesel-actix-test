-- Your SQL goes here

-- be extra careful here: types here must PRECISELY match types of struct
-- e.g. INT is an i32 NOT AN i8 -- "trait not satisfied" otherwise... 

create table person (
    name VARCHAR(15) PRIMARY KEY,
    phrase TEXT NOT NULL,
    age INT NOT NULL
);