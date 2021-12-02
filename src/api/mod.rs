pub mod types;
pub mod query;
pub mod mutation;
pub mod user;
pub mod member;
pub mod team;

use juniper::{RootNode, EmptySubscription};

use crate::Context;

pub type ApiRoot = RootNode<'static, query::RootQuery, mutation::RootMutation, EmptySubscription<Context>>;
