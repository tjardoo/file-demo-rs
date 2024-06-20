use crate::{
    user_filter::{UserFilter, UserFilterInput},
    models::user::User,
};

pub struct NameFilter;

impl UserFilter for NameFilter {
    fn apply(&self, users: Vec<User>, input: UserFilterInput) -> Vec<User> {
        users
            .into_iter()
            .filter(|user| {
                let name = input.name.as_ref().unwrap();

                user.first_name.contains(name) || user.last_name.contains(name)
            })
            .collect()
    }

    fn should_apply(&self, input: &UserFilterInput) -> bool {
        input.name.is_some()
    }
}
