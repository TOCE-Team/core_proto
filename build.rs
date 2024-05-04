fn main() {
    //let serde_impl = "#[derive(serde::Serialize, serde::Deserialize, utoipa::ToSchema, redis_macros::FromRedisValue, redis_macros::ToRedisArgs)]";
    let serde_impl = "#[derive(serde::Serialize, serde::Deserialize, utoipa::ToSchema)]";

    #[cfg(feature = "tracking")]
    tonic_build::configure()
        .out_dir("src")
        .type_attribute(".", serde_impl)
        .compile(&["proto/tracking_service.proto"], &["proto"])
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
        .message_attribute(".", "#[derive(redis_macros::FromRedisValue)]")
        .message_attribute(".", "#[derive(redis_macros::ToRedisArgs)]")
        .compile(&["proto/curriculum_service.proto"], &["proto"])
        .expect("Couldn't compile proto files");

    #[cfg(feature = "user")]
    tonic_build::configure()
        .out_dir("src")
        .type_attribute(".", serde_impl)
        .message_attribute(".", "#[derive(sqlx::FromRow)]")
        .compile(&["proto/user_service.proto"], &["proto"])
        .expect("Couldn't compile proto files");

    #[cfg(feature = "payment")]
    tonic_build::configure()
        .out_dir("src")
        .type_attribute(".", serde_impl)
        .message_attribute(".", "#[derive(sqlx::FromRow)]")
        .compile(&["proto/payment_service.proto"], &["proto"])
        .expect("Couldn't compile proto files");

    #[cfg(feature = "common")]
    tonic_build::configure()
        .out_dir("src")
        .type_attribute(".", serde_impl)
        .message_attribute(".", "#[derive(sqlx::FromRow)]")
        .compile(&["proto/form_service.proto"], &["proto"])
        .expect("Couldn't compile proto files");

    #[cfg(feature = "mail")]
    tonic_build::configure()
        .out_dir("src")
        .type_attribute(".", serde_impl)
        .message_attribute(".", "#[derive(sqlx::FromRow)]")
        .compile(&["proto/mail_service.proto"], &["proto"])
        .expect("Couldn't compile proto files");
}
