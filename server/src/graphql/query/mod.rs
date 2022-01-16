mod notes;

use async_graphql::MergedObject;

use self::notes::NotesQuery;

#[derive(MergedObject, Default)]
pub struct Query(pub NotesQuery);
