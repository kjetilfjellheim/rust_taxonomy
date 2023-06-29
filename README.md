# Rust taxonomy api

## Description
This is an api for returning taxonomy data from a postgres data with ITIS data.
ITIS data can be downloaded here https://www.itis.gov/downloads/index.html
I had to fix some data so that it imported into a utf-8 database while the input 
data was iso-8859.

## Build
Run the following command. I used rust version 1.7.0.
cargo build

## Run
Setup database
Fill out .env file. My local -env test file ahs been included in the application. 
cargo run

## Changes in database
This is a view for querying the taxonomy data. 
CREATE OR REPLACE VIEW public.v_taxonomy
 AS
 SELECT a.tsn,
    a.complete_name,
    a.parent_tsn,
    b.kingdom_name,
    c.rank_name,
    d.hierarchy_string
   FROM taxonomic_units a,
    kingdoms b,
    taxon_unit_types c,
    hierarchy d
  WHERE a.kingdom_id = b.kingdom_id AND a.kingdom_id = c.kingdom_id AND a.rank_id = c.rank_id AND a.tsn = d.tsn;

ALTER TABLE public.v_taxonomy
    OWNER TO postgres;

This index makes querying parent tsn much quicker.
CREATE INDEX IF NOT EXISTS ix_parent_tsn
    ON public.taxonomic_units USING hash
    (parent_tsn)
    TABLESPACE pg_default;

## Performance tests
Performance tests have been included in the k6 directory. To run them use the following commands.
k6 run performance_get.js
k6 run performance_list.js

## Logging configuration
The configuration file uses log4rs and an examnple is stored in resources/logging.yaml

## Ide 
I have used VSCode for development. Configuration files have not been included since you may choose any other development environment.

## Information
This api was written in an attempt to learn Rust. I learnt a lot during development so the code is far from perfect.
