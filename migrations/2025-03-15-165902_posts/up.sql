-- Your SQL goes here
create table if not exists posts (
    id serial primary key,
    title varchar not null default '',
    content varchar not null default ''
);
