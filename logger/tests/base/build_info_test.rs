#[cfg(test)]
pub mod build_info_test {
    use logger::build_info::RUST_SDK_BUILD_INFO;
    use std::collections::HashMap;

    #[test]
    fn test() {
        let map: HashMap<String, String> =
            serde_json::from_str(RUST_SDK_BUILD_INFO).expect("Invalid JSON");
        assert!(map.get("my_version").is_some());
        assert!(map.get("my_commit").is_some());
        assert!(map.get("my_build_time").is_some());
        assert!(map.get("my_target").is_some());
        assert_eq!(map.len(), 4);
    }
}
