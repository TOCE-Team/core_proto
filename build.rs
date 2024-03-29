fn main() {
    let serde_impl = "#[derive(serde::Serialize, serde::Deserialize)]";

    #[cfg(feature = "tracking")]
    tonic_build::configure()
        .out_dir("src")
        .type_attribute(".", serde_impl)
        .compile(
            &["proto/tracking/tracking_service.proto"],
            &["proto/tracking"],
        )
        .expect("Couldn't compile proto files");

    #[cfg(feature = "hackathon")]
    tonic_build::configure()
        .out_dir("src")
        .type_attribute(".", serde_impl)
        .compile(&["proto/hackathon_service.proto"], &["proto"])
        .expect("Couldn't compile proto files");

    #[cfg(feature = "category")]
    tonic_build::configure()
        .out_dir("src")
        .type_attribute(".", serde_impl)
        .compile(&["proto/category_service.proto"], &["proto"])
        .expect("Couldn't compile proto files");

    #[cfg(feature = "category")]
    tonic_build::configure()
        .out_dir("src")
        .type_attribute(".", serde_impl)
        .compile(&["proto/category_service.proto"], &["proto"])
        .expect("Couldn't compile proto files");

    #[cfg(feature = "curriculum")]
    tonic_build::configure()
        .out_dir("src")
        .type_attribute(".", serde_impl)
        .compile(
            &["proto/curriculum/curriculum_service.proto"],
            &["proto/curriculum"],
        )
        .expect("Couldn't compile proto files");

    #[cfg(feature = "user")]
    tonic_build::configure()
        .out_dir("src")
        .type_attribute(".", serde_impl)
        .message_attribute(".", "#[derive(sqlx::FromRow)]")
        .compile(&["proto/user_service.proto"], &["proto"])
        .expect("Couldn't compile proto files");

    #[cfg(feature = "common")]
    tonic_build::configure()
        .out_dir("src")
        .type_attribute(".", serde_impl)
        .message_attribute(".", "#[derive(sqlx::FromRow)]")
        .compile(&["proto/common/form_service.proto"], &["proto"])
        .expect("Couldn't compile proto files");
}
