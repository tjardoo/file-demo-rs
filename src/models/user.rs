use chrono::NaiveDate;

#[derive(Debug)]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub date_of_birth: NaiveDate,
    pub is_active: i32,
    pub created_at: i64,
}
