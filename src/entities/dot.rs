use super::polygon::*;
use macroquad::prelude::*;

const DELTA_T: f32 = 0.1;

#[derive(Clone, Debug)]
pub struct Dot {
	pos: Vec2,
	radius: f32,
	vel: Vec2,
	force: Vec2,
	mass: f32,
	freeze: bool,
}

impl Dot {
	pub fn new(pos: Option<Vec2>, mass: Option<f32>) -> Dot {
		Dot {
			pos: pos.unwrap_or(vec2(screen_width() / 2., screen_height() / 2.)),
			mass: mass.unwrap_or(1.),
			radius: 6.,
			vel: vec2(0., 0.),
			force: vec2(0., 0.),
			freeze: false,
		}
	}
	
	pub fn is_out_of_bounds(&self) -> bool {
		(self.pos[0]).abs() > screen_width() * 2. || (self.pos[1]).abs() > screen_height() * 2.
	}

	fn add_gravity(&mut self) {
		self.force += vec2(0., 9.8 * self.mass);
	}

	pub fn update(&mut self) {
		self.add_gravity();
		if !self.freeze {
			self.vel += (self.force * DELTA_T) / self.mass;
			self.pos += self.vel * DELTA_T;
		}
		self.force = vec2(0., 0.);
	}

	fn push(&mut self, push_vec: &Vec2) {
		// self.force += *push_vec;
		self.vel += *push_vec;
		self.pos += *push_vec;
	}

	pub fn draw(&self) {
		draw_circle(self.pos[0], self.pos[1], self.radius, YELLOW);
	}

	fn is_in_bounding_box(&self, polygon: &Polygon) -> bool {
		let (min_hor, max_hor, min_ver, max_ver) = polygon.bounding_box();
		(self.pos[0] - self.radius) < max_hor
			&& (self.pos[0] + self.radius) > min_hor
			&& (self.pos[1] - self.radius) < max_ver
			&& (self.pos[1] + self.radius) > min_ver
	}

	pub fn is_center_inside_polygon(&self, polygon: &Polygon) -> bool {
		let counter = polygon
			.segments()
			.iter()
			.fold(0, |acc, cur| acc + cur.vertical_raycast(&self.pos) as i32);

		counter % 2 != 0
	}

	// Gets the closest projection of the Dot on any segment of a Polygon
	fn get_closest_projection(&self, polygon: &Polygon) -> Vec2 {
		let projections = polygon
			.segments()
			.iter()
			.map(|segment| segment.get_projection(&self.pos))
			.filter(|projection| projection.is_some())
			.map(|projection| projection.unwrap() - self.pos)
			.collect::<Vec<Vec2>>();

		projections.iter().for_each(|proj| {
			draw_line(self.pos[0], self.pos[1], self.pos[0] + proj[0], self.pos[1] + proj[1], 2., RED);
		});

		projections.iter().fold(
			vec2(f32::INFINITY, f32::INFINITY),
			|distance1, &distance2| {
				if distance1.length() < distance2.length() {
					distance1
				} else {
					distance2
				}
			},
		)
	}

	// Calculates push vector to move Dot out of a Polygon
	// If no Polygon intersects the Dot, None is returned
	fn get_push_vector(&self, polygon: &Polygon) -> Option<Vec2> {
		if !self.is_in_bounding_box(polygon) {
			return None;
		}

		let closes_projection = self.get_closest_projection(polygon);
		if closes_projection.length() < self.radius {
			return Some(closes_projection - closes_projection.normalize()*self.radius)
		}

		None
	}

	pub fn handle_collision(&mut self, polygon: &Polygon) {
		match self.get_push_vector(polygon) {
			None => (),
			Some(vector) => {
				self.push(&vector);
			}
		}
	}

}