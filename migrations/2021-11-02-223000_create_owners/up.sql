CREATE TABLE owners (
	id BIGSERIAL PRIMARY KEY,
	-- one of these will not be null, and that will be the owner
	user_id BIGINT REFERENCES users,
	team_id BIGINT REFERENCES teams
);
