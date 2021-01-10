create table messages
(
    id serial
        constraint messages_pk
            primary key,
    msg text
);
