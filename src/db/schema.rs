table! {
    datasets (id) {
        id -> Int8,
        team_id -> Int8,
        form_id -> Int8,
        name -> Varchar,
    }
}

table! {
    entries (id) {
        id -> Int8,
        dataset_id -> Int8,
    }
}

table! {
    forms (id) {
        id -> Int8,
        team_id -> Int8,
        name -> Varchar,
    }
}

table! {
	use diesel::types::Int8;
	use crate::db::team::MemberTypeMapping;
    members (id) {
        id -> Int8,
        user_id -> Int8,
        team_id -> Int8,
        mtype -> MemberTypeMapping,
    }
}

table! {
    teams (id) {
        id -> Int8,
        name -> Varchar,
    }
}

table! {
    users (id) {
        id -> Int8,
        name -> Varchar,
        password -> Text,
    }
}

joinable!(datasets -> forms (form_id));
joinable!(datasets -> teams (team_id));
joinable!(entries -> datasets (dataset_id));
joinable!(forms -> teams (team_id));
joinable!(members -> teams (team_id));
joinable!(members -> users (user_id));

allow_tables_to_appear_in_same_query!(
    datasets,
    entries,
    forms,
    members,
    teams,
    users,
);
