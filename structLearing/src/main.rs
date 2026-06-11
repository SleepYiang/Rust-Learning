struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

//普通初始化结构体
fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    //结构体内容改变
    user1.email = String::from("anotheremail@example.com");
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

//使用函数初始化结构体
fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

struct User1 {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

//函数简化初始化结构体
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn main1() {
    let user1 = build_user(
        String::from("someone@example.com"),
        String::from("someusername123"),
    );
    //使用结构体更新语法创建新实例
    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };
    //使用结构体更新语法创建新实例..省略后面内容但是效果同上
    // let user3 = User {
    //     email: String::from("another@example.com"),
    //     ..user1
    // };
    
}


