pub mod query;
pub mod relay;

use async_graphql::{EmptyMutation, EmptySubscription};

use self::query::Query;

pub type Schema = async_graphql::Schema<Query, EmptyMutation, EmptySubscription>;
