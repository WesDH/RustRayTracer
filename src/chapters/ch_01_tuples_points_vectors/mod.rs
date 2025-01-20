use crate::features::tuples;
use std::{thread, time};

#[derive(Clone, Copy)]
pub struct Projectile {
    pub position: tuples::Tuple, // -> point
    pub velocity: tuples::Tuple, // -> vector
}

#[derive(Clone, Copy)]
pub struct Environment {
    pub gravity: tuples::Tuple, // -> vector
    pub wind: tuples::Tuple,    // -> vector
}

pub fn run() {
    // End of chapter 1 exercise

    let one_second = time::Duration::from_millis(200);
    // projectile starts one unit above the origin.
    // velocity is normalized to 1 unit/tick.
    let mut cannon_ball = Projectile {
        position: tuples::point(0., 1., 0.),
        velocity: tuples::normalize(tuples::vector(1., 1., 0.)),
    };

    // gravity -0.1 unit/tick, and wind is -0.01 unit/tick.
    let e = Environment {
        gravity: tuples::vector(0., -0.1, 0.),
        wind: tuples::vector(-0.01, 0., 0.),
    };


    while cannon_ball.position.y > -2. {
        println!("===========");
        println!("Cannon ball position on x axis is {}", cannon_ball.position.x);
        println!("Cannon ball position on y axis is {}", cannon_ball.position.y);
        println!("Cannon ball position on z axis is {}", cannon_ball.position.z);
        println!("Cannon ball velocity it {}", cannon_ball.position.z);
        cannon_ball = tick(cannon_ball, e);
        thread::sleep(one_second);  // For now, sleep is the 'tick'
    }


    println!("Cannon ball finished falling!")
}

pub fn tick(mut projectiles: Projectile, env: Environment) -> Projectile {

    // Update position with cur velocity
    let mut pos = tuples::add(projectiles.position, projectiles.velocity);
    projectiles.position = pos.unwrap();


    // Update projectiles.velocity with wind and gravity vectors
    let add_wind = tuples::add(projectiles.velocity, env.wind);
    let add_wind_grav = tuples::add(add_wind.unwrap(), env.gravity);
    projectiles.velocity = add_wind_grav.unwrap();

    projectiles

}
