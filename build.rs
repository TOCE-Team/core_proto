fn main() {
    let serde_impl = "#[derive(serde::Serialize, serde::Deserialize)]";

    #[cfg(feature = "user")]
    tonic_build::configure()
        .out_dir("src")
        .type_attribute(".", serde_impl)
        .compile(&["proto/user_service.proto"], &["proto"])
        .expect("Couldn't compile proto files");

    #[cfg(feature = "hackathon")]
    tonic_build::configure()
        .out_dir("src")
        .type_attribute(".", serde_impl)
        .compile(&["proto/hackathon_service.proto"], &["proto"])
        .expect("Couldn't compile proto files");
}
