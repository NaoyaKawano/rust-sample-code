use rusqlite::{Connection, Result};

// データベースファイル名
const DB_FILE: &str = "test.db";

// Person構造体
// データベースのテーブルに対応する構造体
#[derive(Debug)]
struct Person {
    id: i32,      // idは自動採番なので、挿入時には指定しない
    name: String, // 名前
    data: String, // 何らかのデータ
}

// main関数
fn main() -> Result<()> {
    println!("Hello, world!");

    // テーブルの作成
    create_table()?;

    // データの挿入
    // 挿入データはPerson構造体で指定
    let person: Person = Person {
        id: -1,
        name: "Taro".to_string(),
        data: "This is Taro's data.".to_string(),
    };
    insert_data(person)?;

    // データの取得
    // 確認用に取得したデータを表示している
    let person_vec = select_data()?;
    for person in person_vec {
        print!(
            "id: {}, name: {}, data: {} \n",
            person.id, person.name, person.data
        );
    }

    // 結果を返す
    Ok(())
}

// テーブルの作成
fn create_table() -> Result<()> {
    // データベースに接続
    let conn = Connection::open(DB_FILE)?;

    // テーブルの作成
    conn.execute(
        "CREATE TABLE IF NOT EXISTS person (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL,
                data BLOB
            )",
        [],
    )?;

    Ok(())
}

// データの挿入. 挿入データは引数で受け取る
fn insert_data(person: Person) -> Result<()> {
    // データベースに接続
    let conn = Connection::open(DB_FILE)?;

    // データの挿入
    conn.execute(
        "INSERT INTO person (name, data) VALUES (?1, ?2)",
        &[&person.name, &person.data],
    )?;

    Ok(())
}

// データの取得
fn select_data() -> Result<Vec<Person>, rusqlite::Error> {
    // データベースに接続
    let conn = Connection::open(DB_FILE)?;

    // データの取得
    let mut stmt = conn.prepare("SELECT id, name, data FROM person")?;
    let params = [];
    let person_iter = stmt.query_map(params, |row| {
        Ok(Person {
            id: row.get(0)?,
            name: row.get(1)?,
            data: row.get(2)?,
        })
    })?;

    // person_iterからデータを取り出して、vecに格納
    let mut person_vec: Vec<Person> = Vec::new();
    for person in person_iter {
        person_vec.push(person.unwrap());
    }

    // 結果を返す
    Ok(person_vec)
}
