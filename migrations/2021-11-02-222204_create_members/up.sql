CREATE TYPE member_type AS ENUM ('owner', 'admin', 'member');

CREATE TABLE members (
	id BIGSERIAL PRIMARY KEY,
	user_id BIGINT NOT NULL REFERENCES users,
	team_id BIGINT NOT NULL REFERENCES teams,
	mtype member_type NOT NULL
);

SELECT diesel_manage_updated_at('members');
