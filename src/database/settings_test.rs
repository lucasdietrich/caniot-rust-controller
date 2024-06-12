use std::fmt::Debug;

use crate::{
    database::{db, SettingsError, SettingsStore},
    settings::SettingTrait,
};

const TEST_KEY_PREFIX: &str = "test";

#[test]
pub fn test_settings() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        async fn test_setting_type<T: SettingTrait + Debug>(
            store: &SettingsStore<'_>,
            value: T,
            delete: bool,
        ) {
            let key = format!("{}::{}", TEST_KEY_PREFIX, T::type_name());

            store.set(&key, &value).await.unwrap();

            let string = store.read::<T>(&key).await;
            assert!(string.is_ok());
            let string = string.unwrap();
            assert_eq!(string, value);

            if delete {
                store.delete(&key).await.unwrap();
            }
        }

        let pool = db::Database::new("postgres://caniot:caniot@localhost/caniot")
            .await
            .unwrap();
        let store = pool.get_settings_store();

        let delete_immediate = false;

        test_setting_type(&store, String::from("test_value"), delete_immediate).await;
        test_setting_type(&store, true, delete_immediate).await;
        test_setting_type(&store, false, delete_immediate).await;
        test_setting_type(&store, 63_i64, delete_immediate).await;
        test_setting_type(&store, chrono::Utc::now(), delete_immediate).await;

        if !delete_immediate {
            store.delete_all().await.unwrap();
        }
    });
}
