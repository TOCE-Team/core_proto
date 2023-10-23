fn main() {
    #[cfg(feature = "user")]
    tonic_build::configure()
        .out_dir("src")
        .type_attribute(
            ".",
            "#[derive(serde::Serialize, serde::Deserialize, sqlx::FromRow)]",
        )
        .compile(&["proto/user_service.proto"], &["."])
        .expect("Couldn't compile proto files");

    #[cfg(feature = "hackathon")]
    tonic_build::configure()
        .out_dir("src")
        .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
        .compile(&["proto/hackathon_service.proto"], &["."])
        .expect("Couldn't compile proto files");
}
