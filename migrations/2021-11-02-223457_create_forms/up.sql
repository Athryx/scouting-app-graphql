CREATE TABLE forms (
	id BIGSERIAL PRIMARY KEY,
	team_id BIGINT NOT NULL REFERENCES teams,
	name VARCHAR(80) NOT NULL
	-- TODO: figure out how to store form data
);

SELECT diesel_manage_updated_at('forms');
