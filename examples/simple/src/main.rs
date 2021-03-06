extern crate natura;

use natura::*;
use std::thread::sleep;
use std::time::Duration;

// Where we want to animate it.
const TARGET_X: f64 = 50.0;
const TARGET_Y: f64 = 100.0;

fn main() {
    let mut sprite = Sprite::default();
    let fps = natura::fps(60);
    println!("FPS: {}", fps);

    // Initialize a spring with frame-rate, angular frequency, and damping values.
    let mut spring = Spring::new(DeltaTime(fps), AngularFrequency(6.0), DampingRatio(0.5));

    loop {
        let (sprite_x, sprite_x_velocity) = spring.update(sprite.x, sprite.x_velocity, TARGET_X);
        sprite.x = sprite_x;
        sprite.x_velocity = sprite_x_velocity;

        let (sprite_y, sprite_y_velocity) = spring.update(sprite.y, sprite.y_velocity, TARGET_Y);
        sprite.y = sprite_y;
        sprite.y_velocity = sprite_y_velocity;

        sleep(Duration::from_millis(10000));

        println!(
            "Sprite x:{}, y:{}, x_vel:{}, y_vel:{}",
            sprite.x, sprite.y, sprite.x_velocity, sprite.y_velocity
        )
    }
}
