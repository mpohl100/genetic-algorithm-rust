use crate::math2d::{circle::Circle, point::Point, regioned_angle::RegionedAngle, vector::Vector};

use super::{angle_area::AngleArea, source_circle::SourceCircle};

pub fn get_mid_point(from: Point, to: Point) -> Point {
    from + Vector::from((from, to)).scale(0.5)
}

pub enum CircleIntersection {
    None,
    One(Point),
    Two(Point, Point),
}

pub fn calculate_circle_intersection(first: &Circle, second: &Circle) -> CircleIntersection {
    // Calculate the distance between the centers.
    let distance = Vector::from((first.get_center(), second.get_center())).magnitude();

    // Check if the circles are completely separate.
    if distance > first.get_radius() + second.get_radius() {
        return CircleIntersection::None;
    }

    // Check if one circle is contained within the other.
    if distance < (first.get_radius() - second.get_radius()).abs() {
        return CircleIntersection::None;
    }

    // Check if the circles are touching.
    if distance == first.get_radius() + second.get_radius() {
        return CircleIntersection::One(get_mid_point(first.get_center(), second.get_center()));
    }

    // Calculate the intersection points using trigonometry.
    let a = (first.get_radius().powi(2) - second.get_radius().powi(2) + distance.powi(2))
        / (2.0 * distance);
    let h = (first.get_radius().powi(2) - a.powi(2)).sqrt();

    // Calculate the coordinates of the intersection points.
    let p2_x = first.get_center_coords().0
        + a * (second.get_center_coords().0 - first.get_center_coords().0) / distance;
    let p2_y = first.get_center_coords().1
        + a * (second.get_center_coords().1 - first.get_center_coords().1) / distance;

    let intersection_point_1 = Point::new(
        p2_x + h * (second.get_center_coords().1 - first.get_center_coords().1) / distance,
        p2_y - h * (second.get_center_coords().0 - first.get_center_coords().0) / distance,
    );
    let intersection_point_2 = Point::new(
        p2_x - h * (second.get_center_coords().1 - first.get_center_coords().1) / distance,
        p2_y + h * (second.get_center_coords().0 - first.get_center_coords().0) / distance,
    );

    CircleIntersection::Two(intersection_point_1, intersection_point_2)
}

pub fn calculate_first_guess(source_circle: &SourceCircle) -> Circle {
    let angle: RegionedAngle<-180, 180> = source_circle.get_angle_area().get_angle(0.5);
    let radius = source_circle.get_circle().get_radius();
    let center = source_circle.get_circle().get_center()
        + Vector::new(source_circle.get_circle().get_radius() * 2.0, 0.0).rotate(angle);
    Circle::new(center, radius)
}

pub fn deduce_next_source_circles(circle: Circle) -> Vec<SourceCircle> {
    let mut source_circles = Vec::new();
    let nb_angles = 6;
    for i in 0..nb_angles {
        source_circles.push(SourceCircle::new(
            circle,
            AngleArea::new(i as f32, nb_angles),
        ))
    }
    source_circles
}
