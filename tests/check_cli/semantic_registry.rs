#[path = "semantic_registry/fixture.rs"]
mod fixture;

use std::fs;

use bullet_wire::{
    RELEASE_GATE_RECEIPT_SIGNATURE_DOMAIN, hash_framed_bytes, release_bundle_manifest_v2_digest,
};

use super::command;
use fixture::{
    assert_registry_rejected, assert_registry_rejected_with, mutate_registry_manifest,
    profiled_registry_output, release_id, rewrite_registry, write_canonical,
    write_structural_registry,
};

fn add_manifest_fillers(registry: &std::path::Path, count: usize) {
    let manifest_path = registry.join("registry-manifest.json");
    let mut manifest: serde_json::Value =
        serde_json::from_slice(&fs::read(&manifest_path).unwrap()).unwrap();
    let objects = manifest["objects"].as_array_mut().unwrap();
    let insertion = objects
        .iter()
        .position(|object| object["object_kind"] == "signer-policy")
        .unwrap();
    let mut fillers = Vec::with_capacity(count);
    for index in 0..count {
        let bytes = vec![index as u8; 1024 * 1024];
        let path = format!("artifacts/filler-{index:02}.manifest");
        fs::create_dir_all(registry.join("artifacts")).unwrap();
        fs::write(registry.join(&path), &bytes).unwrap();
        fillers.push(serde_json::json!({
            "schema_version": "v1alpha1",
            "object_id": format!("rob_{:064x}", 0x100usize + index),
            "object_kind": "release-bundle-manifest-v2",
            "object_digest": release_bundle_manifest_v2_digest(&bytes).unwrap(),
            "object_path": path,
        }));
    }
    objects.splice(insertion..insertion, fillers);
    write_canonical(&manifest_path, &manifest);
}

#[test]
fn release_profiles_are_named_independent_and_fail_closed() {
    let registry_path =
        std::env::temp_dir().join(format!("bullet-profile-registry-{}", std::process::id()));
    if registry_path.exists() {
        fs::remove_dir_all(&registry_path).unwrap();
    }
    fs::create_dir(&registry_path).unwrap();
    let registry = registry_path.to_str().unwrap();

    let linux_preview = command(&[
        "check",
        "release",
        "--profile",
        "linux-preview",
        "--receipts",
        registry,
        "--json",
    ]);
    assert_eq!(linux_preview.status.code(), Some(3));
    assert!(linux_preview.stderr.is_empty());
    let report: serde_json::Value = serde_json::from_slice(&linux_preview.stdout).unwrap();
    assert_eq!(report["schema_version"], 3);
    assert_eq!(report["profile"], "linux-preview");
    assert_eq!(report["status"], "BLOCKED");
    let ids = report["gates"]
        .as_array()
        .unwrap()
        .iter()
        .map(|gate| gate["id"].as_str().unwrap())
        .collect::<Vec<_>>();
    assert_eq!(ids.len(), 25);
    for excluded in [
        "release.forge.github-app",
        "release.provider.codex",
        "release.provider.cursor",
        "release.provider.antigravity",
        "release.package-matrix",
    ] {
        assert!(!ids.contains(&excluded));
    }
    for required in [
        "release.forge.jeryu",
        "release.provider.claude",
        "release.package-linux-x86_64",
        "release.systemd-v1",
        "release.operations-v1",
        "release.evolution-v1",
    ] {
        assert!(ids.contains(&required));
    }

    for (profile, gate) in [
        ("provider-codex", "release.provider.codex"),
        ("provider-cursor", "release.provider.cursor"),
        ("provider-antigravity", "release.provider.antigravity"),
        ("github-adapter-v1", "release.forge.github-app"),
    ] {
        let output = command(&[
            "check",
            "release",
            "--profile",
            profile,
            "--receipts",
            registry,
            "--json",
        ]);
        assert_eq!(output.status.code(), Some(3), "profile={profile}");
        let report: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
        assert_eq!(report["profile"], profile);
        let gates = report["gates"].as_array().unwrap();
        assert_eq!(gates.len(), 3);
        assert!(gates.iter().any(|item| item["id"] == gate));
        assert!(
            gates
                .iter()
                .any(|item| item["id"] == format!("release.profile.{profile}"))
        );
        assert!(
            gates
                .iter()
                .any(|item| item["id"] == "release.receipt-contracts")
        );
    }

    for profile in [
        "self-hosted-v1",
        "evolution-v1",
        "provider-claude",
        "jeryu-forge-v1",
        "gitlab-adapter-v1",
        "gitlab-self-managed-v1",
        "platform-linux-x86_64",
        "platform-linux-aarch64",
        "platform-macos-x86_64",
        "platform-macos-aarch64",
        "platform-windows-x86_64",
        "universal-v1",
        "team-v1",
        "saga-v1",
    ] {
        let output = command(&[
            "check",
            "release",
            "--profile",
            profile,
            "--receipts",
            registry,
            "--json",
        ]);
        assert_eq!(output.status.code(), Some(3), "profile={profile}");
        let report: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
        assert_eq!(report["profile"], profile);
        assert_eq!(report["status"], "BLOCKED");
    }

    write_structural_registry(&registry_path);
    let structurally_valid = profiled_registry_output(&registry_path, "provider-codex");
    #[cfg(target_os = "linux")]
    {
        assert_eq!(structurally_valid.status.code(), Some(3));
        let report: serde_json::Value = serde_json::from_slice(&structurally_valid.stdout).unwrap();
        let receipt_gate = report["gates"]
            .as_array()
            .unwrap()
            .iter()
            .find(|gate| gate["id"] == "release.receipt-contracts")
            .unwrap();
        assert_eq!(receipt_gate["status"], "BLOCKED");
        assert!(
            receipt_gate["detail"]
                .as_str()
                .unwrap()
                .contains("external trust-root")
        );
        assert!(
            report["gates"]
                .as_array()
                .unwrap()
                .iter()
                .all(|gate| gate["status"] != "PASS")
        );
    }
    #[cfg(not(target_os = "linux"))]
    assert_eq!(structurally_valid.status.code(), Some(1));

    #[cfg(target_os = "linux")]
    {
        write_structural_registry(&registry_path);
        rewrite_registry(&registry_path, |records| {
            records.manifest.registry_signer_key_id = "attestor".to_owned();
        });
        assert_registry_rejected_with(
            &registry_path,
            "provider-codex",
            Some("signer policy subjects are inconsistent"),
        );

        write_structural_registry(&registry_path);
        rewrite_registry(&registry_path, |records| {
            records.time.trusted_time_key_id = "registry".to_owned();
        });
        assert_registry_rejected_with(
            &registry_path,
            "provider-codex",
            Some("trusted-time observation names a key outside"),
        );

        write_structural_registry(&registry_path);
        rewrite_registry(&registry_path, |records| {
            records.receipt.attestor_key_id = "time".to_owned();
        });
        assert_registry_rejected_with(
            &registry_path,
            "provider-codex",
            Some("signer key with the wrong policy role"),
        );

        write_structural_registry(&registry_path);
        rewrite_registry(&registry_path, |records| {
            records.request.family_subject.family = "other-family".to_owned();
            records.receipt.family_subject.family = "other-family".to_owned();
            records.time.family = "other-family".to_owned();
        });
        assert_registry_rejected_with(
            &registry_path,
            "provider-codex",
            Some("release record names the wrong family"),
        );

        write_structural_registry(&registry_path);
        rewrite_registry(&registry_path, |records| {
            records.graph.profiles[0].gate_ids = vec!["release.provider.cursor".to_owned()];
            records.spec.gate_id = "release.provider.cursor".to_owned();
            records.request.gate_id = "release.provider.cursor".to_owned();
            records.receipt.gate_id = "release.provider.cursor".to_owned();
        });
        assert_registry_rejected_with(
            &registry_path,
            "provider-codex",
            Some("changes the declared gates"),
        );

        write_structural_registry(&registry_path);
        rewrite_registry(&registry_path, |records| {
            records.receipt.expires_at_unix_ms = records.request.expires_at_unix_ms + 1;
        });
        assert_registry_rejected_with(
            &registry_path,
            "provider-codex",
            Some("windows are incoherent"),
        );

        write_structural_registry(&registry_path);
        rewrite_registry(&registry_path, |records| {
            records.policy.signer_keys[1].revoked_at_unix_ms = Some(250);
        });
        assert_registry_rejected_with(
            &registry_path,
            "provider-codex",
            Some("attestor key lifecycle"),
        );

        write_structural_registry(&registry_path);
        mutate_registry_manifest(&registry_path, |manifest| {
            manifest["entries"] = serde_json::json!([]);
        });
        assert_registry_rejected_with(
            &registry_path,
            "provider-codex",
            Some("gate/profile coverage differs"),
        );

        write_structural_registry(&registry_path);
        let signature = fs::read(registry_path.join("signatures/provider-codex.sig")).unwrap();
        let legacy_digest = format!(
            "blake3:{}",
            hash_framed_bytes(RELEASE_GATE_RECEIPT_SIGNATURE_DOMAIN, &signature)
                .unwrap()
                .to_hex()
        );
        mutate_registry_manifest(&registry_path, |manifest| {
            manifest["objects"]
                .as_array_mut()
                .unwrap()
                .iter_mut()
                .find(|object| object["object_path"] == "signatures/provider-codex.sig")
                .unwrap()["object_digest"] = legacy_digest.clone().into();
            manifest["entries"]
                .as_array_mut()
                .unwrap()
                .iter_mut()
                .find(|entry| entry["gate_id"] == "release.provider.codex")
                .unwrap()["receipt_signature_digest"] = legacy_digest.into();
        });
        assert_registry_rejected_with(
            &registry_path,
            "provider-codex",
            Some("digest differs from its manifest binding"),
        );

        write_structural_registry(&registry_path);
        add_manifest_fillers(&registry_path, 15);
        assert_registry_rejected_with(
            &registry_path,
            "provider-codex",
            Some("registry contains an unreferenced object"),
        );

        write_structural_registry(&registry_path);
        add_manifest_fillers(&registry_path, 16);
        assert_registry_rejected_with(
            &registry_path,
            "provider-codex",
            Some("aggregate byte budget is exhausted"),
        );
    }

    assert_registry_rejected(&registry_path, "provider-cursor");

    write_structural_registry(&registry_path);
    write_canonical(
        &registry_path.join("registry-manifest.json"),
        &serde_json::json!({"payload":"generic","signature":"self-selected"}),
    );
    assert_registry_rejected(&registry_path, "provider-codex");

    write_structural_registry(&registry_path);
    fs::write(registry_path.join("registry-manifest.json"), b"{malformed").unwrap();
    assert_registry_rejected(&registry_path, "provider-codex");

    write_structural_registry(&registry_path);
    mutate_registry_manifest(&registry_path, |manifest| {
        manifest["objects"][0]["object_path"] = "../receipt.json".into();
    });
    assert_registry_rejected(&registry_path, "provider-codex");

    write_structural_registry(&registry_path);
    mutate_registry_manifest(&registry_path, |manifest| {
        manifest["objects"][1]["object_path"] = manifest["objects"][0]["object_path"].clone();
    });
    assert_registry_rejected(&registry_path, "provider-codex");

    write_structural_registry(&registry_path);
    mutate_registry_manifest(&registry_path, |manifest| {
        manifest["entries"][1]["gate_receipt_id"] = release_id("grc", '3').into();
    });
    assert_registry_rejected(&registry_path, "provider-codex");

    write_structural_registry(&registry_path);
    let receipt_path = registry_path.join("receipts/provider-codex.json");
    let mut receipt: serde_json::Value =
        serde_json::from_slice(&fs::read(&receipt_path).unwrap()).unwrap();
    receipt["gate_receipt_id"] = release_id("grc", '2').into();
    write_canonical(&receipt_path, &receipt);
    assert_registry_rejected(&registry_path, "provider-codex");

    write_structural_registry(&registry_path);
    fs::remove_file(registry_path.join("requests/provider-codex.json")).unwrap();
    assert_registry_rejected(&registry_path, "provider-codex");

    write_structural_registry(&registry_path);
    fs::remove_file(&receipt_path).unwrap();
    fs::create_dir(&receipt_path).unwrap();
    assert_registry_rejected(&registry_path, "provider-codex");

    #[cfg(unix)]
    {
        use std::os::unix::fs::symlink;

        write_structural_registry(&registry_path);
        fs::rename(
            &receipt_path,
            registry_path.join("receipts/provider-codex.real"),
        )
        .unwrap();
        symlink("provider-codex.real", &receipt_path).unwrap();
        assert_registry_rejected(&registry_path, "provider-codex");

        write_structural_registry(&registry_path);
        fs::rename(
            registry_path.join("receipts"),
            registry_path.join("receipts-real"),
        )
        .unwrap();
        symlink("receipts-real", registry_path.join("receipts")).unwrap();
        assert_registry_rejected(&registry_path, "provider-codex");

        write_structural_registry(&registry_path);
        let manifest = registry_path.join("registry-manifest.json");
        fs::rename(&manifest, registry_path.join("registry-manifest.real")).unwrap();
        symlink("registry-manifest.real", &manifest).unwrap();
        assert_registry_rejected(&registry_path, "provider-codex");

        write_structural_registry(&registry_path);
        let receipt_signature = registry_path.join("signatures/provider-codex.sig");
        let time_signature = registry_path.join("time/provider-codex.sig");
        fs::remove_file(&time_signature).unwrap();
        fs::hard_link(&receipt_signature, &time_signature).unwrap();
        assert_registry_rejected(&registry_path, "provider-codex");

        write_structural_registry(&registry_path);
        let admitted = registry_path.with_extension("admitted");
        if admitted.exists() {
            fs::remove_dir_all(&admitted).unwrap();
        }
        fs::rename(&registry_path, &admitted).unwrap();
        symlink(&admitted, &registry_path).unwrap();
        assert_registry_rejected(&registry_path, "provider-codex");
        fs::remove_file(&registry_path).unwrap();
        fs::rename(&admitted, &registry_path).unwrap();
    }
    fs::remove_dir_all(&registry_path).unwrap();
}
