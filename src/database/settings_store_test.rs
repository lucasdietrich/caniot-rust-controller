use std::fmt::Debug;

use crate::database::{db, settings_types::SettingTrait, DatabaseConfig, SettingsStore};

const TEST_KEY_PREFIX: &str = "test";

#[tokio::test]
pub async fn test_settings() {
    async fn test_setting_type<T: SettingTrait + Debug>(
        store: &SettingsStore<'_>,
        value: T,
        delete: bool,
    ) {
        let key = format!("{}::{}", TEST_KEY_PREFIX, T::type_name());

        store.write(&key, &value).await.unwrap();

        let string = store.read::<T>(&key).await;
        assert!(string.is_ok());
        let string = string.unwrap();
        assert_eq!(string, value);

        if delete {
            store.delete(&key).await.unwrap();
        }
    }

    let config = DatabaseConfig {
        #[cfg(feature = "db-postgres")]
        connection_string: "postgres://caniot:caniot@localhost/caniot".to_string(),
        #[cfg(feature = "db-sqlite")]
        connection_string: "sqlite::memory:".to_string(),
        ..Default::default()
    };
    let storage = db::Storage::try_connect(&config).await.unwrap();
    storage.initialize_tables().await.unwrap();
    let store = storage.get_settings_store();

    let delete_immediate = false;

    test_setting_type(&store, String::from("test_value"), delete_immediate).await;
    test_setting_type(&store, true, delete_immediate).await;
    test_setting_type(&store, false, delete_immediate).await;
    test_setting_type(&store, 63_i64, delete_immediate).await;
    test_setting_type(&store, chrono::Utc::now(), delete_immediate).await;
    test_setting_type(
        &store,
        chrono::NaiveTime::from_hms_opt(8, 45, 30).unwrap(),
        delete_immediate,
    )
    .await;

    if !delete_immediate {
        store.delete_all().await.unwrap();
    }
}
