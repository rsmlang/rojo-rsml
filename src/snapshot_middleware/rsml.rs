// Modules -------------------------------------------------------------------------------------------
use std::{collections::HashMap, path::Path};

use memofs::{IoResultExt, Vfs};

use crate::snapshot::{InstanceContext, InstanceMetadata, InstanceSnapshot};

use super::meta_file::AdjacentMetadata;

use rbx_dom_weak::types::{Attributes, Variant};

use rbx_rsml::{tokenize_rsml, parse_rsml, Arena, TokenTreeNode};
// ---------------------------------------------------------------------------------------------------


// Functions -----------------------------------------------------------------------------------------
fn attributes_from_hashmap(variables: &HashMap<&str, Variant>) -> Attributes {
    let mut attributes = Attributes::new();
    if !variables.is_empty() {
        for (key, value) in variables {
            attributes.insert(key.to_string(), value.clone());
        }
    }

    attributes
}

fn apply_token_tree_to_stylesheet_snapshot(
    mut snapshot: InstanceSnapshot, selector: &str, data: &TokenTreeNode, arena: &Arena<TokenTreeNode>
) -> InstanceSnapshot {
    for (selector, child_idx) in &data.rules {
        let mut style_rule = InstanceSnapshot::new()
            .class_name("StyleRule")
            .name(selector.to_owned());

        let child_data = arena.get(*child_idx).unwrap();
        style_rule = apply_token_tree_to_stylesheet_snapshot(style_rule, &selector, &child_data, &arena);

        snapshot.children.push(style_rule);
    }

    let attributes = attributes_from_hashmap(&data.variables);
    let styled_properties = attributes_from_hashmap(&data.properties);

    let priority = match data.priority {
        Some(some_priority) => Variant::Int32(some_priority),
        None => Variant::Int32(0)
    };

    let properties = [
        ("Selector".into(), Variant::String(selector.to_string())),
        ("Priority".into(), priority),
        ("Attributes".into(), attributes.into()),
        ("StyledProperties".into(), styled_properties.into())
    ];

    snapshot.properties(properties)
}
// ---------------------------------------------------------------------------------------------------


pub fn snapshot_rsml(
    context: &InstanceContext,
    vfs: &Vfs,
    path: &Path,
    name: &str,
) -> anyhow::Result<Option<InstanceSnapshot>> {
    let contents = vfs.read_to_string(path)?;
    let contents_str = contents.as_str();

    let tokens = tokenize_rsml(contents_str);
    let token_tree_arena = parse_rsml(&tokens);

    let meta_path = path.with_file_name(format!("{}.meta.json", name));

    let mut snapshot = InstanceSnapshot::new()
        .name(name)
        .class_name("StyleSheet")
        .metadata(
            InstanceMetadata::new()
                .instigating_source(path)
                .relevant_paths(vec![path.to_path_buf(), meta_path.clone()])
                .context(context),
        );

    if let Some(meta_contents) = vfs.read(&meta_path).with_not_found()? {
        let mut metadata = AdjacentMetadata::from_slice(&meta_contents, meta_path)?;
        metadata.apply_all(&mut snapshot)?;
    }

    let root_node = &token_tree_arena.get(0).unwrap();

    let root_attributes = attributes_from_hashmap(&root_node.variables);

    snapshot = snapshot.properties([
        ("Attributes".into(), root_attributes.into()),
    ]);

    for (selector, rule_idx) in &root_node.rules {
        let mut rule_snapshot = InstanceSnapshot::new()
            .class_name("StyleRule")
            .name(selector.to_owned());

        rule_snapshot = apply_token_tree_to_stylesheet_snapshot(
            rule_snapshot, selector, &token_tree_arena.get(rule_idx.to_owned()).unwrap(), &token_tree_arena
        );

        snapshot.children.push(rule_snapshot);
    }

    Ok(Some(snapshot))
}


#[cfg(test)]
mod test {
    use super::*;

    use memofs::{InMemoryFs, VfsSnapshot};

    #[test]
    fn instance_from_vfs() {
        let mut imfs = InMemoryFs::new();
        imfs.load_snapshot("/foo.rsml", VfsSnapshot::file("TextButton {  }"))
            .unwrap();

        let mut vfs = Vfs::new(imfs.clone());

        let instance_snapshot = snapshot_rsml(
            &InstanceContext::default(),
            &mut vfs,
            Path::new("/foo.rsml"),
            "foo",
        )
        .unwrap()
        .unwrap();

        insta::assert_yaml_snapshot!(instance_snapshot);
    }
}