use crates_io_webapi::get_crate;

#[test]
fn test_exact_matches() {
    for crate_name in &[
        "adhesion",
        "serde",
        "serde-derive",
    ] {
        match get_crate(crate_name) {
            Ok(Some(_)) => (),
            Ok(None) => panic!("no crate {:?} found", crate_name),
            Err(e) => panic!("error querying crate {:?}: {}", crate_name, e),
        }
    }
}

#[test]
fn test_missing_crate_returns_none() {
    // Use a name that definitely can't happen
    assert!(get_crate("@").unwrap().is_none());
}
