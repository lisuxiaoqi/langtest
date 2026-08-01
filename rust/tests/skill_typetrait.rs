use std::marker::PhantomData;

#[test]
fn test_config() {
    use std::{fmt::Display, marker::PhantomData};
    trait Storage<K> {
        fn save(key: K, value: &str);
    }

    struct MemoryStorage;
    struct DatabaseStorage;

    impl<K: Display> Storage<K> for MemoryStorage {
        fn save(key: K, value: &str) {
            println!("mem save, key:{}, value:{}", key, value);
        }
    }
    impl<K: Display> Storage<K> for DatabaseStorage {
        fn save(key: K, value: &str) {
            println!("db save, key:{}, value:{}", key, value);
        }
    }

    trait Config {
        type UserId;
        type Storage: Storage<Self::UserId>;
    }

    struct UserService<T: Config>(PhantomData<T>);
    impl<T: Config> UserService<T> {
        fn save_user(key: T::UserId, value: &str) {
            T::Storage::save(key, value);
        }
    }

    struct DevConfig;
    struct ProdConfig;

    impl Config for DevConfig {
        type UserId = u32;
        type Storage = MemoryStorage;
    }
    impl Config for ProdConfig {
        type UserId = String;
        type Storage = DatabaseStorage;
    }
    UserService::<DevConfig>::save_user(123u32, "Alice");
    UserService::<ProdConfig>::save_user(String::from("user-001"), "Bob");
}

#[test]
fn test_const() {
    trait Config {
        const MAX_USERS: usize;
        type UserId;
    }

    struct UserRegistry<T: Config> {
        users: Vec<T::UserId>,
    }

    impl<T: Config> UserRegistry<T> {
        fn is_full(&self) -> bool {
            self.users.len() >= T::MAX_USERS
        }
    }

    struct DevConfig;
    struct ProdConfig;
    impl Config for DevConfig {
        const MAX_USERS: usize = 2;

        type UserId = u32;
    }
    impl Config for ProdConfig {
        const MAX_USERS: usize = 1000;

        type UserId = String;
    }

    let registry = UserRegistry::<DevConfig> { users: vec![1, 2] };

    assert!(registry.is_full());
}

#[test]
fn test_recur() {
    #[derive(Debug)]
    enum UserEvent {
        UserCreated,
    }

    //uni config
    trait Config {
        type Event: From<UserEvent>;
    }

    //app spec config
    #[derive(Debug)]
    enum AppEvent {
        User(UserEvent),
    }

    impl From<UserEvent> for AppEvent {
        fn from(value: UserEvent) -> Self {
            Self::User(value)
        }
    }

    struct AppConfig;
    impl Config for AppConfig {
        type Event = AppEvent;
    }

    //user service
    struct UserService<T: Config>(PhantomData<T>);
    impl<T: Config> UserService<T> {
        fn emit_created() -> T::Event {
            UserEvent::UserCreated.into()
        }
    }

    UserService::<AppConfig>::emit_created();
}
