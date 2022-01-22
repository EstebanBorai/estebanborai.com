table! {
    notes (id) {
        id -> Uuid,
        title -> Varchar,
        slug -> Varchar,
        description -> Varchar,
        categories -> Array<Text>,
        date -> Date,
        lang -> Varchar,
        sha -> Varchar,
        preview_image_url -> Varchar,
        download_url -> Varchar,
        content -> Text,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}
