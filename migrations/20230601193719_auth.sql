CREATE TABLE account (
    id BIGINT PRIMARY KEY,
    api_key_hash BYTEA
);

CREATE TABLE entity (
    id BIGINT,
    holder_id BIGINT REFERENCES account(id),
    entity_type SMALLINT NOT NULL,
    access_token CHAR(64) NOT NULL,

    PRIMARY KEY(id, holder_id)
);
