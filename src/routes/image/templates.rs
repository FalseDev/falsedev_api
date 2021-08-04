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
    let mut bytes: Vec<u8> = Vec::new();
    let image = server_state
        .config
        .get_template(name)?
        .process(server_state, template_input?.into_inner())
        .await?;
    image.write_to(&mut bytes, image::ImageOutputFormat::Png)?;
    Ok(bytes)
}
