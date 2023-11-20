use std::collections::HashMap;

pub struct Context {
    pub name: String,
    pub friends: HashMap<String, i32>,
}

#[inline]
pub fn get_name_with_string(ctx: &Context) -> &str {
    &ctx.name.as_str()
}

#[inline]
pub fn get_friend_id(ctx: &Context, friend_name: &str) -> Option<i32> {
    ctx.friends.get(friend_name).cloned()
}
