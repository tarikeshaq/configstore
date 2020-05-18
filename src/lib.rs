use platform_dirs::AppDirs;
/// Expose so that consumer can determine the type of the application;
pub use platform_dirs::AppUI;
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::BufReader;
use std::path::PathBuf;

pub struct Configstore {
    prefix_dir: PathBuf,
}

impl Configstore {
    pub fn new(app_name: &str, app_ui: AppUI) -> Result<Self, Box<dyn std::error::Error>> {
        let prefix_dir = match AppDirs::new(Some(&app_name), app_ui) {
            Some(dir) => dir.config_dir,
            None => return Err("Unable to find config directory".into()),
        };
        std::fs::create_dir_all(prefix_dir.clone())?;

        Ok(Configstore { prefix_dir })
    }

    pub fn set<T>(&self, key: &str, value: T) -> Result<(), Box<dyn std::error::Error>>
    where
        T: Serialize + for<'de> Deserialize<'de>,
    {
        let mut file_name = String::from(key);
        file_name.push_str(".json");
        let config_path = self.prefix_dir.join(&file_name);
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(config_path)?;
        serde_json::to_writer(&file, &value)?;
        Ok(())
    }

    pub fn get<T>(&self, key: &str) -> Result<T, Box<dyn std::error::Error>>
    where
        T: Serialize + for<'de> Deserialize<'de>,
    {
        let mut file_name = String::from(key);
        file_name.push_str(".json");
        let config_path = self.prefix_dir.join(&file_name);
        let file = std::fs::File::open(config_path)?;
        let buff_reader = BufReader::new(file);
        let ret: T = serde_json::from_reader(buff_reader)?;
        Ok(ret)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_derive::*;
    #[derive(Deserialize, Serialize, Eq, PartialEq, Debug, Clone)]
    struct TestStruct {
        str_test: String,
        num: i64,
    }
    #[test]
    fn test_struct() {
        let config_store = Configstore::new("tests", AppUI::CommandLine).unwrap();
        let test_struct = TestStruct {
            str_test: "Hello World".to_string(),
            num: 1000,
        };
        config_store.set("test1", test_struct.clone()).unwrap();
        let other_struct: TestStruct = config_store.get("test1").unwrap();
        assert_eq!(test_struct, other_struct);
    }

    #[test]
    fn test_string() {
        let config_store = Configstore::new("tests", AppUI::CommandLine).unwrap();
        config_store.set("test2", String::from("World")).unwrap();
        let out: String = config_store.get("test2").unwrap();
        assert_eq!(out, "World".to_string());
    }

    #[test]
    fn reset_same_type() {
        let config_store = Configstore::new("tests", AppUI::CommandLine).unwrap();
        let test_struct = TestStruct {
            str_test: "Hello World".to_string(),
            num: 1000,
        };
        config_store.set("test3", test_struct.clone()).unwrap();
        let other_struct: TestStruct = config_store.get("test3").unwrap();
        assert_eq!(test_struct, other_struct);
        let replacement_struct = TestStruct {
            str_test: "Goodbye World".to_string(),
            num: 4242,
        };
        config_store
            .set("test3", replacement_struct.clone())
            .unwrap();
        let out: TestStruct = config_store.get("test3").unwrap();
        assert_eq!(replacement_struct, out);
    }

    #[test]
    fn test_vector() {
        let config_store = Configstore::new("tests", AppUI::CommandLine).unwrap();
        let test_struct_1 = TestStruct {
            str_test: "Hello World".to_string(),
            num: 1000,
        };
        let test_struct_2 = TestStruct {
            str_test: "Goodbye world".to_string(),
            num: 4524,
        };
        let test_vec = vec![test_struct_1, test_struct_2];
        config_store.set("test4", test_vec.clone()).unwrap();
        let out: Vec<TestStruct> = config_store.get("test4").unwrap();
        assert_eq!(out.len(), test_vec.len());
        for (i, val) in out.iter().enumerate() {
            assert_eq!(test_vec[i], *val);
        }
    }
}
