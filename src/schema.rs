// @generated automatically by Diesel CLI.

diesel::table! {
    bookmark (bookmark_id) {
        bookmark_id -> Int4,
        user_fk -> Int4,
        bookmark_name -> Varchar,
        release_fk -> Int4,
    }
}

diesel::table! {
    episode (episode_id) {
        episode_id -> Int4,
        release_fk -> Int4,
        ep_name -> Nullable<Varchar>,
        url -> Varchar,
    }
}

diesel::table! {
    history (history_id) {
        history_id -> Int4,
        user_fk -> Int4,
        episode -> Int4,
        date_watched -> Date,
    }
}

diesel::table! {
    releases (release_id) {
        release_id -> Int4,
        release_type -> Varchar,
        release_date -> Date,
        rating -> Numeric,
        min_age -> Int4,
        director -> Nullable<Varchar>,
        author -> Nullable<Varchar>,
        studio -> Nullable<Varchar>,
        description -> Varchar,
    }
}

diesel::table! {
    review (review_id) {
        review_id -> Int4,
        user_fk -> Int4,
        review_text -> Text,
        rev_data -> Date,
        rating -> Numeric,
        release_fk -> Int4,
    }
}

diesel::table! {
    user_friend_requests (friend_id) {
        friend_id -> Int4,
        usr -> Int4,
        friend -> Int4,
        request_status -> Int4,
    }
}

diesel::table! {
    user_info (id) {
        id -> Int4,
        user_fk -> Int4,
        avatar -> Varchar,
        status -> Varchar,
        register_date -> Date,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        login -> Varchar,
        password -> Varchar,
        email -> Varchar,
    }
}

diesel::joinable!(bookmark -> releases (release_fk));
diesel::joinable!(bookmark -> users (user_fk));
diesel::joinable!(episode -> releases (release_fk));
diesel::joinable!(history -> episode (episode));
diesel::joinable!(history -> users (user_fk));
diesel::joinable!(review -> releases (release_fk));
diesel::joinable!(review -> users (user_fk));
diesel::joinable!(user_info -> users (user_fk));

diesel::allow_tables_to_appear_in_same_query!(
    bookmark,
    episode,
    history,
    releases,
    review,
    user_friend_requests,
    user_info,
    users,
);
