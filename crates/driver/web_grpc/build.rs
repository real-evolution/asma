fn main() -> anyhow::Result<()> {
    tonic_build::configure().build_transport(true).compile(
        &[
            // models
            "proto/models/user.proto",
            "proto/models/chat.proto",
            "proto/models/instance.proto",
            "proto/models/message.proto",
            // services
            "proto/services/chats.proto",
        ],
        &["proto/"],
    )?;

    Ok(())
}
