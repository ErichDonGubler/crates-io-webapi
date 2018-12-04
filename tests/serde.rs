extern crate crates_io_webapi;

use crates_io_webapi::FullCrateDetails;

#[test]
fn test_de() {
    let _: FullCrateDetails =
        serde_json::from_str(include_str!("./sample_responses/inexact-serde-derive.json")).unwrap();
    let _: FullCrateDetails =
        serde_json::from_str(include_str!("./sample_responses/exact-adhesion.json")).unwrap();
    let _: FullCrateDetails =
        serde_json::from_str(include_str!("./sample_responses/exact-serde.json")).unwrap();
}
