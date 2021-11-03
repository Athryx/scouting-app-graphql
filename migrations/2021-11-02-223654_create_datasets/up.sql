CREATE TABLE datasets (
	id BIGSERIAL PRIMARY KEY,
	team_id BIGINT NOT NULL REFERENCES teams,
	form_id BIGINT NOT NULL REFERENCES forms,
	name VARCHAR(80) NOT NULL
);

SELECT diesel_manage_updated_at('datasets');
