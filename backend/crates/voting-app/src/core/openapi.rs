use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(info(
    title = "Voting App API",
    description = "HTTP API for the voting application backend."
))]
pub struct ApiDoc;
