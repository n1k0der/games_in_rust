use comfy::*;

simple_game!("Nice red circle", update);

fn update(_c: &mut EngineContext) {
   draw_circle(vec2(0.0, 0.0), 0.5, RED, 0);
}