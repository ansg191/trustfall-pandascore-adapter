use compact_str::CompactString;
use pandascore::model::{
    matches::{MatchStatus, MatchType},
    tournament::Tier,
};
use time::format_description::well_known::{Iso8601, Rfc3339};
use trustfall::FieldValue;

pub trait IntoFieldValue {
    fn into_field_value(self) -> FieldValue;
}

macro_rules! impl_field_value {
    ( $($t:ty),* ) => {
        $(
            impl IntoFieldValue for $t {
                fn into_field_value(self) -> FieldValue {
                    FieldValue::from(self)
                }
            }
        )*
    };
}

impl_field_value! { i8, i16, i32, i64, u8, u16, u32, u64, String, bool, FieldValue }

impl<T: IntoFieldValue> IntoFieldValue for Option<T> {
    fn into_field_value(self) -> FieldValue {
        match self {
            None => FieldValue::Null,
            Some(v) => v.into_field_value(),
        }
    }
}

impl IntoFieldValue for CompactString {
    fn into_field_value(self) -> FieldValue {
        FieldValue::String(self.into())
    }
}

impl IntoFieldValue for time::OffsetDateTime {
    fn into_field_value(self) -> FieldValue {
        FieldValue::String(
            self.format(&Rfc3339)
                .expect("time format should succeed")
                .into(),
        )
    }
}

impl IntoFieldValue for time::Date {
    fn into_field_value(self) -> FieldValue {
        FieldValue::String(
            self.format(&Iso8601::DATE)
                .expect("date format should succeed")
                .into(),
        )
    }
}

impl IntoFieldValue for Tier {
    fn into_field_value(self) -> FieldValue {
        match self {
            Tier::Unranked => FieldValue::String("unranked".into()),
            Tier::D => FieldValue::String("d".into()),
            Tier::C => FieldValue::String("c".into()),
            Tier::B => FieldValue::String("b".into()),
            Tier::A => FieldValue::String("a".into()),
            Tier::S => FieldValue::String("s".into()),
        }
    }
}

impl IntoFieldValue for MatchType {
    fn into_field_value(self) -> FieldValue {
        match self {
            MatchType::AllGamesPlayed => FieldValue::String("all_games_played".into()),
            MatchType::BestOf => FieldValue::String("best_of".into()),
            MatchType::Custom => FieldValue::String("custom".into()),
            MatchType::FirstTo => FieldValue::String("first_to".into()),
            MatchType::OwBestOf => FieldValue::String("ow_best_of".into()),
            MatchType::RedBullHomeGround => FieldValue::String("red_bull_home_ground".into()),
            _ => FieldValue::String("unknown".into()),
        }
    }
}

impl IntoFieldValue for MatchStatus {
    fn into_field_value(self) -> FieldValue {
        match self {
            MatchStatus::Canceled => FieldValue::String("canceled".into()),
            MatchStatus::Finished => FieldValue::String("finished".into()),
            MatchStatus::NotStarted => FieldValue::String("not_started".into()),
            MatchStatus::Postponed => FieldValue::String("postponed".into()),
            MatchStatus::Running => FieldValue::String("running".into()),
            _ => FieldValue::String("unknown".into()),
        }
    }
}
