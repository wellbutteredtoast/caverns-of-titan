use macroquad::prelude::*;

#[macroquad::main("Caverns of Titan")]
async fn main() {
    loop {
        clear_background(BLANK);

        // ...

        next_frame().await;
    }
}