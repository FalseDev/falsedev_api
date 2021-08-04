use rocket::State;

use crate::{
    datastructures::template::TemplateInput, errors::Errors, state::serverstate::ServerState,
};

#[post("/<name>", data = "<template_input>")]
pub async fn template(
    name: String,
    server_state: &State<&'static ServerState>,
    template_input: TemplateInput<'_>,
) -> Result<Vec<u8>, Errors> {
    let template_input = template_input?.into_inner();
    let template = server_state.config.get_template(name)?;
    template.validate(&template_input)?;
    let image = template.process(server_state, template_input).await?;

    let mut bytes: Vec<u8> = Vec::new();
    image.write_to(&mut bytes, image::ImageOutputFormat::Png)?;
    Ok(bytes)
}
