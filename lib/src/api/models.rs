use poem_openapi::Tags;

#[derive(Tags)]
pub enum ApiTags {
    Auth,
    User,
}
