USE labDB;

drop table if exists users;

create table users
(
    id INT AUTO_INCREMENT PRIMARY KEY,
    username varchar(60) not null,
    email varchar(140) not null,
    `password` varchar(255) not null,
    created_at TIMESTAMP default now(),
    CONSTRAINT ix_users_email UNIQUE (email)
);