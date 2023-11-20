use protocol::{get_friend_id, get_name_with_string, Context};
use provider::{create_call_back, shout_one_or_two};

use std::collections::HashMap;
use std::sync::Arc;

fn main() {
    shout_one_or_two!(10~"bork");
    let func = create_call_back!(get_friends_id <= { String: "Felix" });

    let context = Context {
        name: "Felix".to_string(),
        friends: {
            let mut map: HashMap<String, i32> = HashMap::new();
            map.insert("Felix".to_string(), 10);
            map
        },
    };

    let res = (func)(&context).unwrap();
    println!("res is: {}", res);

    // let func = |ctx: &'_ Context| -> Option<&'_ i32> {
    //     let name = "felix";
    //     let res = get_friend_id(ctx, name);
    //     res
    // };
}

// fn some_func(ctx: &Context) -> Option<&i32> {
//     let name = "felix";
//     let res = get_friend_id(ctx, name);
//     res
// }
