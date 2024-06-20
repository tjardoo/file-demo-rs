use crate::{
    database::{self},
    user_filter::{filter, UserFilterInput},
};

pub fn filter_users(input: UserFilterInput) {
    let users = database::users();

    let filtered_users = filter(users, input);

    for user in filtered_users {
        println!("{:?}", user);
    }
}
