fn main() {
    //let serde_impl = "#[derive(serde::Serialize, serde::Deserialize, utoipa::ToSchema, redis_macros::FromRedisValue, redis_macros::ToRedisArgs)]";
    let serde_impl = "#[derive(serde::Serialize, serde::Deserialize, utoipa::ToSchema )]";
    let mut proto_files = vec![];

    #[cfg(feature = "tracking")]
    {
        println!("cargo:rerun-if-changed=proto/tracking_service.proto");
        proto_files.push("proto/service.tracking_service.proto");
    }
    #[cfg(feature = "hackathon")]
    {
        proto_files.push("proto/service.hackathon_service.proto");
    }
    #[cfg(feature = "category")]
    {
        proto_files.push("proto/service.category_service.proto");
    }
    #[cfg(feature = "curriculum")]
    {
        proto_files.push("proto/service.curriculum_service.proto");
    }
    #[cfg(feature = "user")]
    {
        proto_files.push("proto/service.user_service.proto");
    }
    #[cfg(feature = "payment")]
    {
        proto_files.push("proto/service.payment_service.proto");
    }
    #[cfg(feature = "common")]
    {
        proto_files.push("proto/service.builder_service.proto");
        proto_files.push("proto/service.form_service.proto");
    }
    #[cfg(feature = "mail")]
    {
        proto_files.push("proto/service.mail_service.proto");
    }

    tonic_build::configure()
        .out_dir("src")
        .type_attribute(".", serde_impl)
        .message_attribute(".", "#[derive(redis_macros::FromRedisValue)]")
        .message_attribute(".", "#[derive(redis_macros::ToRedisArgs)]")
        .message_attribute(".", "#[derive(sqlx::FromRow)]")
        .message_attribute(".", "#[serde(rename_all = \"camelCase\")]")
        .emit_rerun_if_changed(true)
        .compile(&proto_files, &["proto"])
        .expect("Couldn't compile proto files");
}
