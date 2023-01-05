use json;
use tungstenite::Message;

struct Metadata {
    title: String,
    diff: String,
    artist: String,
    mapper: String,
}

struct Stats {
    od: f32,
    hp: f32,
    sr: f32,
    bpmmin: f32,
    bpmmax: f32,
}

struct Path {
    folder: String,
    file: String,
    bg: String,
    audio: String,
}

struct OsuMap {
    metadata: Metadata,
    stats: Stats,
    path: Path,
}

impl OsuMap {
    fn new(msg: Message) -> Result<OsuMap, &'static str> {
        let text = msg.to_text().unwrap();
        let text_json = json::parse(text).unwrap();

        // metadata
        let metadata_json = &text_json["menu"]["bm"]["metadata"];
        let metadata = Metadata {
            title: metadata_json["title"].to_string(),
            diff: metadata_json["difficulty"].to_string(),
            artist: metadata_json["artist"].to_string(),
            mapper: metadata_json["mapper"].to_string(),
        };

        // stats
        let stats_json = &text_json["menu"]["bm"]["stats"];
        let stats = Stats {
            od: stats_json["memoryOD"].as_f32().unwrap(),
            hp: stats_json["memoryHP"].as_f32().unwrap(),
            sr: stats_json["fullSR"].as_f32().unwrap(),
            bpmmin: stats_json["BPM"]["min"].as_f32().unwrap(),
            bpmmax: stats_json["BPM"]["max"].as_f32().unwrap(),
        };

        // path
        let path_json = &text_json["menu"]["bm"]["path"];
        let path = Path {
            folder: path_json["folder"].to_string(),
            file: path_json["file"].to_string(),
            bg: path_json["bg"].to_string(),
            audio: path_json["audio"].to_string(),
        };

        Ok(OsuMap {
            metadata,
            stats,
            path,
        })
    }
}