use chrono::Utc;

use crate::{
    models::user::User,
    user_filter::{UserFilter, UserFilterInput},
};

pub struct AgeFilter;

impl UserFilter for AgeFilter {
    fn apply(&self, users: Vec<User>, input: UserFilterInput) -> Vec<User> {
        let current_timestamp = Utc::now().timestamp();

        let min_age_timestamp = get_age_timestamp(current_timestamp, input.min_age);
        let max_age_timestamp = get_age_timestamp(current_timestamp, input.max_age);

        users
            .into_iter()
            .filter(|user| {
                let date_of_birth_timestamp = user
                    .date_of_birth
                    .and_hms_opt(0, 0, 0)
                    .unwrap()
                    .and_utc()
                    .timestamp();

                if let Some(min_age_timestamp) = min_age_timestamp {
                    if date_of_birth_timestamp > min_age_timestamp {
                        return false;
                    }
                }

                if let Some(max_age_timestamp) = max_age_timestamp {
                    if date_of_birth_timestamp < max_age_timestamp {
                        return false;
                    }
                }

                return true;
            })
            .collect()
    }

    fn should_apply(&self, input: &UserFilterInput) -> bool {
        input.min_age.is_some() || input.max_age.is_some()
    }
}

fn get_age_timestamp(current_timestamp: i64, age: Option<u32>) -> Option<i64> {
    if age.is_none() {
        return None;
    }

    Some(current_timestamp - (age.unwrap() as i64 * (365 * 24 * 60 * 60)))
}
