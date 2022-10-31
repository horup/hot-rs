use context::*;
use crate::Textures;

#[no_mangle]
pub fn draw(canvas:&mut dyn Canvas) {
    let screen_size = canvas.screen_size();
    let scale = 2.0;
    let tex_size = canvas.texture_size(Textures::PokemonCard.into()) * scale;
    
    let margin = 8.0;
    canvas.draw_texture(DrawTextureParams {
        x:margin,
        y:margin,
        w:tex_size.x,
        h:tex_size.y,
        texture:Textures::PokemonCard.into(),
        ..Default::default()
    });
}