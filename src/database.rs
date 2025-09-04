use sqlite::{Connection, State};
use std::path::Path;

#[derive(Debug, Clone)]
pub struct Song {
    pub number: i64,
    pub title: String,
    pub file: String,
}

pub struct Database {
    connection: Connection,
}

impl Database {
    pub fn new(path: &str) -> Result<Self, String> {
        if !Path::new(path).exists() {
            return Err(format!("Database file does not exist: {}", path));
        }

        let connection = Connection::open(path)
            .map_err(|e| format!("Failed to connect to database '{}': {}", path, e))?;

        let db = Database { connection };
        db.validate_schema()?;

        Ok(db)
    }

    fn validate_schema(&self) -> Result<(), String> {
        let query = "SELECT name FROM sqlite_master WHERE type='table' AND name='song';";
        let mut statement = self
            .connection
            .prepare(query)
            .map_err(|e| format!("Failed to prepare schema validation query: {}", e))?;

        match statement.next() {
            Ok(State::Row) => Ok(()),
            Ok(State::Done) => Err("Database does not contain a 'song' table".to_string()),
            Err(e) => Err(format!("Failed to validate database schema: {}", e)),
        }
    }

    pub fn search_songs(&self, search_text: &str) -> Result<Vec<Song>, String> {
        if search_text.trim().is_empty() {
            return Ok(Vec::new());
        }

        let query = "
            SELECT number, title, file FROM song
            WHERE CAST(number AS TEXT) LIKE ?1 || '%'
            OR title LIKE '%' || ?1 || '%'
            ORDER BY number ASC
            LIMIT 10;
        ";

        let mut statement = self
            .connection
            .prepare(query)
            .map_err(|e| format!("Failed to prepare search query: {}", e))?;

        statement
            .bind((1, search_text))
            .map_err(|e| format!("Failed to bind search parameter: {}", e))?;

        let mut results = Vec::new();

        while let Ok(State::Row) = statement.next() {
            let song = Song {
                number: statement
                    .read::<i64, _>("number")
                    .map_err(|e| format!("Failed to read song number: {}", e))?,
                title: statement
                    .read::<String, _>("title")
                    .map_err(|e| format!("Failed to read song title: {}", e))?,
                file: statement
                    .read::<String, _>("file")
                    .map_err(|e| format!("Failed to read song file: {}", e))?,
            };

            results.push(song);
        }

        Ok(results)
    }

    pub fn get_all_songs(&self) -> Result<Vec<Song>, String> {
        let query = "SELECT number, title, file FROM song ORDER BY number ASC;";

        let mut statement = self
            .connection
            .prepare(query)
            .map_err(|e| format!("Failed to prepare get all songs query: {}", e))?;

        let mut results = Vec::new();

        while let Ok(State::Row) = statement.next() {
            let song = Song {
                number: statement
                    .read::<i64, _>("number")
                    .map_err(|e| format!("Failed to read song number: {}", e))?,
                title: statement
                    .read::<String, _>("title")
                    .map_err(|e| format!("Failed to read song title: {}", e))?,
                file: statement
                    .read::<String, _>("file")
                    .map_err(|e| format!("Failed to read song file: {}", e))?,
            };

            results.push(song);
        }

        Ok(results)
    }

    pub fn get_song_by_number(&self, number: i64) -> Result<Option<Song>, String> {
        let query = "SELECT number, title, file FROM song WHERE number = ?1;";

        let mut statement = self
            .connection
            .prepare(query)
            .map_err(|e| format!("Failed to prepare get song query: {}", e))?;

        statement
            .bind((1, number))
            .map_err(|e| format!("Failed to bind song number parameter: {}", e))?;

        match statement.next() {
            Ok(State::Row) => {
                let song = Song {
                    number: statement
                        .read::<i64, _>("number")
                        .map_err(|e| format!("Failed to read song number: {}", e))?,
                    title: statement
                        .read::<String, _>("title")
                        .map_err(|e| format!("Failed to read song title: {}", e))?,
                    file: statement
                        .read::<String, _>("file")
                        .map_err(|e| format!("Failed to read song file: {}", e))?,
                };
                Ok(Some(song))
            }
            Ok(State::Done) => Ok(None),
            Err(e) => Err(format!("Failed to get song by number: {}", e)),
        }
    }

    pub fn count_songs(&self) -> Result<i64, String> {
        let query = "SELECT COUNT(*) as count FROM song;";

        let mut statement = self
            .connection
            .prepare(query)
            .map_err(|e| format!("Failed to prepare count query: {}", e))?;

        match statement.next() {
            Ok(State::Row) => {
                let count = statement
                    .read::<i64, _>("count")
                    .map_err(|e| format!("Failed to read song count: {}", e))?;
                Ok(count)
            }
            Ok(State::Done) => Ok(0),
            Err(e) => Err(format!("Failed to count songs: {}", e)),
        }
    }
}
