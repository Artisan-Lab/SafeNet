fn option_field_absent_owned() {
    #[derive(serde::Deserialize, Debug)]
    pub struct Person {
        pub name: String,
        pub middle_name: Option<String>,
        pub friends: Vec<String>,
    }
    let mut raw_json = r#"{"name":"bob","friends":[]}"#.to_string();
    let result: Result<Person, _> = crate::to_owned_value(unsafe { raw_json.as_bytes_mut() })
        .and_then(super::super::from_value);
    assert!(result.is_ok());
}

// https://github.com/simd-lite/simd-json/blob/21f7878505a472ab93077b18294888d73ce330b9/src/serde/value/owned/de.rs#L814