// fn type_of<T>(_: T) -> &'static str {
//     std::any::type_name::<T>()
// }

pub trait ValueUtils {
    fn has<T: serde_json::value::Index>(&self, key: T) -> bool;
    fn validate<T: serde_json::value::Index>(&self, fields: Vec<T>) -> Vec<T>;
    fn validate_or_die(&self, fields: &[&'static str]);
    fn only<T: serde_json::value::Index>(&self, fields: &[T]) -> serde_json::Value;
    fn only_or_die(&self, fields: &[&'static str]) -> serde_json::Value;
}

impl ValueUtils for serde_json::Value {
    fn has<T: serde_json::value::Index>(&self, key: T) -> bool {
        match self.get(key) {
            Some(_) => true,
            None => false,
        }
    }

    fn validate<T: serde_json::value::Index>(&self, fields: Vec<T>) -> Vec<T> {
        return fields.into_iter().filter(|key| !self.has(key)).collect();
    }

    fn validate_or_die(&self, fields: &[&'static str]) {
        for key in fields {
            let _ = self.get(key).expect(format!("Missing {}", key).as_str());
        }
    }

    fn only<T: serde_json::value::Index>(&self, fields: &[T]) -> serde_json::Value {
        let mut data = serde_json::json!({});
        for key in fields {
            let value = self.get(key);
            if value.is_some() {
                data[key] = value.unwrap().clone();
            }
        }
        data
    }

    fn only_or_die(&self, fields: &[&'static str]) -> serde_json::Value {
        let mut data = serde_json::json!({});
        for key in fields {
            data[key] = self.get(key).expect(format!("Missing {}", key).as_str()).clone();
        }
        data
    }
}

macro_rules! require {
    ($arg:expr, $fields:expr) => {
        let _missign_fields = $arg.validate($fields);
        if _missign_fields.len() != 0 {
            return Err(Error::MissingRequiredFields(_missign_fields));
        }
    };
}