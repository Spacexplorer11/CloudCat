use quad_storage::STORAGE;

pub(crate) struct HighscoreManager;

impl HighscoreManager {
    pub(crate) fn load() -> u32 {
        let storage = STORAGE.lock().unwrap();
        let zero: String = "0".parse().unwrap();
        storage
            .get("cloudcat_highscore")
            .unwrap_or(zero)
            .parse::<u32>()
            .unwrap_or(0)
    }

    pub(crate) fn save(score: u32) {
        let mut storage = STORAGE.lock().unwrap();
        storage.set("cloudcat_highscore", &*score.to_string());
    }
}
