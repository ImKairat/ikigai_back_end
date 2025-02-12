CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    email VARCHAR UNIQUE,
    phone VARCHAR UNIQUE,
    google_id VARCHAR UNIQUE,
    password_hash VARCHAR,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE roles (
    id SERIAL PRIMARY KEY,
    name VARCHAR UNIQUE
);

CREATE TABLE user_roles (
    user_id INT REFERENCES users(id),
    role_id INT REFERENCES roles(id),
    PRIMARY KEY (user_id, role_id)
);

CREATE TABLE verification_codes (
    id SERIAL PRIMARY KEY,
    phone VARCHAR,
    code VARCHAR,
    expires_at TIMESTAMP
);

CREATE TABLE sessions (
    id VARCHAR PRIMARY KEY,
    user_id INT REFERENCES users(id),
    expires_at TIMESTAMP
);
