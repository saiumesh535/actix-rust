CREATE DATABASE "actix-web";

CREATE TABLE rust_users (
   user_id serial PRIMARY KEY,
   username VARCHAR (50) UNIQUE NOT NULL,
   password VARCHAR (50) NOT NULL
);