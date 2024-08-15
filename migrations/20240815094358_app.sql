-- Add migration script here
CREATE TABLE users
(
    id            UUID PRIMARY KEY            DEFAULT GEN_RANDOM_UUID(),
    username      VARCHAR(32) UNIQUE NOT NULL,
    password_hash TEXT,
    profile_img   TEXT,
    about         TEXT,
    created_at    TIMESTAMPTZ        NOT NULL DEFAULT CURRENT_TIMESTAMP
);


CREATE TABLE chats
(
    id            UUID PRIMARY KEY     DEFAULT GEN_RANDOM_UUID(),
    name          TEXT,
    description   TEXT,
    password_hash TEXT,
    profile_img   TEXT,
    created_at    TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);


CREATE TABLE messages
(
    id         UUID PRIMARY KEY     DEFAULT GEN_RANDOM_UUID(),
    origin_id  UUID        NOT NULL REFERENCES users (id),
    from_id    UUID        NOT NULL REFERENCES users (id),
    to_id      UUID        NOT NULL,
    to_type    TEXT        NOT NULL CHECK (to_type IN ('user', 'chat')),
    message    TEXT        NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE users_chats
(
    user_id         UUID NOT NULL REFERENCES users (id),
    chat_id         UUID NOT NULL REFERENCES chats (id),
    is_admin        BOOLEAN DEFAULT FALSE,
    last_message_id UUID REFERENCES messages (id),
    PRIMARY KEY (user_id, chat_id)
);

CREATE OR REPLACE FUNCTION enforce_foreign_key() RETURNS TRIGGER AS
$$
BEGIN
    IF NEW.to_type = 'user' THEN
        -- Check if the to_id exists in the users table
        PERFORM 1 FROM users WHERE id = NEW.to_id;
        IF NOT FOUND THEN
            RAISE EXCEPTION 'Foreign key violation: user with id % does not exist', NEW.to_id;
        END IF;
    ELSIF NEW.to_type = 'chat' THEN
        -- Check if the to_id exists in the chats table
        PERFORM 1 FROM chats WHERE id = NEW.to_id;
        IF NOT FOUND THEN
            RAISE EXCEPTION 'Foreign key violation: chat with id % does not exist', NEW.to_id;
        END IF;
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION trigger_set_timestamp()
    RETURNS TRIGGER AS
$$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER set_timestamp
    BEFORE UPDATE
    ON messages
    FOR EACH ROW
EXECUTE PROCEDURE trigger_set_timestamp();

CREATE TRIGGER enforce_fk_trigger
    BEFORE INSERT OR UPDATE
    ON messages
    FOR EACH ROW
EXECUTE FUNCTION enforce_foreign_key();
