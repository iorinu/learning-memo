use crate::structure::LearningList;
use std::collections::BTreeMap;
use std::path::PathBuf;

ust chrono::{Local, NaiveDate};
use rusqlite::{Connection, Result};

// DB保存先のパス
fn db_path() -> PathBuf {
    let mut path = dirs::data_dir().unwrap_or_else(|| PathBuf::from("."));
    path.push("lmemo");
    let _ = std::fs::create_dir_all(&path);
    path.push("learning_memo.db");
    path
}

pub fn create_sql() -> Result<Connection> {
    let conn = Connection::open(db_path())?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS learning_list (
            id    INTEGER PRIMARY KEY,
            url TEXT,
            title TEXT,
            memo TEXT,
            date TEXT,
            domain TEXT
        )",
        (),
    )?;
    Ok(conn)
}

pub fn insert_sql(list: &Connection, add_list: LearningList) -> Result<()> {
    list.execute(
        "INSERT INTO learning_list (url, title, memo, date, domain) VALUES (?1, ?2, ?3, ?4, ?5)",
        (
            add_list.url,
            add_list.title,
            add_list.memo,
            add_list.date,
            add_list.domain,
        ),
    )?;
    Ok(())
}

pub fn select_all_table(list: &Connection) -> Result<Vec<LearningList>> {
    let mut stmt = list.prepare("SELECT id, url, title, memo, date, domain FROM learning_list")?;
    let rows = stmt.query_map([], |row| {
        Ok(LearningList {
            id: row.get(0)?,
            url: row.get(1)?,
            title: row.get(2)?,
            memo: row.get(3)?,
            date: row.get(4)?,
            domain: row.get(5)?,
        })
    })?;

    let mut view_list = Vec::new();
    for item in rows {
        view_list.push(item?);
    }
    Ok(view_list)
}

pub fn view_table(list: &[LearningList]) -> Result<()> {
    let list_len = list.len();
    for i in 0..list_len {
        println!("{}", list[i].id);
        println!("{}", list[i].url);
        println!("{}", list[i].title);
        println!("{}", list[i].memo);
        println!("{}", list[i].date);
        println!("-------------------------------------------------");
    }
    Ok(())
}

pub fn recent_table(list: &Connection) -> Result<Vec<LearningList>> {
    let mut stmt = list.prepare(
        "SELECT id, url, title, memo, date, domain FROM learning_list 
        ORDER BY id DESC LIMIT 10",
    )?;
    let rows = stmt.query_map([], |row| {
        Ok(LearningList {
            id: row.get(0)?,
            url: row.get(1)?,
            title: row.get(2)?,
            memo: row.get(3)?,
            date: row.get(4)?,
            domain: row.get(5)?,
        })
    })?;

    let mut view_list = Vec::new();
    for item in rows {
        view_list.push(item?);
    }
    Ok(view_list)
}

pub fn daily_chart(list: &[LearningList]) -> Result<(), Box<dyn std::error::Error>> {
    if list.is_empty() {
        println!("データなし");
        return Ok(());
    }

    let mut counts: BTreeMap<NaiveDate, usize> = BTreeMap::new();
    for item in list {
        *counts.entry(item.date).or_insert(0) += 1;
    }

    let end = Local::now().date_naive();
    let start = end - chrono::Duration::days(30);

    let mut date = start;
    while date <= end {
        counts.entry(date).or_insert(0);
        date = date.succ_opt().unwrap();
    }

    let max = counts.values().max().copied().unwrap_or(1);
    for (date, count) in &counts {
        let bar_width = count * 40 / max;
        let bar = "⬛︎".repeat(bar_width);
        println!("{}, {}, {}", date, bar, count)
    }
    Ok(())
}

pub fn view_all_site(list: &Connection) -> Result<()> {
    let mut stmt = list.prepare(
        "SELECT domain, COUNT(*) FROM learning_list
        GROUP BY domain ORDER BY COUNT(*) DESC",
    )?;

    let rows = stmt.query_map([], |row| {
        let domain: String = row.get(0)?;
        let count: i64 = row.get(1)?;
        Ok((domain, count))
    })?;

    let mut printed = false;
    for row in rows {
        let (domain, count) = row?;
        println!("{:20} {}", domain, count);
        printed = true;
    }

    if !printed {
        println!("データなし");
    }
    Ok(())
}

pub fn select_domain_table(
    list: &Connection,
    domain: &String,
) -> Result<Vec<LearningList>, Box<dyn std::error::Error>> {
    let mut stmt = list.prepare(
        "SELECT id, url, title, memo, date, domain FROM learning_list \
         WHERE domain = ?1 ORDER BY id DESC",
    )?;
    let rows = stmt.query_map([domain], |row| {
        Ok(LearningList {
            id: row.get(0)?,
            url: row.get(1)?,
            title: row.get(2)?,
            memo: row.get(3)?,
            date: row.get(4)?,
            domain: row.get(5)?,
        })
    })?;

    let mut list = Vec::new();
    for item in rows {
        list.push(item?);
    }
    Ok(list)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::structure::LearningList;
    use chrono::NaiveDate;
    use rusqlite::Connection;

    // テスト用のメモリ内DB
    fn test_conn() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute(
            "CREATE TABLE learning_list (
                id    INTEGER PRIMARY KEY,
                url   TEXT,
                title TEXT,
                memo  TEXT,
                date  TEXT,
                domain TEXT
            )",
            (),
        )
        .unwrap();
        conn
    }

    fn sample(n: i32, domain: &str) -> LearningList {
        LearningList {
            id: 0,
            url: format!("https://{}/page{}", domain, n),
            title: format!("title {}", n),
            memo: format!("memo {}", n),
            date: NaiveDate::from_ymd_opt(2026, 6, 28).unwrap(),
            domain: domain.to_string(),
        }
    }

    // 1件 INSERT して SELECT で取り出し、全フィールドが一致することを確認
    #[test]
    fn insert_then_select_all_roundtrip() {
        let conn = test_conn();
        insert_sql(&conn, sample(1, "example.com")).unwrap();

        let rows = select_all_table(&conn).unwrap();
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].url, "https://example.com/page1");
        assert_eq!(rows[0].title, "title 1");
        assert_eq!(rows[0].memo, "memo 1");
        assert_eq!(rows[0].domain, "example.com");
        assert_eq!(rows[0].date, NaiveDate::from_ymd_opt(2026, 6, 28).unwrap());
    }

    // 空DBに対する select_all_table は空 Vec を返すべき
    #[test]
    fn select_all_returns_empty_on_empty_db() {
        let conn = test_conn();
        let rows = select_all_table(&conn).unwrap();
        assert!(rows.is_empty());
    }

    // 15件入れて、LIMIT 10 と id降順が効いているかを確認
    #[test]
    fn recent_table_returns_at_most_10_in_desc_order() {
        let conn = test_conn();
        for i in 0..15 {
            insert_sql(&conn, sample(i, "example.com")).unwrap();
        }

        let rows = recent_table(&conn).unwrap();
        assert_eq!(rows.len(), 10, "LIMIT 10 が効いていない");

        // ids がすでに降順になっているか：自分自身をソートしたものと比較する
        let ids: Vec<i32> = rows.iter().map(|r| r.id).collect();
        let mut expected = ids.clone();
        expected.sort_by(|a, b| b.cmp(a)); // 降順ソート
        assert_eq!(ids, expected, "id 降順になっていない");
    }

    #[test]
    fn recent_table_empty_db() {
        let conn = test_conn();
        let rows = recent_table(&conn).unwrap();
        assert!(rows.is_empty());
    }

    // 3ドメイン分入れて、指定したドメインだけ返ることを確認
    #[test]
    fn select_domain_table_filters_by_domain() {
        let conn = test_conn();
        insert_sql(&conn, sample(1, "example.com")).unwrap();
        insert_sql(&conn, sample(2, "example.com")).unwrap();
        insert_sql(&conn, sample(3, "other.com")).unwrap();

        let rows = select_domain_table(&conn, &"example.com".to_string()).unwrap();
        assert_eq!(rows.len(), 2);
        assert!(rows.iter().all(|r| r.domain == "example.com"));
    }

    // マッチしないドメインを指定したら空 Vec
    #[test]
    fn select_domain_table_returns_empty_when_no_match() {
        let conn = test_conn();
        insert_sql(&conn, sample(1, "example.com")).unwrap();

        let rows = select_domain_table(&conn, &"unknown.com".to_string()).unwrap();
        assert!(rows.is_empty());
    }
}
