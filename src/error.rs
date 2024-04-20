use bevy::ecs::query::QueryEntityError;
use bevy::ecs::system::In;
use std::result::Result;

pub fn handle_query_entity_errors(In(result): In<Result<(), QueryEntityError>>) {
    if let Err(e) = result {
        eprintln!("Query entity error occurred: {}", e);
    }
}
