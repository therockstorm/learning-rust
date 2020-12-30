use core::{Transform, Vector4f};
use pvs::{self, SceneItem, Source};

#[test]
fn matches_golden_output() {
    let expected = vec![
        SceneItem {
            depth: 0,
            material_override: None,
            parent_id: None,
            source: None,
            supplied_id: "/".to_string(),
            transform: None,
        },
        SceneItem {
            depth: 1,
            material_override: None,
            parent_id: Some("/".to_string()),
            source: None,
            supplied_id: "/109".to_string(),
            transform: None,
        },
        SceneItem {
            depth: 2,
            material_override: None,
            parent_id: Some("/109".to_string()),
            source: Some(Source {
                file_name: "PN1.ol".to_string(),
                supplied_part_id: "PN1, Bolt".to_string(),
                supplied_revision_id: "1".to_string(),
            }),
            supplied_id: "/109/104".to_string(),
            transform: Some(Transform {
                r0: Vector4f {
                    x: 1.0,
                    y: 0.0,
                    z: 0.0,
                    w: 0.0,
                },
                r1: Vector4f {
                    x: 0.0,
                    y: -1.0,
                    z: 0.0,
                    w: 45.0,
                },
                r2: Vector4f {
                    x: 0.0,
                    y: 0.0,
                    z: -1.0,
                    w: 45.0,
                },
                r3: Vector4f {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                    w: 1.0,
                },
            }),
        },
        SceneItem {
            depth: 2,
            material_override: None,
            parent_id: Some("/109".to_string()),
            source: Some(Source {
                file_name: "PN0.ol".to_string(),
                supplied_part_id: "PN0, Washer".to_string(),
                supplied_revision_id: "1".to_string(),
            }),
            supplied_id: "/109/107".to_string(),
            transform: Some(Transform {
                r0: Vector4f {
                    x: 1.0,
                    y: 0.0,
                    z: 0.0,
                    w: 0.0,
                },
                r1: Vector4f {
                    x: 0.0,
                    y: 0.0,
                    z: 1.0,
                    w: 30.0,
                },
                r2: Vector4f {
                    x: 0.0,
                    y: -1.0,
                    z: 0.0,
                    w: 45.0,
                },
                r3: Vector4f {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                    w: 1.0,
                },
            }),
        },
        SceneItem {
            depth: 1,
            material_override: None,
            parent_id: Some("/".to_string()),
            source: Some(Source {
                file_name: "PN0.ol".to_string(),
                supplied_part_id: "PN0, Washer".to_string(),
                supplied_revision_id: "1".to_string(),
            }),
            supplied_id: "/107".to_string(),
            transform: Some(Transform {
                r0: Vector4f {
                    x: 1.0,
                    y: 0.0,
                    z: 0.0,
                    w: 0.0,
                },
                r1: Vector4f {
                    x: 0.0,
                    y: 1.0,
                    z: 0.0,
                    w: 0.000000000000000000000000017993725,
                },
                r2: Vector4f {
                    x: 0.0,
                    y: 0.0,
                    z: 1.0,
                    w: 30.0,
                },
                r3: Vector4f {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                    w: 1.0,
                },
            }),
        },
    ];

    let items = pvs::run("tests/pvs.xml").unwrap();

    assert_eq!(5, items.len());
    assert_eq!(expected, items);
}
