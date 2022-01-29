use nannou::prelude::*;
use nannou::noise::*;
use nannou::rand::*;
use rand_distr::{Normal, Distribution};

const TITLE: &str = "angel_hair";
const N_THINGS: usize = 4000;
const FRAME_WIDTH: u32 = 1080;
const FRAME_HEIGHT: u32 = 1080;

fn main() {
    nannou::app(model).update(update).run();
}


struct Thing {
    positions: Vec<Vec2>,
}

impl Thing {
    pub fn new(p:Vec2) -> Self {
        let mut positions = Vec::new();
        positions.push(p);
        Thing {
            positions,
        }
    }
}


struct Model {
    things: Vec<Thing>,
    noise: Perlin,
    draw: nannou::Draw,
}

fn model(app: &App) -> Model {
    let _window = app.new_window()
                     .size(FRAME_WIDTH,FRAME_HEIGHT)
                     .title(TITLE)
                     .view(view)
                     .build()
                     .unwrap();

    // Define the traits for Model
    let mut things = Vec::new();
    for _i in 0..N_THINGS {
        let thing = Thing::new(vec2(
            (random::<f32>()-0.5)*FRAME_WIDTH as f32, 
            (random::<f32>()-0.5)*FRAME_HEIGHT as f32,
            ));
        things.push(thing);
    }
    let noise = Perlin::new();
    let draw = nannou::Draw::new();

    Model {
        things,
        noise,
        draw,
    }
}


fn update(_app: &App, model: &mut Model, _update: Update) {

    let sn = 0.0079;
    for thing in model.things.iter_mut() {   
        thing.positions.clear();
        thing.positions.push(vec2(
            (random::<f32>()-0.5) * FRAME_WIDTH as f32, 
            (random::<f32>()-0.5) * FRAME_HEIGHT as f32,
        ));

        for _i in 0..25 {
            let last = thing.positions[0];
            let new = last + vec2(
                model.noise.get([sn*last.x as f64, sn*last.y as f64, 0.0]) as f32,
                model.noise.get([sn*last.x as f64, sn*last.y as f64, 1.0]) as f32,
            );
            thing.positions.insert(0, new);
        }
        
    }

    let draw = &model.draw;

    let normal = Normal::new(0.4, 0.1).unwrap();
    let v = normal.sample(&mut rand::thread_rng());

    draw.rect().w_h(FRAME_WIDTH as f32, FRAME_HEIGHT as f32).color(srgba(0.145, 0.26, 0.92, 0.06));

    for thing in model.things.iter() {   
        draw.polyline().weight(1.0).points(thing.positions.iter().cloned()).color(hsl(0.07, 0.97, v));
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    model.draw.to_frame(app, &frame).unwrap();
}