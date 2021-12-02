CREATE TABLE teams (
	id BIGSERIAL PRIMARY KEY,
	name VARCHAR(80) NOT NULL UNIQUE
);

SELECT diesel_manage_updated_at('teams');
