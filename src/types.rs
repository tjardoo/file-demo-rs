use chrono::NaiveDate;
use rusqlite::types::FromSql;

#[derive(Debug)]
pub struct Date(NaiveDate);

impl FromSql for Date {
    fn column_result(value: rusqlite::types::ValueRef) -> rusqlite::types::FromSqlResult<Self> {
        value
            .as_str()
            .and_then(|s| Ok(NaiveDate::parse_from_str(s, "%Y-%m-%d").unwrap().into()))
    }
}

impl From<NaiveDate> for Date {
    fn from(date: NaiveDate) -> Self {
        Date(date)
    }
}

impl Date {
    pub fn timestamp(&self) -> i64 {
        self.0.and_hms_opt(0, 0, 0).unwrap().and_utc().timestamp()
    }
}
