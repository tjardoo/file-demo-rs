use crate::{
    filters::{age::AgeFilter, is_active::IsActiveFilter, name::NameFilter},
    models::user::User,
};

pub trait UserFilter {
    fn apply(&self, users: Vec<User>, input: UserFilterInput) -> Vec<User>;

    fn should_apply(&self, input: &UserFilterInput) -> bool;
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct UserFilterInput {
    pub name: Option<String>,
    pub min_age: Option<u32>,
    pub max_age: Option<u32>,
    pub is_active: Option<bool>,
}

pub fn filter(users: Vec<User>, input: UserFilterInput) -> Vec<User> {
    let filters: Vec<Box<dyn UserFilter>> = vec![
        Box::new(AgeFilter),
        Box::new(NameFilter),
        Box::new(IsActiveFilter),
    ];

    filters
        .iter()
        .filter(|filter| filter.should_apply(&input))
        .fold(users, |users, filter| filter.apply(users, input.clone()))
}
