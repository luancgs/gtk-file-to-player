use sqlite::{Connection, Error, State};

#[derive(Debug)]
pub struct Song {
    pub number: i64,
    pub title: String,
    pub file: String,
}

pub fn connect_to_database(path: &str) -> Connection {
    let connection = Connection::open(path);

    match connection {
        Ok(conn) => conn,
        Err(err) => {
            eprintln!("Failed to connect to database: {}", err);
            std::process::exit(1);
        }
    }
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
            number: statement.read::<i64, _>("number")?,
            title: statement.read::<String, _>("title")?,
            file: statement.read::<String, _>("file")?,
        };

        results.push(song);
    }

    Ok(results)
}
