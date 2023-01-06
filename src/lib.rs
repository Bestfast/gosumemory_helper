use json::JsonValue;

#[derive(Clone, PartialEq, Default)]
pub struct Metadata {
    pub title: String,
    pub diff: String,
    pub artist: String,
    pub mapper: String,
    _private: (),
}

impl Metadata {
    pub fn new(msg: &JsonValue) -> Result<Metadata, &'static str> {
        // metadata
        let metadata_json = &msg["menu"]["bm"]["metadata"];
        Ok(Metadata {
            title: metadata_json["title"].to_string(),
            diff: metadata_json["difficulty"].to_string(),
            artist: metadata_json["artist"].to_string(),
            mapper: metadata_json["mapper"].to_string(),
            _private: ()
        })
    }
}

#[derive(Clone, PartialEq, Default)]
pub struct Stats {
    od: f32,
    hp: f32,
    sr: f32,
    bpmmin: f32,
    bpmmax: f32,
    _private: (),
}

impl Stats {
    pub fn new(msg: &JsonValue) -> Result<Stats, &'static str> {
        // stats
        let stats_json = &msg["menu"]["bm"]["stats"];
        Ok(Stats {
            od: stats_json["memoryOD"].as_f32().unwrap(),
            hp: stats_json["memoryHP"].as_f32().unwrap(),
            sr: stats_json["fullSR"].as_f32().unwrap(),
            bpmmin: stats_json["BPM"]["min"].as_f32().unwrap(),
            bpmmax: stats_json["BPM"]["max"].as_f32().unwrap(),
            _private: (),
        })
    }
}

#[derive(Clone, PartialEq, Default)]
pub struct Path {
    folder: String,
    file: String,
    bg: String,
    audio: String,
    _private: (),
}

impl Path {
    pub fn new(msg: &JsonValue) -> Result<Path, &'static str> {
        // path
        let path_json = &msg["menu"]["bm"]["path"];
        Ok(Path {
            folder: path_json["folder"].to_string(),
            file: path_json["file"].to_string(),
            bg: path_json["bg"].to_string(),
            audio: path_json["audio"].to_string(),
            _private: (),
        })
    }
}

#[derive(Clone, PartialEq, Default)]
pub struct OsuMap {
    pub metadata: Metadata,
    pub stats: Stats,
    pub path: Path,
    _private: (),
}

impl OsuMap {
    pub fn new(msg: &JsonValue) -> Result<OsuMap, &'static str> {
        Ok(OsuMap {
            metadata: Metadata::new(msg).unwrap(),
            stats: Stats::new(msg).unwrap(),
            path: Path::new(msg).unwrap(),
            _private: (),
        })
    }
}
