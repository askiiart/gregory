use alphanumeric_sort::sort_str_slice;

#[test]
/// This isn't to test the program, more to test the crate works how I want, especially if I switch crates
fn test_semver_sorting() {
    // copied from https://pkgs.org/download/xorg-x11-xauth
    let mut versions = [
        "xorg-x11-xauth-1.1.3",
        "xorg-x11-xauth-1.1.2",
        "xorg-x11-xauth-1.0.9",
        "xorg-x11-xauth-1.0.10",
        "xorg-x11-xauth-1.1",
    ];
    sort_str_slice(&mut versions);
    assert_eq!(
        versions,
        [
            "xorg-x11-xauth-1.0.9",
            "xorg-x11-xauth-1.0.10",
            "xorg-x11-xauth-1.1",
            "xorg-x11-xauth-1.1.2",
            "xorg-x11-xauth-1.1.3"
        ]
    );
}

#[test]
fn test_date_versioning() {
    // copied from https://pkgs.org/download/vpnc-script
    let mut versions = [
        "vpnc-script-20230907",
        "vpnc-script-20230103",
        "vpnc-script-20220404",
    ];
    sort_str_slice(&mut versions);
    assert_eq!(
        versions,
        [
            "vpnc-script-20220404",
            "vpnc-script-20230103",
            "vpnc-script-20230907"
        ]
    );
}
#[test]
fn test_git_versioning() {
    // copied from aurpublish versions - https://gitlab.archlinux.org/archlinux/packaging/packages/avahi/-/commits/main
    let mut versions = [
        "1:0.8+r194+g3f79789-3",
        "1:0.8+r194+g3f79789-2",
        "0.7+4+gd8d8c67-1",
        "0.8+r189+g35bb1ba-1",
        "0.8+r127+g55d783d-1",
    ];
    sort_str_slice(&mut versions);
    assert_eq!(
        versions,
        [
            "0.7+4+gd8d8c67-1",
            "0.8+r127+g55d783d-1",
            "0.8+r189+g35bb1ba-1",
            "1:0.8+r194+g3f79789-2",
            "1:0.8+r194+g3f79789-3"
        ]
    );
}
