use super::*;
use crate::publication::tests::Fixture;

#[test]
fn hosted_observation_preserves_event_and_member_identities_without_release_authority() {
    let fixture = Fixture::new();
    let (store, request, prepared) = fixture.prepared();
    let aggregate = fixture.aggregate(&store, &prepared);
    let artifacts = fixture.temp.path().join("artifacts");
    fs::create_dir(&artifacts).unwrap();
    for name in ARTIFACTS {
        fs::write(artifacts.join(name), b"fixture artifact\n").unwrap();
    }
    let context = Context {
        event_sha: prepared.aggregate_commit.clone(),
        event_name: "pull_request".into(),
        run_id: "1234".into(),
        run_attempt: "2".into(),
        workflow_ref: "neverhuman/bulletfarm/.github/workflows/publication.yml@refs/pull/1/merge"
            .into(),
        workflow_sha: prepared.aggregate_commit.clone(),
    };
    let bytes = observe(&aggregate, &fixture.root, &artifacts, context.clone()).unwrap();
    let value: serde_json::Value = crate::publication::decode(&bytes).unwrap();
    assert_eq!(value["context"]["event_sha"], prepared.aggregate_commit);
    assert_eq!(
        value["members"]["bullet-farm"]["commit"],
        request.manifest.members["bullet-farm"].commit
    );
    assert_eq!(value["context"]["run_attempt"], "2");
    assert_eq!(value["release_authority"], false);
    assert_eq!(
        value["artifact_sha256"].as_object().unwrap().len(),
        ARTIFACTS.len()
    );
    let mut wrong = context.clone();
    wrong.event_sha = request.expected_main;
    assert!(observe(&aggregate, &fixture.root, &artifacts, wrong).is_err());
    let mut wrong = context.clone();
    wrong.run_attempt = "0".into();
    assert!(observe(&aggregate, &fixture.root, &artifacts, wrong).is_err());
    fs::remove_file(artifacts.join(ARTIFACTS[0])).unwrap();
    assert!(observe(&aggregate, &fixture.root, &artifacts, context).is_err());
}

#[test]
fn hosted_artifact_inventory_refuses_extra_missing_empty_or_symbolic_bytes() {
    let temp = tempfile::tempdir().unwrap();
    for name in ARTIFACTS {
        fs::write(temp.path().join(name), b"artifact").unwrap();
    }
    assert!(artifact_hashes(temp.path()).is_ok());
    fs::write(temp.path().join("extra"), b"extra").unwrap();
    assert!(artifact_hashes(temp.path()).is_err());
    fs::remove_file(temp.path().join("extra")).unwrap();
    fs::write(temp.path().join(ARTIFACTS[0]), b"").unwrap();
    assert!(artifact_hashes(temp.path()).is_err());
    fs::remove_file(temp.path().join(ARTIFACTS[0])).unwrap();
    std::os::unix::fs::symlink(
        temp.path().join(ARTIFACTS[1]),
        temp.path().join(ARTIFACTS[0]),
    )
    .unwrap();
    assert!(artifact_hashes(temp.path()).is_err());
}
