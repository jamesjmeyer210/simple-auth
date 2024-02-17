drop table if exists `realms`;
create table if not exists `realms` (
  `name` text primary key not null unique,
  `created_on` text not null,
  `deleted_on` text
);

drop table if exists `realm_events`;
create table if not exists `realm_events` (
    `id` integer primary key autoincrement not null,
    `realm_id` text not null,
    `type` integer not null,
    `value` blob,
    `ip_address` blob,
    `created_on` text not null,
    `deleted_on` text,
    constraint fk_realms
        foreign key (`realm_id`)
            references realms(`name`)
            on delete cascade
);

drop table if exists `roles`;
create table if not exists `roles` (
    `name` text primary key not null unique,
    `max` integer,
    `realm_id` text not null,
    `created_on` text not null,
    `deleted_on` text,
    constraint fk_realm
        foreign key (`realm_id`)
            references realms(`name`)
            on delete cascade
);

drop table if exists `role_events`;
create table if not exists `role_events` (
    `id` integer primary key autoincrement not null,
    `role_id` text not null,
    `type` integer not null,
    `value` blob,
    `ip_address` blob,
    `created_on` text not null,
    `deleted_on` text,
    constraint fk_realms
        foreign key (`role_id`)
            references roles(`name`)
            on delete cascade
);

drop table if exists `role_events`;
create table if not exists `role_events` (
    `id` integer primary key autoincrement not null,
    `user_id` text not null,
    `type` integer not null,
    `value` blob,
    `ip_address` blob,
    `created_on` text not null,
    `deleted_on` text,
    constraint fk_realms
        foreign key (`user_id`)
            references users(`id`)
            on delete cascade
);

drop table if exists `users`;
create table if not exists `users` (
    `id` text primary key not null unique,
    `name` text not null unique,
    `password` blob not null,
    `created_on` text not null,
    `deleted_on` text
);

drop table if exists `users_to_realms`;
create table if not exists `users_to_realms` (
     `user_id` text not null,
     `realm_id` text not null,
     constraint fk_user_id
         foreign key (`user_id`)
             references users(`id`)
             on delete cascade,
     constraint fk_realm_id
         foreign key (`realm_id`)
             references realms(`name`)
             on delete cascade
);

drop table if exists `users_to_roles`;
create table if not exists `users_to_roles` (
    `user_id` text not null,
    `role_id` text not null,
    constraint fk_user_id
        foreign key (`user_id`)
            references users(`id`)
            on delete cascade,
    constraint fk_role_id
        foreign key (`role_id`)
            references roles(`name`)
            on delete cascade
);

drop table if exists `users_contact_info`;
create table if not exists `users_contact_info` (
    `hash` blob primary key not null unique,
    `user_id` text not null,
    `label` text not null,
    `enc` blob not null,
    `verified` boolean not null,
    `created_on` text not null,
    `deleted_on` text,
    foreign key (`user_id`)
        references users(`id`)
        on delete cascade
);

drop table if exists `secrets`;
create table if not exists `secrets` (
    `key` text primary key not null,
    `value_enc` blob not null,
    `expires_on` text
);