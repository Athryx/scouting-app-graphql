CREATE TABLE teams (
	id BIGSERIAL PRIMARY KEY,
	owner_id BIGINT NOT NULL REFERENCES users,
	name VARCHAR(80) NOT NULL UNIQUE
);

SELECT diesel_manage_updated_at('teams');
