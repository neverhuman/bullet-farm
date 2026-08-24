use std::{collections::BTreeMap, fs, path::PathBuf};

use bullet_wire::{
    MAX_CANONICAL_DOCUMENT_BYTES, PolicyTemplateV1, canonical_json, decode_canonical,
    decode_canonical_value, hash_framed_bytes,
};

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|path| path.parent())
        .unwrap()
        .to_path_buf()
}

#[test]
fn hostile_fixture_files_fail_with_stable_reason_codes() {
    let expected = BTreeMap::from([
        ("bidi.json", "DIRECTIONAL_CONTROL_FORBIDDEN"),
        ("bom.json", "UTF8_BOM_FORBIDDEN"),
        ("crlf.json", "NON_CANONICAL_JSON"),
        ("duplicate-key.json", "DUPLICATE_JSON_KEY"),
        ("escaped-control.json", "CONTROL_CHARACTER_FORBIDDEN"),
        ("invalid-utf8.json", "INVALID_UTF8"),
        ("lf.json", "NON_CANONICAL_JSON"),
        ("non-nfc.json", "NON_NFC_STRING"),
        ("nul.json", "CONTROL_CHARACTER_FORBIDDEN"),
        ("raw-control.json", "INVALID_JSON"),
        ("zero-width.json", "ZERO_WIDTH_CHARACTER_FORBIDDEN"),
    ]);
    for (name, code) in expected {
        let bytes = fs::read(root().join("fixtures/hostile/cases").join(name)).unwrap();
        let error = decode_canonical_value(&bytes).unwrap_err();
        assert_eq!(error.code(), code, "fixture {name}");
    }
}

#[test]
fn overlong_and_unsafe_numeric_inputs_fail_before_use() {
    let overlong = vec![b' '; MAX_CANONICAL_DOCUMENT_BYTES + 1];
    assert_eq!(
        decode_canonical_value(&overlong).unwrap_err().code(),
        "DOCUMENT_TOO_LARGE"
    );
    assert_eq!(
        decode_canonical_value(b"9007199254740992")
            .unwrap_err()
            .code(),
        "UNSAFE_JSON_INTEGER"
    );
}

#[test]
fn strict_type_decode_rejects_unknown_fields() {
    let mut value = serde_json::from_slice::<serde_json::Value>(
        &fs::read(root().join("policy/v1alpha1/policy-template.json")).unwrap(),
    )
    .unwrap();
    value
        .as_object_mut()
        .unwrap()
        .insert("surprise".to_owned(), serde_json::json!(true));
    let bytes = canonical_json(&value).unwrap();
    assert_eq!(
        decode_canonical::<PolicyTemplateV1>(&bytes)
            .unwrap_err()
            .code(),
        "DOCUMENT_SCHEMA_INVALID"
    );
}

#[test]
fn framing_and_domains_disambiguate_hostile_preimages() {
    let joined_left = hash_framed_bytes("golden.left", b"ab\0c").unwrap();
    let joined_right = hash_framed_bytes("golden.left", b"a\0bc").unwrap();
    let other_domain = hash_framed_bytes("golden.right", b"ab\0c").unwrap();
    assert_ne!(joined_left, joined_right);
    assert_ne!(joined_left, other_domain);
    assert_eq!(
        hash_framed_bytes("UPPER", b"x").unwrap_err().code(),
        "INVALID_HASH_DOMAIN"
    );
}

#[test]
fn generated_cross_language_golden_is_itself_canonical() {
    let bytes = fs::read(root().join("fixtures/canonical/canonical-golden.json")).unwrap();
    let value = decode_canonical_value(&bytes).unwrap();
    assert_eq!(
        value["canonical_json_utf8"],
        r#"{"a":"é","array":[true,null,17],"z":"last"}"#
    );
    assert_eq!(value["domain"], "golden.cross-language");
}
