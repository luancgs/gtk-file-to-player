use sqlite::{Connection, Error, State};

#[derive(Debug)]
pub struct Song {
    pub id: i64,
    pub number: i64,
    pub title: String,
    pub lyrics: String,
}

pub fn connect_to_database(path: &str) -> Result<Connection, Error> {
    let connection = Connection::open(path)?;
    Ok(connection)
}

pub fn get_songs(connection: &Connection, text: &str) -> Result<Vec<Song>, Error> {
    let query = "
        SELECT * FROM song
        WHERE song.number LIKE :text || '%'
        OR song.title LIKE '%' || :text || '%'
        ORDER BY song.number ASC;
    ";
    let mut statement = connection.prepare(query)?;
    statement.bind((":text", text))?;

    let mut results = Vec::new();

    while let Ok(State::Row) = statement.next() {
        let song = Song {
            id: statement.read::<i64, _>("id")?,
            number: statement.read::<i64, _>("number")?,
            title: statement.read::<String, _>("title")?,
            lyrics: statement.read::<String, _>("lyrics")?,
        };

        results.push(song);
    }

    Ok(results)
}
