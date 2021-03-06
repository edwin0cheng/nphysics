use draw_helper::DRAW_SCALE;
use na::{Isometry2, Point3};
use nphysics2d::object::ColliderHandle;
use nphysics2d::world::World;
use objects;
use sfml::graphics;
use sfml::graphics::{CircleShape, Color, RenderTarget, Shape, Transformable};
use sfml::system::Vector2f;

pub struct Ball<'a> {
    color: Point3<u8>,
    base_color: Point3<u8>,
    delta: Isometry2<f32>,
    collider: ColliderHandle,
    gfx: CircleShape<'a>,
}

impl<'a> Ball<'a> {
    pub fn new(
        collider: ColliderHandle,
        _: &World<f32>,
        delta: Isometry2<f32>,
        radius: f32,
        color: Point3<u8>,
    ) -> Ball<'a> {
        let dradius = radius as f32 * DRAW_SCALE;

        let mut res = Ball {
            color: color,
            base_color: color,
            delta: delta,
            collider: collider,
            gfx: CircleShape::new().unwrap(),
        };

        res.gfx
            .set_fill_color(&Color::new_rgb(color.x, color.y, color.z));
        res.gfx.set_radius(dradius);
        res.gfx.set_origin(&Vector2f::new(dradius, dradius));

        res
    }
}

impl<'a> Ball<'a> {
    pub fn collider(&self) -> ColliderHandle {
        self.collider
    }

    pub fn update(&mut self, world: &World<f32>) {
        objects::update_scene_node(
            &mut self.gfx,
            self.collider,
            world,
            &self.color,
            &self.delta,
        )
    }

    pub fn draw(&self, rw: &mut graphics::RenderWindow, _: &World<f32>) {
        rw.draw(&self.gfx);
    }

    pub fn set_color(&mut self, color: Point3<u8>) {
        self.color = color;
        self.base_color = color;
    }

    pub fn select(&mut self) {
        self.color = Point3::new(200, 0, 0);
    }

    pub fn unselect(&mut self) {
        self.color = self.base_color;
    }
}
