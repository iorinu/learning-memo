use rusqlite::{self, Connection, OptionalExtension};

pub(crate) fn open_url(id: i64, list: &Connection) -> Result<(), Box<dyn std::error::Error>> {
    let mut stmt = list.prepare("SELECT url FROM learning_list WHERE id = ?")?;
    let url = stmt
        .query_row([id], |row| row.get::<_, String>(0))
        .optional()?;

    match url {
        Some(u) => {
            let _ = open::that_detached(&u);
        }
        None => {
            println!("none id!")
        }
    }

    Ok(())
}
