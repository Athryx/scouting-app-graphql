CREATE TABLE users (
	id BIGSERIAL PRIMARY KEY,
	-- use length limited varchar to top usernames taking up too much space
	name VARCHAR(80) NOT NULL UNIQUE,
	password TEXT NOT NULL
);

SELECT diesel_manage_updated_at('users');
