use super::{git, store, tests::Fixture, transport};

fn remote(fixture: &Fixture, store: &store::Store, request: &store::Request) -> String {
    git::bytes(fixture.temp.path(), &["init", "--bare", "remote.git"]).unwrap();
    let remote = fixture.temp.path().join("remote.git");
    git::execute(
        git::command(&store.objects)
            .arg("push")
            .arg(&remote)
            .arg(format!("{}:refs/heads/main", request.expected_main)),
        None,
    )
    .unwrap();
    remote.to_str().unwrap().into()
}

#[test]
fn atomic_source_publication_reconstructs_real_checkouts_and_reconciles_response_loss() {
    let fixture = Fixture::new();
    let (store, request, prepared) = fixture.prepared();
    let remote = remote(&fixture, &store, &request);
    let first = transport::publish_to(&store, &request, &prepared, &remote, None).unwrap();
    // Receipt was lost. Same request reads back the same immutable effects.
    assert_eq!(
        first,
        transport::publish_to(&store, &request, &prepared, &remote, None).unwrap()
    );
    let aggregate = fixture.aggregate(&store, &prepared);
    let root = fixture.temp.path().join("reconstructed");
    transport::reconstruct_from(&aggregate, &root, &remote).unwrap();
    assert_eq!(super::capture(&root).unwrap(), request.manifest);
    assert!(transport::reconstruct_from(&aggregate, &root, &remote).is_err());
    // A later integrated main does not rewrite the original publication receipt.
    git::bytes(
        std::path::Path::new(&remote),
        &["update-ref", "refs/heads/main", &prepared.aggregate_commit],
    )
    .unwrap();
    assert_eq!(
        first,
        transport::publish_to(&store, &request, &prepared, &remote, None).unwrap()
    );
}

#[test]
fn conflicting_or_stale_remote_ref_cannot_partially_publish() {
    let fixture = Fixture::new();
    let (store, request, prepared) = fixture.prepared();
    let remote = remote(&fixture, &store, &request);
    let subject = &request.manifest.members["bullet-git"];
    git::bytes(
        std::path::Path::new(&remote),
        &["update-ref", &subject.source_ref, &request.expected_main],
    )
    .unwrap();
    assert!(transport::publish_to(&store, &request, &prepared, &remote, None).is_err());
    assert!(
        git::text(
            std::path::Path::new(&remote),
            &["for-each-ref", "--format=%(refname)", &prepared.review_ref]
        )
        .unwrap()
        .is_empty()
    );
    git::bytes(
        std::path::Path::new(&remote),
        &["update-ref", "-d", &subject.source_ref],
    )
    .unwrap();
    git::bytes(
        std::path::Path::new(&remote),
        &["update-ref", "-d", "refs/heads/main"],
    )
    .unwrap();
    assert!(transport::publish_to(&store, &request, &prepared, &remote, None).is_err());
    assert!(
        git::text(
            std::path::Path::new(&remote),
            &["for-each-ref", "--format=%(refname)"]
        )
        .unwrap()
        .is_empty()
    );
}

#[test]
fn reconstructed_source_ref_drift_is_refused_before_checkout() {
    let fixture = Fixture::new();
    let (store, request, prepared) = fixture.prepared();
    let remote = remote(&fixture, &store, &request);
    transport::publish_to(&store, &request, &prepared, &remote, None).unwrap();
    let subject = &request.manifest.members["bullet-git"];
    git::bytes(
        std::path::Path::new(&remote),
        &["update-ref", &subject.source_ref, &request.expected_main],
    )
    .unwrap();
    let aggregate = fixture.aggregate(&store, &prepared);
    assert!(
        transport::reconstruct_from(&aggregate, &fixture.temp.path().join("drift"), &remote)
            .is_err()
    );
}

#[test]
fn actual_prepare_recovers_interruption_after_refs_and_rejects_changed_request() {
    let fixture = Fixture::new();
    let (objects, request, _) = fixture.prepared();
    let remote = remote(&fixture, &objects, &request);
    let path = fixture.root.join("bullet-farm/.git/bullet-publication");
    let first = store::prepare_from(
        &fixture.root,
        &path,
        "real-prepare",
        &request.expected_main,
        &remote,
    )
    .unwrap();
    assert_eq!(
        first,
        store::prepare_from(
            &fixture.root,
            &path,
            "real-prepare",
            &request.expected_main,
            &remote
        )
        .unwrap()
    );
    std::fs::remove_file(path.join("real-prepare.prepared.json")).unwrap();
    assert_eq!(
        first,
        store::prepare_from(
            &fixture.root,
            &path,
            "real-prepare",
            &request.expected_main,
            &remote
        )
        .unwrap()
    );
    assert!(
        store::prepare_from(
            &fixture.root,
            &path,
            "real-prepare",
            &request.manifest.members["bullet-git"].commit,
            &remote
        )
        .is_err()
    );
    std::fs::write(
        fixture.root.join("bullet-git/source.txt"),
        "reviewed later source\n",
    )
    .unwrap();
    super::tests::commit(&fixture.root.join("bullet-git"), "changed source subject");
    assert!(
        store::prepare_from(
            &fixture.root,
            &path,
            "real-prepare",
            &request.expected_main,
            &remote
        )
        .is_err()
    );
}

#[test]
fn git_basic_auth_encoding_and_unrelated_child_custody_are_exact() {
    for (input, expected) in [
        ("", ""),
        ("f", "Zg=="),
        ("fo", "Zm8="),
        ("foo", "Zm9v"),
        ("foob", "Zm9vYg=="),
        ("fooba", "Zm9vYmE="),
        ("foobar", "Zm9vYmFy"),
    ] {
        assert_eq!(transport::base64(input.as_bytes()), expected);
    }
    assert_eq!(transport::base64(&[0, 255, 128]), "AP+A");
    let command = git::command(std::path::Path::new("/tmp"));
    for key in [
        "BULLET_PUBLICATION_TOKEN",
        "GH_TOKEN",
        "GITHUB_TOKEN",
        "GH_ENTERPRISE_TOKEN",
    ] {
        assert!(
            command
                .get_envs()
                .any(|(name, value)| name == key && value.is_none())
        );
    }
}
