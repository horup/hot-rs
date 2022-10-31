use context::*;
use crate::Textures;

#[no_mangle]
pub fn draw(canvas:&mut dyn Canvas) {
    canvas.draw_texture(DrawTextureParams {
        x:10.0,
        y:10.0,
        w:100.0,
        h:100.0,
        texture:Textures::PokemonCard.into(),
        ..Default::default()
    });

    canvas.draw_texture(DrawTextureParams {
        x:5.0,
        y:5.0,
        w:100.0,
        h:100.0,
        texture:Textures::PokemonCard.into(),
        ..Default::default()
    });

    canvas.draw_texture(DrawTextureParams {
        x:55.0,
        y:55.0,
        w:100.0,
        h:100.0,
        texture:Textures::PokemonCard.into(),
        ..Default::default()
    });

    canvas.draw_string(DrawStringParams {
        str:"Hello World".into(),
        x:16.0,
        y:16.0,
        ..Default::default()
    });
}