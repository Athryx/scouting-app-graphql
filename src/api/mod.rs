pub mod query;
pub mod mutation;
pub mod user;

use juniper::{RootNode, EmptySubscription};

use crate::db::Connection;

pub type ApiRoot = RootNode<'static, query::RootQuery, mutation::RootMutation, EmptySubscription<Connection>>;
