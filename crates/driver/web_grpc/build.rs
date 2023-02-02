use std::{env, path::PathBuf};

use prost_wkt_build::{FileDescriptorSet, Message};

fn main() -> anyhow::Result<()> {
    let out = PathBuf::from(env::var("OUT_DIR").unwrap());
    let descriptor_file = out.join("descriptors.bin");

    tonic_build::configure()
        .build_transport(true)
        .build_server(true)
        .build_client(false)
        .type_attribute(".", "#[derive(serde::Serialize,serde::Deserialize)]")
        .extern_path(".google.protobuf.Any", "::prost_wkt_types::Any")
        .extern_path(
            ".google.protobuf.Timestamp",
            "::prost_wkt_types::Timestamp",
        )
        .extern_path(".google.protobuf.Value", "::prost_wkt_types::Value")
        .file_descriptor_set_path(&descriptor_file)
        .compile(
            &[
                // value types
                "proto/value_types/pagination.proto",
                // models
                "proto/models/user.proto",
                "proto/models/chat.proto",
                "proto/models/instance.proto",
                "proto/models/message.proto",
                // services
                "proto/services/chats.proto",
                "proto/services/stats.proto",
            ],
            &["proto/"],
        )?;

    let descriptor_bytes = std::fs::read(descriptor_file).unwrap();
    let descriptor = FileDescriptorSet::decode(&descriptor_bytes[..]).unwrap();

    prost_wkt_build::add_serde(out, descriptor);

    Ok(())
}
