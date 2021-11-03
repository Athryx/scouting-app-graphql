CREATE TABLE entries (
	id BIGSERIAL PRIMARY KEY,
	dataset_id BIGINT NOT NULL REFERENCES datasets
	-- TODO: figure out how to store data
);

SELECT diesel_manage_updated_at('entries');
