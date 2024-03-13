#[cfg(feature = "user")]
pub mod user_service;

#[cfg(feature = "hackathon")]
pub mod hackathon_service;

//#[cfg(feature = "org")]
//pub mod organize_service;

#[cfg(feature = "category")]
pub mod category_service;

#[cfg(feature = "curriculum")]
pub mod curriculum_service;

#[cfg(feature = "tracking")]
pub mod tracking_service;

#[cfg(feature = "common")]
pub mod common_service;

#[cfg(feature = "mail")]
pub mod mail_service;

#[cfg(feature = "payment")]
pub mod payment_service;
