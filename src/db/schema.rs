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
    members (id) {
        id -> Int8,
        user_id -> Int8,
        team_id -> Int8,
        admin -> Bool,
    }
}

table! {
    teams (id) {
        id -> Int8,
        owner_id -> Int8,
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
joinable!(teams -> users (owner_id));

allow_tables_to_appear_in_same_query!(
    datasets,
    entries,
    forms,
    members,
    teams,
    users,
);
