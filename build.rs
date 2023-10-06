fn main() {
    let serde_impl = "#[derive(serde::Serialize, serde::Deserialize)]";

    #[cfg(feature = "user")]
    tonic_build::configure()
        .out_dir("src")
        .enum_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
        .message_attribute(
            ".",
            "#[derive(serde::Serialize, serde::Deserialize, derive_builder::Builder)]",
        )
        .compile(&["proto/user_service.proto"], &["proto"])
        .expect("Couldn't compile proto files");

    #[cfg(feature = "hackathon")]
    tonic_build::configure()
        .out_dir("src")
        .type_attribute(".", serde_impl)
        .compile(&["proto/hackathon_service.proto"], &["proto"])
        .expect("Couldn't compile proto files");
}
