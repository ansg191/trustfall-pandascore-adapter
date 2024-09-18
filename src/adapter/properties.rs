use pandascore::model::Winner;
use trustfall::{
    provider::{AsVertex, ContextIterator, ContextOutcomeIterator},
    FieldValue,
};

use super::vertex::Vertex;
use crate::field_value::IntoFieldValue;

macro_rules! impl_properties {
    (
        $contexts:ident,
        $prop_name:ident,
        $ty:ident,
        $($f:ident $(=> $f2:ident)?),* $(,)?
    ) => {
        match $prop_name {
            $(
                stringify!($f) => Box::new($contexts.map(|ctx| {
                    let value = ctx.active_vertex().map(|v| match v {
                        Vertex::$ty(v) => impl_properties!(@internal v, $f $(=> $f2)?),
                        _ => unreachable!("expected active vertex to be '{}'", stringify!($ty)),
                    });
                    (ctx, value.into())
                })),
            )*
            _ => {
                unreachable!("attempted to read unexpected property '{}' on type '{}'", $prop_name, stringify!($ty))
            }
        }
    };

    (@internal $v:ident, $f:ident) => {
        $v.$f.clone().into_field_value()
    };
    (@internal $v:ident, $f:ident => $f2:ident) => {
        $v.$f2.clone().into_field_value()
    };
}

pub(super) fn resolve_league_property<'a, V: AsVertex<Vertex> + 'a>(
    contexts: ContextIterator<'a, V>,
    property_name: &str,
) -> ContextOutcomeIterator<'a, V, FieldValue> {
    impl_properties! {
        contexts,
        property_name,
        League,
        id,
        image_url,
        modified_at,
        name,
        slug,
        url
    }
}

pub(super) fn resolve_series_property<'a, V: AsVertex<Vertex> + 'a>(
    contexts: ContextIterator<'a, V>,
    property_name: &str,
) -> ContextOutcomeIterator<'a, V, FieldValue> {
    impl_properties! {
        contexts,
        property_name,
        Series,
        id,
        modified_at,
        begin_at,
        end_at,
        full_name,
        name,
        season,
        slug,
        year
    }
}

pub(super) fn resolve_tournament_property<'a, V: AsVertex<Vertex> + 'a>(
    contexts: ContextIterator<'a, V>,
    property_name: &str,
) -> ContextOutcomeIterator<'a, V, FieldValue> {
    impl_properties! {
        contexts,
        property_name,
        Tournament,
        id,
        modified_at,
        begin_at,
        end_at,
        detailed_stats,
        has_bracket,
        live_supported,
        name,
        prize_pool,
        slug,
        tier,
    }
}

pub(super) fn resolve_match_property<'a, V: AsVertex<Vertex> + 'a>(
    contexts: ContextIterator<'a, V>,
    property_name: &str,
) -> ContextOutcomeIterator<'a, V, FieldValue> {
    impl_properties! {
        contexts,
        property_name,
        Match,
        id,
        tournament_id,
        series_id => serie_id,
        league_id,
        modified_at,
        begin_at,
        end_at,
        original_scheduled_at,
        rescheduled,
        scheduled_at,
        detailed_stats,
        draw,
        forfeit,
        game_advantage,
        match_type,
        number_of_games,
        match_status => status,
        name,
        slug,
    }
}

pub(super) fn resolve_team_property<'a, V: AsVertex<Vertex> + 'a>(
    contexts: ContextIterator<'a, V>,
    property_name: &str,
) -> ContextOutcomeIterator<'a, V, FieldValue> {
    impl_properties! {
        contexts,
        property_name,
        Team,
        id,
        acronym,
        image_url,
        location,
        modified_at,
        name,
        slug,
    }
}

pub(super) fn resolve_player_property<'a, V: AsVertex<Vertex> + 'a>(
    contexts: ContextIterator<'a, V>,
    property_name: &str,
) -> ContextOutcomeIterator<'a, V, FieldValue> {
    impl_properties! {
        contexts,
        property_name,
        Player,
        id,
        age,
        birthday,
        first_name,
        image_url,
        last_name,
        modified_at,
        name,
        nationality,
        role,
        slug,
    }
}

pub(super) fn resolve_video_game_property<'a, V: AsVertex<Vertex> + 'a>(
    contexts: ContextIterator<'a, V>,
    property_name: &str,
) -> ContextOutcomeIterator<'a, V, FieldValue> {
    impl_properties! {
        contexts,
        property_name,
        VideoGame,
        id,
        name,
        slug,
        current_version
    }
}

pub(super) fn resolve_winner_property<'a, V: AsVertex<Vertex> + 'a>(
    contexts: ContextIterator<'a, V>,
    property_name: &str,
) -> ContextOutcomeIterator<'a, V, FieldValue> {
    match property_name {
        "id" => Box::new(contexts.map(|ctx| {
            let value = ctx.active_vertex().map(|v| match v {
                Vertex::Winner(v) => match v {
                    Winner::Team { id, .. } => id.into_field_value(),
                    Winner::Player { id, .. } => id.into_field_value(),
                    _ => unreachable!("unexpected winner variant"),
                },
                _ => unreachable!("expected active vertex to be '{}'", stringify!(Winner)),
            });
            (ctx, value.into())
        })),
        _ => {
            unreachable!(
                "attempted to read unexpected property '{}' on type '{}'",
                property_name,
                stringify!(Winner)
            )
        }
    }
}

pub(super) fn resolve_winner_team_property<'a, V: AsVertex<Vertex> + 'a>(
    contexts: ContextIterator<'a, V>,
    property_name: &str,
) -> ContextOutcomeIterator<'a, V, FieldValue> {
    match property_name {
        "id" => Box::new(contexts.map(|ctx| {
            let value = ctx
                .active_vertex()
                .and_then(|v| v.as_winner_team().map(|(id, _)| id.into_field_value()));
            (ctx, value.into())
        })),
        _ => {
            unreachable!(
                "attempted to read unexpected property '{}' on type '{}'",
                property_name,
                stringify!(Winner)
            )
        }
    }
}

pub(super) fn resolve_winner_player_property<'a, V: AsVertex<Vertex> + 'a>(
    contexts: ContextIterator<'a, V>,
    property_name: &str,
) -> ContextOutcomeIterator<'a, V, FieldValue> {
    match property_name {
        "id" => Box::new(contexts.map(|ctx| {
            let value = ctx
                .active_vertex()
                .and_then(|v| v.as_winner_player().map(|(id, _)| id.into_field_value()));
            (ctx, value.into())
        })),
        _ => {
            unreachable!(
                "attempted to read unexpected property '{}' on type '{}'",
                property_name,
                stringify!(Winner)
            )
        }
    }
}
