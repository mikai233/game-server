use std::path::PathBuf;
use std::str::FromStr;

use protobuf::descriptor::field_descriptor_proto::Type;
use protobuf::EnumOrUnknown;
use protobuf::reflect::{FieldDescriptor, MessageDescriptor, RuntimeFieldType};
use protobuf_codegen::{Customize, CustomizeCallback};

struct GenSerde;

impl CustomizeCallback for GenSerde {
    fn message(&self, _message: &MessageDescriptor) -> Customize {
        Customize::default().before("#[derive(::serde::Serialize, ::serde::Deserialize)]")
    }

    fn field(&self, field: &FieldDescriptor) -> Customize {
        match field.proto().type_() {
            Type::TYPE_MESSAGE => {
                if !field.is_repeated_or_map() {
                    Customize::default().before(
                        "#[serde(with = \"crate::MessageFieldDef\")]")
                } else {
                    Customize::default()
                }
            }
            Type::TYPE_ENUM => {
                // `EnumOrUnknown` is not a part of rust-protobuf, so external serializer is needed.
                Customize::default().before(
                    "#[serde(serialize_with = \"crate::serialize_enum_or_unknown\", deserialize_with = \"crate::deserialize_enum_or_unknown\")]")
            }
            _ => {
                Customize::default()
            }
        }
    }

    fn special_field(&self, _message: &MessageDescriptor, _field: &str) -> Customize {
        Customize::default().before("#[serde(skip)]")
    }
}

fn main() -> anyhow::Result<()> {
    let protoc_path = protoc_bin_vendored::protoc_bin_path()?;
    let proto_path = PathBuf::from_str("src/proto")?;
    let mut inputs = vec![];
    for dir in walkdir::WalkDir::new(&proto_path) {
        let dir = dir?;
        if dir.file_type().is_file() {
            let proto_path = dir.into_path();
            if let Some(ext) = proto_path.extension() {
                if ext == "proto" {
                    inputs.push(proto_path);
                }
            }
        }
    }
    protobuf_codegen::Codegen::new()
        .protoc_path(&protoc_path)
        .cargo_out_dir("game_proto")
        .customize_callback(GenSerde)
        .inputs(inputs)
        .includes([proto_path])
        .run_from_script();
    Ok(())
}