-- Add migration script here
CREATE TABLE users
(
    id                  bigint PRIMARY KEY,
    username            varchar(255) NOT NULL,
    permissions         bigint,
    password            varchar(255) NOT NULL,
    email               varchar(255) NOT NULL,
    profile_picture_url varchar(2048),
    first_name          varchar(255),
    last_name           varchar(255),
    display_name        varchar(255)
);
CREATE TABLE clients
(
    id             bigint PRIMARY KEY                        NOT NULL,
    owner          bigint REFERENCES users ON UPDATE CASCADE NOT NULL,
    name           varchar(32)                               NOT NULL,
    -- The display_name can be null. In this case, the name takes place
    display_name   varchar(256),
    icon_url       varchar(2048),
    public_key     varchar(256)                              NOT NULL,
    secret_key     varchar(256)                              NOT NULL,
    return_urls    varchar(1024)[]                           NOT NULL,
    granted_scopes varchar(256)[]                            NOT NULL
);
CREATE TABLE refresh_tokens
(
    -- The id is the token itself, returned to the user
    id           varchar(36) PRIMARY KEY                     NOT NULL,
    client_owner bigint REFERENCES clients ON UPDATE CASCADE NOT NULL,
    "user"       bigint REFERENCES users ON UPDATE CASCADE   NOT NULL,
    usages       int                                         NOT NULL
);
