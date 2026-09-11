const PORTAL_LAUNCHER: &str = include_str!("../scripts/portal.sh");

#[test]
fn portal_launcher_enforces_the_same_origin_vite_proxy() {
    assert!(
        PORTAL_LAUNCHER.contains("unset VITE_BULLET_API"),
        "the development launcher must clear an inherited cross-origin API base"
    );
    assert!(
        !PORTAL_LAUNCHER.contains("export VITE_BULLET_API"),
        "the development launcher must not export a cross-origin API base"
    );
    assert!(
        !PORTAL_LAUNCHER.contains("127.0.0.1:7420"),
        "browser API requests must use Vite's same-origin proxy"
    );
    assert!(
        PORTAL_LAUNCHER.contains("npm run dev -- --host 127.0.0.1 --port \"$portal_port\""),
        "the development server must stay bound to loopback"
    );
    assert!(
        PORTAL_LAUNCHER.contains("BULLET_PORTAL_PORT"),
        "the printed console origin must be able to select the Vite port"
    );
}
