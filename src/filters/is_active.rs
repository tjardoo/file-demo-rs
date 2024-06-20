use crate::{
    models::user::User,
    user_filter::{UserFilter, UserFilterInput},
};

pub struct IsActiveFilter;

impl UserFilter for IsActiveFilter {
    fn apply(&self, users: Vec<User>, input: UserFilterInput) -> Vec<User> {
        let is_active = input.is_active.map_or(0, |value| value as i32);

        users
            .into_iter()
            .filter(|user| user.is_active == is_active)
            .collect()
    }

    fn should_apply(&self, input: &UserFilterInput) -> bool {
        input.is_active.is_some()
    }
}
