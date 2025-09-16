use parry2d::{math::Vector, na::Isometry2, query, shape::Ball};

pub fn get_contact_distance(
    a_pos: Vector<f32>,
    a_radius: f32,
    b_pos: Vector<f32>,
    b_radius: f32,
) -> Option<f32> {
    let a_ball = Ball::new(a_radius);
    let [x, y] = a_pos.into();
    let a_trans = Isometry2::translation(x, y);

    let b_ball = Ball::new(b_radius);
    let [x, y] = b_pos.into();
    let b_trans = Isometry2::translation(x, y);

    let penetrating = query::contact(
        &a_trans, &a_ball, &b_trans, &b_ball, 1.0, // Estimate
    );

    penetrating.unwrap_or(None).map(|p| p.dist)
}
