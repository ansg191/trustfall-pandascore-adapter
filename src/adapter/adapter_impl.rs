use std::sync::Arc;

use pandascore::ClientTransport;
use trustfall::{
    provider::{
        resolve_coercion_using_schema, AsVertex, ContextIterator, ContextOutcomeIterator,
        EdgeParameters, VertexIterator,
    },
    FieldValue,
};

use crate::adapter::{Adapter, Vertex};

impl<'a, T: ClientTransport + 'a> trustfall::provider::BasicAdapter<'a> for Adapter<T> {
    type Vertex = Vertex;

    fn resolve_starting_vertices(
        &self,
        edge_name: &str,
        parameters: &EdgeParameters,
    ) -> VertexIterator<'a, Self::Vertex> {
        match edge_name {
            "Leagues" => {
                let game = parameters
                    .get("game")
                    .expect("failed to find parameter 'game' when resolving 'Leagues' starting vertices")
                    .as_str();
                super::entrypoints::leagues(Arc::clone(&self.0), game)
            }
            _ => unreachable!(
                "attempted to resolve starting vertices for unexpected edge name: {edge_name}"
            ),
        }
    }

    fn resolve_property<V: AsVertex<Self::Vertex> + 'a>(
        &self,
        contexts: ContextIterator<'a, V>,
        type_name: &str,
        property_name: &str,
    ) -> ContextOutcomeIterator<'a, V, FieldValue> {
        match type_name {
            "League" => super::properties::resolve_league_property(contexts, property_name),
            "Series" => super::properties::resolve_series_property(contexts, property_name),
            "Tournament" => super::properties::resolve_tournament_property(contexts, property_name),
            "Match" => super::properties::resolve_match_property(contexts, property_name),
            "Team" => super::properties::resolve_team_property(contexts, property_name),
            "Player" => super::properties::resolve_player_property(contexts, property_name),
            "VideoGame" => super::properties::resolve_video_game_property(contexts, property_name),
            "Winner" => super::properties::resolve_winner_property(contexts, property_name),
            "WinnerTeam" => {
                super::properties::resolve_winner_team_property(contexts, property_name)
            }
            "WinnerPlayer" => {
                super::properties::resolve_winner_player_property(contexts, property_name)
            }
            _ => {
                unreachable!(
                    "attempted to read property '{property_name}' on unexpected type: {type_name}"
                )
            }
        }
    }

    fn resolve_neighbors<V: AsVertex<Self::Vertex> + 'a>(
        &self,
        contexts: ContextIterator<'a, V>,
        type_name: &str,
        edge_name: &str,
        parameters: &EdgeParameters,
    ) -> ContextOutcomeIterator<'a, V, VertexIterator<'a, Self::Vertex>> {
        match type_name {
            "League" => super::edges::resolve_league_edge(
                Arc::clone(&self.0),
                contexts,
                edge_name,
                parameters,
            ),
            "Series" => super::edges::resolve_series_edge(
                Arc::clone(&self.0),
                contexts,
                edge_name,
                parameters,
            ),
            "Tournament" => super::edges::resolve_tournament_edge(
                Arc::clone(&self.0),
                contexts,
                edge_name,
                parameters,
            ),
            "Team" => super::edges::resolve_team_edge(
                Arc::clone(&self.0),
                contexts,
                edge_name,
                parameters,
            ),
            "Match" => super::edges::resolve_match_edge(
                Arc::clone(&self.0),
                contexts,
                edge_name,
                parameters,
            ),
            "WinnerTeam" => super::edges::resolve_winner_team_edge(
                Arc::clone(&self.0),
                contexts,
                edge_name,
                parameters,
            ),
            "WinnerPlayer" => super::edges::resolve_winner_player_edge(
                Arc::clone(&self.0),
                contexts,
                edge_name,
                parameters,
            ),
            _ => {
                unreachable!(
                    "attempted to resolve edge '{edge_name}' on unexpected type: {type_name}"
                )
            }
        }
    }

    fn resolve_coercion<V: AsVertex<Self::Vertex> + 'a>(
        &self,
        contexts: ContextIterator<'a, V>,
        _type_name: &str,
        coerce_to_type: &str,
    ) -> ContextOutcomeIterator<'a, V, bool> {
        resolve_coercion_using_schema(contexts, Self::schema(), coerce_to_type)
    }
}
