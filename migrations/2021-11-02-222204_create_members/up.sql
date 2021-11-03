CREATE TABLE members (
	id BIGSERIAL PRIMARY KEY,
	user_id BIGINT NOT NULL REFERENCES users,
	team_id BIGINT NOT NULL REFERENCES teams,
	admin BOOL NOT NULL
);

SELECT diesel_manage_updated_at('members');
