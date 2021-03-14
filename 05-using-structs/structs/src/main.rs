#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        sign_in_count: 1,
        active: true,
    };
    println!("The first user has e-mail {}", user1.email);
    let user2 = build_user(
        String::from("another@example.com"),
        String::from("someusername234"),
    );
    println!("The second user has username {}", user2.username);
    println!("The whole struct of the second user is: {:#?}", user2);
    let user3 = User::new_user(
        String::from("example@example.com"),
        String::from("someusername345"),
    );
    println!("The whole struct of the third user is: {:?}", user3);
    println!("Is the third user is active? {}", user3.is_active());
}

fn build_user(email: String, username: String) -> User {
    User {
        email, // field init shorthand
        username,
        active: true,
        sign_in_count: 1,
    }
}

impl User {
    fn is_active(&self) -> bool {
        self.active
    }

    fn new_user(email: String, username: String) -> User {
        User {
            email,
            username,
            active: true,
            sign_in_count: 1,
        }
    }
}
