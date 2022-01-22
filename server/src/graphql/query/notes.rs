use async_graphql::{Context, Object};
use std::sync::Arc;

use crate::graphql::relay;
use crate::models::Note;
use crate::services::Services;

#[derive(Default)]
pub struct NotesQuery;

#[Object]
impl NotesQuery {
    async fn notes(
        &self,
        ctx: &Context<'_>,
        slug: Option<String>,
        after: Option<String>,
        before: Option<String>,
        first: Option<i32>,
        last: Option<i32>,
    ) -> relay::ConnectionResult<Note> {
        let services = ctx.data::<Arc<Services>>().unwrap();

        if let Some(slug) = slug {
            let result = services.notes_service.find_by_slug(&slug).await?;

            if let Some(note) = result {
                return relay::query(
                    vec![note].into_iter(),
                    relay::Params::new(after, before, first, last),
                    10,
                )
                .await;
            }

            return relay::query(
                Vec::default().into_iter(),
                relay::Params::new(after, before, first, last),
                10,
            )
            .await;
        }

        let notes = services.notes_service.list().await?;

        relay::query(
            notes.into_iter(),
            relay::Params::new(after, before, first, last),
            10,
        )
        .await
    }
}
