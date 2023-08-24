use std::path::PathBuf;
use serde::Deserialize;
use serde::Serialize;

// int_as_bool from Rust discord server
mod int_as_bool {
    use serde::de::{self, Visitor};
    use serde::Deserializer;
    use serde::Serializer;
    use std::fmt;

    #[allow(clippy::trivially_copy_pass_by_ref)]
    pub fn serialize<S>(val: &bool, ser: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        ser.serialize_u8((*val).into())
    }

    pub struct DeVisitor;

    impl<'de> Visitor<'de> for DeVisitor {
        type Value = bool;

        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str("an integer value 0 or 1 representing false or true")
        }

        fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            match value {
                0 => Ok(false),
                1 => Ok(true),
                _ => Err(E::custom(format!("out of range: {value}"))),
            }
        }

        fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            match value {
                0 => Ok(false),
                1 => Ok(true),
                _ => Err(E::custom(format!("out of range: {value}"))),
            }
        }
    }

    pub fn deserialize<'de, D>(de: D) -> Result<bool, D::Error>
    where
        D: Deserializer<'de>,
    {
        de.deserialize_u64(DeVisitor)
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Gosumemory {
    pub settings: Settings,
    pub menu: Menu,
    pub gameplay: Gameplay,
    pub results_screen: ResultsScreen,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    pub show_interface: bool,
    pub folders: Folders,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Folders {
    pub game: PathBuf,
    pub skin: PathBuf,
    pub songs: PathBuf,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Menu {
    pub main_menu: MainMenu,
    pub state: u8,
    pub game_mode: u8,
    #[serde(with = "int_as_bool")]
    pub is_chat_enabled: bool,
    pub bm: Bm,
    pub mods: Mods,
    pub pp: Pp,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MainMenu {
    pub bass_density: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Bm {
    pub time: Time,
    pub id: i32,
    pub set: i32,
    pub md5: String,
    pub ranked_status: u8,
    pub metadata: Metadata,
    pub stats: Stats,
    pub path: Path,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Time {
    pub first_obj: i32,
    pub current: i32,
    pub full: i32,
    pub mp3: i32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
    pub artist: String,
    pub artist_original: String,
    pub title: String,
    pub title_original: String,
    pub mapper: String,
    pub difficulty: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Stats {
    #[serde(rename = "AR")]
    pub ar: f32,
    #[serde(rename = "CS")]
    pub cs: f32,
    #[serde(rename = "OD")]
    pub od: f32,
    #[serde(rename = "HP")]
    pub hp: f32,
    #[serde(rename = "SR")]
    pub sr: f32,
    #[serde(rename = "BPM")]
    pub bpm: Bpm,
    pub max_combo: i32,
    #[serde(rename = "fullSR")]
    pub full_sr: f32,
    #[serde(rename = "memoryAR")]
    pub memory_ar: f32,
    #[serde(rename = "memoryCS")]
    pub memory_cs: f32,
    #[serde(rename = "memoryOD")]
    pub memory_od: f32,
    #[serde(rename = "memoryHP")]
    pub memory_hp: f32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Bpm {
    pub min: i64,
    pub max: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Path {
    pub full: PathBuf,
    pub folder: PathBuf,
    pub file: PathBuf,
    pub bg: PathBuf,
    pub audio: PathBuf,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mods {
    pub num: i32,
    pub str: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pp {
    #[serde(rename = "100")]
    pub n100: i32,
    #[serde(rename = "99")]
    pub n99: i32,
    #[serde(rename = "98")]
    pub n98: i32,
    #[serde(rename = "97")]
    pub n97: i32,
    #[serde(rename = "96")]
    pub n96: i32,
    #[serde(rename = "95")]
    pub n95: i32,
    pub strains: std::option::Option<Vec<f64>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Gameplay {
    pub game_mode: u8,
    pub name: std::option::Option<String>,
    pub score: i32,
    pub accuracy: f64,
    pub combo: Combo,
    pub hp: Hp,
    pub hits: Hits,
    pub pp: Pp2,
    pub key_overlay: KeyOverlay,
    pub leaderboard: Leaderboard,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Combo {
    pub current: i16,
    pub max: i16,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Hp {
    pub normal: f64,
    pub smooth: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Hits {
    #[serde(rename = "300")]
    pub n300: i16,
    pub geki: i16,
    #[serde(rename = "100")]
    pub n100: i16,
    pub katu: i16,
    #[serde(rename = "50")]
    pub n50: i16,
    #[serde(rename = "0")]
    pub n0: i16,
    pub slider_breaks: i16,
    pub grade: Grade,
    pub unstable_rate: f64,
    pub hit_error_array: std::option::Option<Vec<i32>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Grade {
    pub current: String,
    pub max_this_play: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pp2 {
    pub current: i32,
    pub fc: i32,
    pub max_this_play: i32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KeyOverlay {
    pub k1: KeyOverlayButton,
    pub k2: KeyOverlayButton,
    pub m1: KeyOverlayButton,
    pub m2: KeyOverlayButton,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KeyOverlayButton {
    pub is_pressed: bool,
    pub count: i32,
}


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Leaderboard {
    pub has_leaderboard: bool,
    pub is_visible: bool,
    pub ourplayer: Ourplayer,
    pub slots: std::option::Option<Vec<Ourplayer>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ourplayer {
    pub name: String,
    pub score: i32,
    pub combo: i16,
    pub max_combo: i16,
    pub mods: String,
    pub h300: i16,
    pub h100: i16,
    pub h50: i16,
    pub h0: i16,
    pub team: i32,
    pub position: i32,
    #[serde(with = "int_as_bool")]
    pub is_passing: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResultsScreen {
    pub name: String,
    pub score: i32,
    pub max_combo: i16,
    pub mods: Mods,
    #[serde(rename = "300")]
    pub n300: i16,
    pub geki: i16,
    #[serde(rename = "100")]
    pub n100: i16,
    pub katu: i16,
    #[serde(rename = "50")]
    pub n50: i16,
    #[serde(rename = "0")]
    pub n0: i16,
}
