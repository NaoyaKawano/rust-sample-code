// モデルの定義: Person
#[derive(Debug)]
pub struct Person {
    pub id: i32,      // idは自動採番なので、挿入時には指定しない
    pub name: String, // 名前
    pub data: String, // 何らかのデータ
}
