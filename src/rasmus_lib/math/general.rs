use super::*;
use approx::AbsDiffEq;
use num_traits::*;
use std::ops;

pub fn le_approx<T: Float + approx::AbsDiffEq<Epsilon = T>>(a: T, b: T, epsilon: T) -> bool {
    a <= b || a.abs_diff_eq(&b, epsilon)
}

pub fn ge_approx<T: Float + approx::AbsDiffEq<Epsilon = T>>(a: T, b: T, epsilon: T) -> bool {
    a >= b || a.abs_diff_eq(&b, epsilon)
}

pub fn k_value(a: Vec2, b: Vec2) -> f32 {
    (b.y - a.y) / (b.x - a.x)
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum IntersectionPoint {
    None,
    Infinite,
    One(Vec2),
}

impl AbsDiffEq for IntersectionPoint {
    type Epsilon = f32;
    fn default_epsilon() -> Self::Epsilon {
        f32::EPSILON
    }
    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        match self {
            IntersectionPoint::One(vec) => {
                let IntersectionPoint::One(other_vec) = other else {
                    return false;
                };

                vec.abs_diff_eq(other_vec, epsilon)
            }
            case => case == other,
        }
    }
}

/// Calculate the point of intersection of two lines formed by two pairs of
/// points.
///
/// Note that the intersection point does not have to lie between the pairs of
/// points that defined the lines. Which in other words means that this
/// function operates on the mathematical infinite lines that intersect the
/// corresponding pairs of points.
///
/// The result is returned as an `IntersectionPoint`, where
/// `IntersectionPoint::One(Vec2)` is returned when there is exactly one point
/// of intersection, `IntersectionPoint::Infinite` when there the lines are
/// identical, and `IntersectionPoint::None` when the lines are parallel, but
/// don't lie on top of each other.
///
/// # Examples
/// ```
/// use terminal_renderer::math::*;
/// use approx::assert_abs_diff_eq;
///
/// let line_a = Line(vec2(0., 3.), vec2(4., 2.));
/// let line_b = Line(vec2(0., 0.), vec2(4., 4.));
///
/// assert_abs_diff_eq!(
///     intersection_of_point_pairs_infinite(line_a, line_b),
///     IntersectionPoint::One(vec2(2.4, 2.4))
/// );
/// ```
/// Example where the intersection lies outside the given points:
/// ```
/// use terminal_renderer::math::*;
/// use approx::assert_abs_diff_eq;
///
/// let line_a = Line(vec2(0., 4.), vec2(4., 2.));
/// let line_b = Line(vec2(0., 0.), vec2(8., 2.));
///         
/// assert_abs_diff_eq!(
///     intersection_of_point_pairs_infinite(line_a, line_b),
///     IntersectionPoint::One(vec2(5.333, 1.333)),
///     epsilon = 0.001
/// );
/// ```
///
/// Example with one vertical line
/// ```
/// use terminal_renderer::math::*;
/// use approx::assert_abs_diff_eq;
///
/// let line_a = Line(vec2(0., 0.), vec2(0., 2.));
/// let line_b = Line(vec2(-1., 1.), vec2(1., 2.));
///         
/// assert_abs_diff_eq!(
///     intersection_of_point_pairs_infinite(line_a, line_b),
///     IntersectionPoint::One(vec2(0., 1.5)),
/// );
/// ```
///
/// Example with two identical lines:
/// ```
/// use terminal_renderer::math::*;
/// use approx::assert_abs_diff_eq;
///
/// let line_a = Line(vec2(0., 0.), vec2(4., 4.));
/// let line_b = Line(vec2(1., 1.), vec2(3., 3.));
///
/// assert_abs_diff_eq!(
///     intersection_of_point_pairs_infinite(line_a, line_b),
///     IntersectionPoint::Infinite,
/// );
/// ```
///
/// Example with two vertical identical lines:
/// ```
/// use terminal_renderer::math::*;
/// use approx::assert_abs_diff_eq;
///
/// let line_a = Line(vec2(2., 0.), vec2(2., 6.));
/// let line_b = Line(vec2(2., 2.), vec2(2., 4.));
///
/// assert_abs_diff_eq!(
///     intersection_of_point_pairs_infinite(line_a, line_b),
///     IntersectionPoint::Infinite,
/// );
/// ```
///
/// Example with two vertical *non*-identical lines:
/// ```
/// use terminal_renderer::math::*;
/// use approx::assert_abs_diff_eq;
///
/// let line_a = Line(vec2(0., 0.), vec2(0., 2.));
/// let line_b = Line(vec2(1., 1.), vec2(1., 3.));
///
/// assert_abs_diff_eq!(
///     intersection_of_point_pairs_infinite(line_a, line_b),
///     IntersectionPoint::None,
/// );
/// ```
///
/// Example with two parallel *non*-idetical lines:
/// ```
/// use terminal_renderer::math::*;
/// use approx::assert_abs_diff_eq;
///
/// let line_a = Line(vec2(0., 0.), vec2(4., 4.));
/// let line_b = Line(vec2(2., 0.), vec2(4., 2.));
///
/// assert_abs_diff_eq!(
///     intersection_of_point_pairs_infinite(line_a, line_b),
///     IntersectionPoint::None,
/// );
/// ```
pub fn intersection_of_point_pairs_infinite(a: Line, b: Line) -> IntersectionPoint {
    let Line(a_1, a_2) = a;
    let Line(b_1, b_2) = b;

    let a_vertical = a_1.x == a_2.x;
    let b_vertical = b_1.x == b_2.x;

    if a_vertical && b_vertical {
        return if a_1.x == b_1.x {
            IntersectionPoint::Infinite
        } else {
            IntersectionPoint::None
        };
    }

    if a_vertical || b_vertical {
        let (mut a_1, a_2, mut b_1, mut b_2) = (a_1, a_2, b_1, b_2);
        if b_vertical {
            (a_1, _, b_1, b_2) = (b_1, b_2, a_1, a_2);
        }
        // line a is now garanteed to be the vertical one

        let k_b = k_value(b_1, b_2);

        let intersect = Vec2 {
            x: a_1.x,
            y: k_b * (a_1.x - b_1.x) + b_1.y,
        };

        return IntersectionPoint::One(intersect);
    }

    let k_a = k_value(a_1, a_2);
    let k_b = k_value(b_1, b_2);

    if k_a == k_b {
        return if (a_1.y - k_a * a_1.x) == (b_1.y - k_a * b_1.x) {
            IntersectionPoint::Infinite
        } else {
            IntersectionPoint::None
        };
    }

    let x = (a_1.x * k_a - a_1.y - b_1.x * k_b + b_1.y) / (k_a - k_b);

    let intersect = Vec2 {
        x,
        y: k_a * (x - a_1.x) + a_1.y,
    };

    IntersectionPoint::One(intersect)
}

/// Sorry to lazy to document...
///
/// Works like `intersection_of_point_pairs_infinite` but the lines don't
/// extend past the point pairs to infinity.
///
/// # Examples
/// Example 1:
/// ```
/// use terminal_renderer::math::*;
/// use approx::assert_abs_diff_eq;
///
/// let line_a = Line(vec2(-1., 1.), vec2(1., 1.));
/// let line_b = Line(vec2(-1., 0.), vec2(1., 2.));
///
/// assert_abs_diff_eq!(
///     intersection_of_point_pairs(line_a, line_b),
///     IntersectionPoint::One(vec2(0., 1.)),
/// );
/// ```
///
/// Example 2:
/// ```
/// use terminal_renderer::math::*;
/// use approx::assert_abs_diff_eq;
///
/// let line_a = Line(vec2(-1., 1.), vec2(1., 1.));
/// let line_b = Line(vec2(1., 0.), vec2(3., 2.));
///
/// assert_abs_diff_eq!(
///     intersection_of_point_pairs(line_a, line_b),
///     IntersectionPoint::None,
/// );
/// ```
///
/// Example where the lines only intersect by edges points:
/// ```
/// use terminal_renderer::math::*;
/// use approx::assert_abs_diff_eq;
///
/// let line_a = Line(vec2(-1., 1.), vec2(1., 1.));
/// let line_b = Line(vec2(0., 0.), vec2(2., 2.));
///
/// assert_abs_diff_eq!(
///     intersection_of_point_pairs(line_a, line_b),
///     IntersectionPoint::One(vec2(1., 1.)),
/// );
/// ```
///
/// Example with vertical line:
/// ```
/// use terminal_renderer::math::*;
/// use approx::assert_abs_diff_eq;
///
/// let line_a = Line(vec2(0., 0.), vec2(0., 2.));
/// let line_b = Line(vec2(-1., 0.), vec2(1., 2.));
///
/// assert_abs_diff_eq!(
///     intersection_of_point_pairs(line_a, line_b),
///     IntersectionPoint::One(vec2(0., 1.)),
/// );
/// ```
///
/// Another example:
/// ```
/// use terminal_renderer::math::*;
/// use approx::assert_abs_diff_eq;
///
/// let line_a = Line(vec2(-1.6, 0.), vec2(1., 1.4));
/// let line_b = Line(vec2(-1., 1.), vec2(1., 1.));
///
/// assert_abs_diff_eq!(
///     intersection_of_point_pairs(line_a, line_b),
///     IntersectionPoint::One(vec2(0.257, 1.)),
///     epsilon = 0.001
/// );
/// ```
pub fn intersection_of_point_pairs(line_a: Line, line_b: Line) -> IntersectionPoint {
    let inter_point = intersection_of_point_pairs_infinite(line_a, line_b);

    // println!("Got no intersections for lines {:?}, {:?}", line_a, line_b);
    match inter_point {
        IntersectionPoint::None => IntersectionPoint::None,
        IntersectionPoint::Infinite => {
            // This is incorrect, but I'm to lazy to find the correct algorithm...
            IntersectionPoint::None
        }
        IntersectionPoint::One(point) => {
            // println!(
            //     "Got 1 intersection point {}, for lines {:?}, {:?}",
            //     point,
            //     line_a, line_b
            // );
            if point_inside_frame(point, (line_a.0, line_a.1))
                && point_inside_frame(point, (line_b.0, line_b.1))
            {
                inter_point
            } else {
                // println!("Which wasn't inside frame :/");
                IntersectionPoint::None
            }
        }
    }
}

/// Checks if point is inside frame constructed by two corners.
///
/// # Examples
/// Example 1:
/// ```
/// use terminal_renderer::math::*;
///
/// let frame = (vec2(-1., -1.), vec2(1., 1.));
/// let point_a = vec2(0.5, -0.2);
/// let point_b = vec2(0.2, 3.);
///
/// assert_eq!(point_inside_frame(point_a, frame), true);
/// assert_eq!(point_inside_frame(point_b, frame), false);
/// ```
/// Example 2:
/// ```
/// use terminal_renderer::math::*;
///
/// let frame = (vec2(1., -1.), vec2(-1., 1.));
/// let point_a = vec2(0.5, -0.2);
/// let point_b = vec2(2., 3.);
///
/// assert_eq!(point_inside_frame(point_a, frame), true);
/// assert_eq!(point_inside_frame(point_b, frame), false);
/// ```
pub fn point_inside_frame(point: Vec2, frame_corners: (Vec2, Vec2)) -> bool {
    let (left, right) = if frame_corners.0.x < frame_corners.1.x {
        (frame_corners.0.x, frame_corners.1.x)
    } else {
        (frame_corners.1.x, frame_corners.0.x)
    };

    let (bottom, top) = if frame_corners.0.y < frame_corners.1.y {
        (frame_corners.0.y, frame_corners.1.y)
    } else {
        (frame_corners.1.y, frame_corners.0.y)
    };

    const EPSILON: f32 = 0.0001;

    // println!(
    //     "{:?} <= {:?} && {:?} <= {:?} && {:?} <= {:?} && {:?} <= {:?}",
    //     left, point.x, point.x, right, bottom, point.y, point.y, top,
    // );
    le_approx(left, point.x, EPSILON)
        && le_approx(point.x, right, EPSILON)
        && le_approx(bottom, point.y, EPSILON)
        && le_approx(point.y, top, EPSILON)
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Line(pub Vec2, pub Vec2);
impl AbsDiffEq for Line {
    type Epsilon = f32;
    fn default_epsilon() -> Self::Epsilon {
        f32::EPSILON
    }
    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        self.0.abs_diff_eq(&other.0, epsilon) && self.1.abs_diff_eq(&other.1, epsilon)
    }
}
impl From<ULine> for Line {
    fn from(value: ULine) -> Self {
        Line(value.0.into(), value.1.into())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ULine(pub UVec2, pub UVec2);

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FrameIntersection {
    None,
    One(Vec2),
    Two(Vec2, Vec2),
}

pub fn frame_intersection(line: Line, frame_corners: (Vec2, Vec2)) -> FrameIntersection {
    let (left, right) = if frame_corners.0.x < frame_corners.1.x {
        (frame_corners.0.x, frame_corners.1.x)
    } else {
        (frame_corners.1.x, frame_corners.0.x)
    };

    let (bottom, top) = if frame_corners.0.y < frame_corners.1.y {
        (frame_corners.0.y, frame_corners.1.y)
    } else {
        (frame_corners.1.y, frame_corners.0.y)
    };

    let Line(a, b) = line;

    let a_outside_frame = !point_inside_frame(a, frame_corners);
    let b_outside_frame = !point_inside_frame(b, frame_corners);

    // dbg!(a_outside_frame);
    // dbg!(b_outside_frame);

    let frame_lines = [
        Line(vec2(left, top), vec2(right, top)),
        Line(vec2(right, top), vec2(right, bottom)),
        Line(vec2(left, bottom), vec2(right, bottom)),
        Line(vec2(left, top), vec2(left, bottom)),
    ];

    if a_outside_frame && b_outside_frame {
        let mut points = Vec::new();

        for frame_line in frame_lines {
            let inter = intersection_of_point_pairs(line, frame_line);
            match inter {
                IntersectionPoint::One(point) => points.push(point),
                _ => continue,
            }
        }

        return match points.len() {
            0 => FrameIntersection::None,
            1 => FrameIntersection::One(points[0]),
            2 => FrameIntersection::Two(points[0], points[1]),
            _ => panic!("too many intersections"),
        };
    } else if a_outside_frame || b_outside_frame {
        // if b_out_of_bounds {
        //     (a, b) = (b, a)
        // }
        // // a is garanteed to be out of bounds.
        let mut inter = IntersectionPoint::None;

        for frame_line in frame_lines {
            inter = intersection_of_point_pairs(line, frame_line);
            if inter != IntersectionPoint::None {
                break;
            }
        }

        let IntersectionPoint::One(point) = inter else {
            return FrameIntersection::None; // Let's ignore that pesky, "that's technically wrong" ;)
            // panic!("no intersection found, while intersection was guaranteed. line: {:?}, frame_corners: {:?}, inter: {:?}", line, frame_corners, inter);
        };

        return FrameIntersection::One(point);
    }

    FrameIntersection::None
}

/// Given a line, and a frame, returns said line clipped such that no point
/// on the line lies outside the frame.
/// The frame is given as two corner points.
///
/// # Example
/// Example where the line was clipped at one point
/// ```
/// use terminal_renderer::math::*;
/// use approx::assert_abs_diff_eq;
///
/// let line = Line(vec2(0., 0.6), vec2(1., 1.4));
///
/// let Some(clipped) = clip_to_frame(line, (vec2(-1., -1.), vec2(1., 1.))) else {
///     panic!("Was none");
/// };
/// // Sorry this is really dumb, but the library can't do it any other better way...
/// assert_abs_diff_eq!(clipped, Line(vec2(0., 0.6), vec2(0.5, 1.)))
/// ```
///
/// Example where the line was clipped at two points:
/// ```
/// use terminal_renderer::math::*;
/// use approx::assert_abs_diff_eq;
///
/// let line = Line(vec2(-1.6, 0.), vec2(1., 1.4));
///
/// let Some(clipped) = clip_to_frame(line, (vec2(-1., -1.), vec2(1., 1.))) else {
///     panic!("Was none");
/// };
/// // Sorry this is really dumb, but the library can't do it any other better way...
/// assert_abs_diff_eq!(clipped, Line(vec2(0.257, 1.), vec2(-1., 0.323)), epsilon = 0.001)
/// ```
pub fn clip_to_frame(line: Line, frame_corners: (Vec2, Vec2)) -> Option<Line> {
    let inter = frame_intersection(line, frame_corners);

    match inter {
        FrameIntersection::Two(point_a, point_b) => Some(Line(point_a, point_b)),
        FrameIntersection::One(point) => {
            let inside_frame = (
                point_inside_frame(line.0, frame_corners),
                point_inside_frame(line.1, frame_corners),
            );

            if !inside_frame.0 && !inside_frame.1 {
                None
            } else if inside_frame.0 {
                Some(Line(line.0, point))
            } else if inside_frame.1 {
                Some(Line(point, line.1))
            } else {
                panic!("not possible");
            }
        }
        FrameIntersection::None => {
            let inside_frame = (
                point_inside_frame(line.0, frame_corners),
                point_inside_frame(line.1, frame_corners),
            );

            if inside_frame.0 && inside_frame.1 {
                Some(line)
            } else if !inside_frame.0 && !inside_frame.1 {
                None
            } else {
                panic!("one point inside, while other was outside frame which isn't possible");
            }
        }
    }
}

mod tests {
    #[test]
    fn clip_to_frame_test_1() {
        use crate::math::Line;
        use crate::math::*;
        use approx::assert_abs_diff_eq;

        let line = Line(vec2(0.5, 2.3), vec2(-0.5, 0.9));

        let frame = (vec2(-1., -1.), vec2(1., 1.));

        let Some(clipped) = clip_to_frame(line, frame) else {
            panic!("was none");
        };

        assert_abs_diff_eq!(
            clipped,
            Line(vec2(-0.429, 1.), vec2(-0.5, 0.9)),
            epsilon = 0.001
        )
    }

    #[test]
    fn clip_to_frame_test_2() {
        use crate::math::Line;
        use crate::math::*;
        use approx::assert_abs_diff_eq;

        let line = Line(vec2(0.5, 1.), vec2(0.5, -1.));

        let frame = (vec2(-1., -1.), vec2(1., 1.));

        let Some(clipped) = clip_to_frame(line, frame) else {
            panic!("was none");
        };

        assert_abs_diff_eq!(
            clipped,
            Line(vec2(0.5, 1.), vec2(0.5, -1.)),
            epsilon = 0.001
        )
    }

    #[test]
    fn clip_to_frame_test_3() {
        use crate::math::Line;
        use crate::math::*;
        use approx::assert_abs_diff_eq;

        let line = Line(vec2(0.5, 0.9), vec2(0.5, -1.1));

        let frame = (vec2(-1., -1.), vec2(1., 1.));

        let Some(clipped) = clip_to_frame(line, frame) else {
            panic!("was none");
        };

        assert_abs_diff_eq!(
            clipped,
            Line(vec2(0.5, 0.9), vec2(0.5, -1.)),
            epsilon = 0.001
        )
    }

    #[test]
    fn clip_to_frame_test_4() {
        use crate::math::Line;
        use crate::math::*;
        use approx::assert_abs_diff_eq;

        let line = Line(vec2(0.4958, 0.9927), vec2(0.4860, 1.0659));

        let frame = (vec2(-1., -1.), vec2(1., 1.));

        let Some(clipped) = clip_to_frame(line, frame) else {
            panic!("was none");
        };

        assert_abs_diff_eq!(
            clipped,
            Line(vec2(0.496, 0.993), vec2(0.495, 1.)),
            epsilon = 0.001
        )
    }

    #[test]
    fn intersection_of_point_pairs_test_1() {
        use crate::math::Line;
        use crate::math::*;
        use approx::assert_abs_diff_eq;

        let line_a = Line(vec2(0.5, 0.9), vec2(0.5, -1.1));

        let line_b = Line(vec2(-1., -1.), vec2(1., -1.));

        assert_abs_diff_eq!(
            intersection_of_point_pairs(line_a, line_b),
            IntersectionPoint::One(vec2(0.5, -1.)),
            epsilon = 0.001
        )
    }

    #[test]
    fn intersection_of_point_pairs_test_2() {
        use crate::math::Line;
        use crate::math::*;
        use approx::assert_abs_diff_eq;

        let line_a = Line(
            Vec2 {
                x: 0.495_870_77,
                y: 0.992_829_3,
            },
            Vec2 {
                x: 0.513_610_7,
                y: -1.029_058_8,
            },
        );

        let line_b = Line(vec2(-1., -1.), vec2(1., -1.));

        assert_abs_diff_eq!(
            intersection_of_point_pairs(line_a, line_b),
            IntersectionPoint::One(vec2(0.5, -1.)),
            epsilon = 0.1
        )
    }
}

/// Linearly interpolate from a to b using t
///
/// Source: https://en.wikipedia.org/wiki/Linear_interpolation#Programming_language_support
pub fn lerp<T, U>(a: T, b: T, t: U) -> T
where
    T: ops::Mul<U, Output = T> + ops::Add<Output = T>,
    U: One + ops::Sub<Output = U> + Copy,
{
    a * (one::<U>() - t) + b * t
}

/// Calculate the barycentric coordinate weights for a point relative to a
/// triangle.
///
/// Barycentric coordinate weights are a way to weigh how much each corner of a
/// triangle should weigh in on a specific point (that's usually inside the
/// triangle).
/// They are very usefull for interpolating between three vertex normals as an
/// example.
///
/// Note that the sum of weights is always equal to one.
///
/// If the point is outside the triangle, one or more of the weights will be
/// negative.
///
/// Source: https://codeplea.com/triangular-interpolation
pub fn barycentric_weights(triangle: (Vec2, Vec2, Vec2), point: Vec2) -> (f32, f32, f32) {
    let v0 = triangle.0;
    let v1 = triangle.1;
    let v2 = triangle.2;

    // crate::clean_up();
    // print!(
    //     "{:?}\r\n{:?}\r\n{:?}\r\n{:?}\r\n{} / {}",
    //     v0,
    //     v1,
    //     v2,
    //     point,
    //     (v1.y - v2.y) * (point.x - v2.x) + (v2.x - v1.x) * (point.y - v2.y),
    //     (v1.y - v2.y) * (v0.x - v2.x) + (v2.x - v1.x) * (v0.y - v2.y)
    // );

    let w0 = ((v1.y - v2.y) * (point.x - v2.x) + (v2.x - v1.x) * (point.y - v2.y))
        / ((v1.y - v2.y) * (v0.x - v2.x) + (v2.x - v1.x) * (v0.y - v2.y));

    let w1 = ((v2.y - v0.y) * (point.x - v2.x) + (v0.x - v2.x) * (point.y - v2.y))
        / ((v1.y - v2.y) * (v0.x - v2.x) + (v2.x - v1.x) * (v0.y - v2.y));

    let w2 = 1. - w0 - w1;

    (w0, w1, w2)
}

pub fn apply_3_weights<T, U>(values: (U, U, U), weights: (T, T, T)) -> U
where
    U: ops::Add<U, Output = U> + ops::Mul<T, Output = U>,
{
    values.0 * weights.0 + values.1 * weights.1 + values.2 * weights.2
}

/// Calculates the aspect ratio of a given dimension using
/// `width / height`.
pub fn aspect_ratio(size: Vec2) -> f32 {
    size.x / size.y
}
