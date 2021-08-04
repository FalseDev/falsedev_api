use rocket::State;

use crate::{
    datastructures::template::TemplateInput, errors::Errors,
    imagelib::image_response::ImageResponse, state::serverstate::ServerState,
};

#[post("/<name>", data = "<template_input>")]
pub async fn template(
    name: String,
    server_state: &State<&'static ServerState>,
    template_input: TemplateInput<'_>,
) -> Result<ImageResponse, Errors> {
    let template_input = template_input?.into_inner();
    let template = server_state.config.get_template(name)?;
    template.validate(&template_input)?;
    let image = template.process(server_state, template_input).await?;
    ImageResponse(image).ok()
}
