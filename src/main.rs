use nannou::prelude::*;
use nannou::noise::*;
use nannou::rand::*;
use rand_distr::{Normal, Distribution};

fn main() {
    nannou::app(model).update(update).run();
}

struct Thing 
{
    positions: Vec<Vec2>,
}
impl Thing
{
    pub fn new(p:Vec2) -> Self {
        let mut positions = Vec::new();
        positions.push(p);
        Thing {
            positions,
        }
    }
}

struct Model 
{
    things: Vec<Thing>,
    noise: Perlin,
}

const N_THINGS: usize = 4000;

fn model(app: &App) -> Model {
    let _window = app.new_window().size(1024,1024).view(view).build().unwrap();
    let mut things = Vec::new();

    for _i in 0..N_THINGS
    {
        let thing = Thing::new(vec2(
            (random::<f32>()-0.5)*1024.0, 
            (random::<f32>()-0.5)*1024.0,
            ));
        things.push(thing);
    }

    let noise = Perlin::new();
    Model
    {
        things,
        noise,
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) 
{
    let sn = 0.0079;
    for thing in model.things.iter_mut() 
    {   
        thing.positions.clear();
        thing.positions.push(vec2(
            (random::<f32>()-0.5)*1024.0, 
            (random::<f32>()-0.5)*1024.0,
        ));

        for _i in 0..19
        {
            let last = thing.positions[0];
            let new = last + vec2(
                model.noise.get([sn*last.x as f64, sn*last.y as f64, 0.0]) as f32,
                model.noise.get([sn*last.x as f64, sn*last.y as f64, 1.0]) as f32,
            );
            thing.positions.insert(0, new);
        }
        
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let normal = Normal::new(0.5, 0.1).unwrap();
    let v = normal.sample(&mut rand::thread_rng());

    draw.rect().w_h(1024.0,1024.0).color(srgba(0.145, 0.26, 0.92, 0.06));

    for thing in model.things.iter()
    {   
        draw.polyline().weight(1.0).points(thing.positions.iter().cloned()).color(hsl(0.07, 0.97, v));
    }
    
    draw.to_frame(app, &frame).unwrap();
}