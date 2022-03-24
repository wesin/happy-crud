-- Database: happy_crud

-- DROP DATABASE IF EXISTS happy_crud;

CREATE DATABASE happy_crud
    WITH 
    OWNER = postgres
    ENCODING = 'UTF8'
    LC_COLLATE = 'en_US.utf8'
    LC_CTYPE = 'en_US.utf8'
    TABLESPACE = pg_default
    CONNECTION LIMIT = -1;

CREATE TABLE IF NOT EXISTS public.person
(
    id uuid NOT NULL,
    name character varying COLLATE pg_catalog."default" NOT NULL,
    age integer,
    "desc" character varying COLLATE pg_catalog."default",
    CONSTRAINT person_pkey PRIMARY KEY (id)
)