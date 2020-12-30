extern crate quick_xml;

use quick_xml::de;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use core::{self, ColorMaterial, Transform};

const DEFAULT_ORIENTATION: &str = "1,0,0,0,1,0,0,0,1";
const DEFAULT_SUPPLIED_REVISION_ID: &str = "1";
const DEFAULT_TRANSLATION: &str = "0,0,0";
const PATH_ID_SEPARATOR: &str = "/";

#[derive(Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Source {
    pub file_name: String,
    pub supplied_part_id: String,
    pub supplied_revision_id: String,
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SceneItem {
    pub depth: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub material_override: Option<ColorMaterial>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<Source>,
    pub supplied_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transform: Option<Transform>,
}

impl SceneItem {
    fn new(
        path_id: &str,
        part_name: &str,
        part_revision: &str,
        filename: Option<&str>,
        transform: Option<[[f32; 4]; 4]>,
    ) -> SceneItem {
        let supplied_id = if path_id == core::EMPTY_STR {
            PATH_ID_SEPARATOR
        } else {
            path_id
        };
        let mut supplied_id_parts: Vec<&str> = supplied_id.split(PATH_ID_SEPARATOR).collect();
        supplied_id_parts.remove(supplied_id_parts.len() - 1);
        let parent_id = if supplied_id == PATH_ID_SEPARATOR {
            None
        } else {
            Some(supplied_id_parts.join(PATH_ID_SEPARATOR))
        };
        let path_parts: Vec<&str> = path_id.split(PATH_ID_SEPARATOR).collect();

        return SceneItem {
            depth: path_parts.len() - 1,
            material_override: None,
            parent_id: parent_id.map(|p| {
                if p == core::EMPTY_STR {
                    PATH_ID_SEPARATOR.to_string()
                } else {
                    p
                }
            }),
            source: filename.map(|f| Source {
                file_name: f.to_string(),
                supplied_part_id: part_name.to_string(),
                supplied_revision_id: part_revision.to_string(),
            }),
            supplied_id: supplied_id.to_string(),
            transform: match transform {
                Some(t) => {
                    if !core::is_4x4_identity(t) {
                        Some(core::to_transform(t))
                    } else {
                        None
                    }
                }
                None => None,
            },
        };
    }
}

#[derive(Debug, Deserialize, PartialEq)]
struct ShapeSource {
    file_name: String,
}

#[derive(Debug, Deserialize, PartialEq)]
struct ComponentInstance {
    hide_child: Option<bool>,
    hide_self: Option<bool>,
    id: String,
    index: String,
    orientation: Option<String>,
    translation: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq)]
struct Component {
    #[serde(rename = "component_instance", default)]
    component_instances: Vec<ComponentInstance>,
    name: String,
    shape_source: Option<ShapeSource>,
}

#[derive(Debug, Deserialize, PartialEq)]
struct SectionStructure {
    #[serde(rename = "component", default)]
    components: Vec<Component>,
}

#[derive(Debug, Deserialize, PartialEq)]
struct Property {
    name: String,
    value: String,
}

#[derive(Debug, Deserialize, PartialEq)]
struct PropertyComponentRef {
    #[serde(rename = "property", default)]
    properties: Vec<Property>,
}

#[derive(Debug, Deserialize, PartialEq)]
struct SectionProperty {
    #[serde(rename = "property_component_ref", default)]
    property_component_refs: Vec<PropertyComponentRef>,
}

#[derive(Debug, Deserialize, PartialEq)]
struct PvFile {
    section_structure: SectionStructure,
    section_properties: Option<Vec<SectionProperty>>,
}

pub fn run(src: &str) -> Result<Vec<SceneItem>, Box<dyn Error>> {
    let pv = parse_from(src)?;
    let components = &pv.section_structure.components;
    println!("Found {} components.", components.len());

    Ok(create_items(components, components.len() - 1)?)
}

fn parse_from<P: AsRef<Path>>(path: P) -> Result<PvFile, Box<dyn Error>> {
    let pv: PvFile = de::from_reader(BufReader::new(File::open(path)?))?;
    Ok(pv)
}

fn create_items(
    components: &Vec<Component>,
    root_idx: usize,
) -> Result<Vec<SceneItem>, Box<dyn Error>> {
    let mut items = vec![];
    add_items(components, &components[root_idx], "", None, &mut items)?;
    Ok(items)
}

fn add_items(
    components: &Vec<Component>,
    component: &Component,
    path_id: &str,
    transform: Option<[[f32; 4]; 4]>,
    items: &mut Vec<SceneItem>,
) -> Result<(), Box<dyn Error>> {
    if component.component_instances.len() > 0 {
        items.push(SceneItem::new(
            path_id,
            &component.name,
            DEFAULT_SUPPLIED_REVISION_ID,
            None,
            None,
        ));

        for comp_inst in component.component_instances.iter() {
            if comp_inst.hide_self.unwrap_or(false) || comp_inst.hide_child.unwrap_or(false) {
                continue;
            };

            let inst_transform = to_4x4(
                core::to_arr_9(core::to_float_arr(&match &comp_inst.orientation {
                    Some(o) => o,
                    None => DEFAULT_ORIENTATION,
                })?),
                core::to_arr_3(core::to_float_arr(&match &comp_inst.translation {
                    Some(t) => t,
                    None => DEFAULT_TRANSLATION,
                })?),
                1000.0,
            );
            let idx: usize = comp_inst.index.parse()?;
            add_items(
                components,
                &components[idx],
                &format!("{}/{}", path_id, comp_inst.id),
                Some(match transform {
                    Some(t) => core::multiply_4x4(t, inst_transform),
                    None => inst_transform,
                }),
                items,
            )?;
        }
        return Ok(());
    } else {
        match &component.shape_source {
            Some(ss) => {
                items.push(SceneItem::new(
                    path_id,
                    &component.name,
                    DEFAULT_SUPPLIED_REVISION_ID,
                    Some(&ss.file_name),
                    transform,
                ));
                return Ok(());
            }
            None => return Ok(()),
        }
    }
}

fn to_4x4(orientation: [f32; 9], translation: [f32; 3], scale: f32) -> [[f32; 4]; 4] {
    return [
        [
            orientation[0],
            orientation[3],
            orientation[6],
            translation[0] * scale,
        ],
        [
            orientation[1],
            orientation[4],
            orientation[7],
            translation[1] * scale,
        ],
        [
            orientation[2],
            orientation[5],
            orientation[8],
            translation[2] * scale,
        ],
        [0.0, 0.0, 0.0, 1.0],
    ];
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
