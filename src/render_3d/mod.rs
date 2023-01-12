use crate::ansi_term::*;
use crate::math::*;
use crate::referenceable_vec::*;
use crate::ui;
use crate::utils::AnyIter;
use crate::utils::DeltaTimer;
use crate::utils::VecUtils;
use std::collections::BTreeSet;
use std::fmt::Debug;
use std::fmt::Display;

mod buffer;
mod camera;
mod color;
mod drawers;
pub mod mesh_loader;
mod panel;
mod quaternion;
mod renderer;
mod scene;
pub mod shader;
mod transform;
pub mod shader_fn;

pub use buffer::*;
pub use camera::*;
pub use color::*;
pub use drawers::*;
pub use mesh_loader::*;
pub use panel::*;
pub use quaternion::*;
pub use renderer::*;
pub use scene::*;
pub use transform::*;

pub struct RefTriangle(pub Index<Vec3>, pub Index<Vec3>, pub Index<Vec3>);
impl RefTriangle {
    pub const fn new(indices: (usize, usize, usize)) -> Self {
        Self(
            Index::new(indices.0),
            Index::new(indices.1),
            Index::new(indices.2),
        )
    }
}

#[derive(Debug)]
pub struct Triangle {
    pub points: (Vec3, Vec3, Vec3),
    pub normals: (Vec3, Vec3, Vec3),
    pub vertex_colors: (Rgb, Rgb, Rgb),
}

impl Triangle {
    pub fn new(
        vertices: &[Vec3],
        ref_triangle: &RefTriangle,
        normals: (Vec3, Vec3, Vec3),
        vertex_colors: (Rgb, Rgb, Rgb),
    ) -> Option<Self> {
        Some(Self {
            points: (
                *vertices.get_with(&ref_triangle.0)?,
                *vertices.get_with(&ref_triangle.1)?,
                *vertices.get_with(&ref_triangle.2)?,
            ),
            normals,
            vertex_colors,
        })
    }

    pub fn normal(&self) -> Vec3 {
        (self.points.1 - self.points.0)
            .cross_product(self.points.2 - self.points.0)
            .normalize()
    }

    pub fn transform(self, transform: &Transform) -> Self {
        Self {
            points: (
                transform.transform_point(self.points.0),
                transform.transform_point(self.points.1),
                transform.transform_point(self.points.2),
            ),
            normals: (
                transform.transform_point(self.normals.0),
                transform.transform_point(self.normals.1),
                transform.transform_point(self.normals.2),
            ),
            vertex_colors: self.vertex_colors,
        }
    }
}

impl Display for Triangle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO: make this also display the normals
        f.write_str("{")?;

        Display::fmt(&self.points.0, f)?;
        f.write_str(", ")?;
        Display::fmt(&self.points.1, f)?;
        f.write_str(", ")?;
        Display::fmt(&self.points.2, f)?;

        f.write_str("}")
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct RefEdge(pub Index<Vec3>, pub Index<Vec3>);
impl RefEdge {
    pub const fn new(indices: (usize, usize, usize)) -> Self {
        Self(Index::new(indices.0), Index::new(indices.1))
    }
}

pub struct Edge(pub Vec3, pub Vec3);

impl Edge {
    pub fn new(vertices: &[Vec3], ref_edge: &RefEdge) -> Option<Self> {
        Some(Self(
            *vertices.get_with(&ref_edge.0)?,
            *vertices.get_with(&ref_edge.1)?,
        ))
    }

    pub fn transform(self, transform: &Transform) -> Self {
        Self(
            transform.transform_point(self.0),
            transform.transform_point(self.1),
        )
    }
}

pub struct Mesh {
    vertices: Vec<Vec3>,
    triangles: Vec<RefTriangle>,
    normals: Vec<(Vec3, Vec3, Vec3)>,
    vertex_colors: Vec<(Rgb, Rgb, Rgb)>,
    edge_cache: Vec<RefEdge>,
}

impl Debug for Mesh {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Mesh(<mesh_data>)")
    }
}

impl Mesh {
    fn build_edges(triangles: &Vec<RefTriangle>) -> Vec<RefEdge> {
        let mut edges = BTreeSet::new();
        // let edges = Vec::new();

        fn order_edge(edge: &mut RefEdge) {
            if edge.0 > edge.1 {
                std::mem::swap(&mut edge.0, &mut edge.1);
            }
        }

        fn add_edge(mut edge: RefEdge, edge_list: &mut BTreeSet<RefEdge>) {
            order_edge(&mut edge);
            edge_list.insert(edge);
        }

        for triangle in triangles {
            add_edge(RefEdge(triangle.0, triangle.1), &mut edges);
            add_edge(RefEdge(triangle.0, triangle.2), &mut edges);
            add_edge(RefEdge(triangle.1, triangle.2), &mut edges);
        }

        edges.into_iter().collect()
    }

    pub fn new(
        vertices: Vec<Vec3>,
        triangles: Vec<RefTriangle>,
        normals: Option<Vec<(Vec3, Vec3, Vec3)>>,
        vertex_colors: Option<Vec<(Rgb, Rgb, Rgb)>>,
    ) -> Self {
        let edge_cache = Self::build_edges(&triangles);

        let has_normals = normals.is_some();

        let normals = match normals {
            Some(normals) => normals,
            None => Vec::new(),
        };

        let vertex_colors = match vertex_colors {
            Some(colors) => colors,
            None => vec![(Rgb::default(), Rgb::default(), Rgb::default()); triangles.len()],
        };

        let mut mesh = Self {
            vertices,
            triangles,
            normals,
            vertex_colors,
            edge_cache,
        };

        if !has_normals {
            mesh.re_build_normals();
        }

        mesh
    }

    pub fn re_build_normals(&mut self) {
        self.normals = self
            .triangles_iter()
            .map(|tri| {
                let normal = tri.normal();
                (normal, normal, normal)
            })
            .collect();
    }

    // pub fn verts(&self) -> &[Vec3] {
    //     self.vertices.as_ref()
    // }

    // pub fn triangles(&self) -> &Vec<RefTriangle> {
    //     &self.triangles
    // }

    pub fn get_vert(&self, vert: Index<Vec3>) -> Option<&Vec3> {
        self.vertices.get_with(&vert)
    }

    pub fn get_vert_mut(&mut self, vert: Index<Vec3>) -> Option<&mut Vec3> {
        self.vertices.get_with_mut(&vert)
    }

    pub fn verts_iter(&self) -> std::slice::Iter<Vec3> {
        self.vertices.iter()
    }

    pub fn verts_iter_mut(&mut self) -> std::slice::IterMut<Vec3> {
        self.vertices.iter_mut()
    }

    pub fn edges_iter(&self) -> AnyIter<Edge> {
        AnyIter::new(
            self.edge_cache
                .iter()
                .map(|ref_edge| Edge::new(&self.vertices, ref_edge).unwrap()),
        )
    }

    pub fn triangles_iter(&self) -> AnyIter<Triangle> {
        AnyIter::new(
            self.triangles
                .iter()
                .zip(self.normals_iter())
                .zip(self.vertex_colors_iter())
                .map(|((ref_triangle, normals), vertex_colors)| {
                    Triangle::new(&self.vertices, ref_triangle, *normals, *vertex_colors).unwrap()
                }),
        )
    }

    pub fn normals_iter(&self) -> std::slice::Iter<(Vec3, Vec3, Vec3)> {
        self.normals.iter()
    }

    pub fn vertex_colors_iter(&self) -> std::slice::Iter<(Rgb, Rgb, Rgb)> {
        self.vertex_colors.iter()
    }
}

#[derive(Debug)]
pub struct Object {
    pub mesh: Mesh,
    pub transform: Transform,
    pub color: Rgb,
}

impl Object {
    pub fn new(mesh: Mesh, transform: Transform, color: Rgb) -> Self {
        Self {
            mesh,
            transform,
            color,
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
pub struct UDimensions {
    pub x: usize,
    pub y: usize,
}

#[derive(PartialEq, PartialOrd, Clone, Copy, Debug)]
pub struct Dimensions {
    pub x: f32,
    pub y: f32,
}

pub const fn udimensions(x: usize, y: usize) -> UDimensions {
    UDimensions { x, y }
}

pub const fn dimensions(x: f32, y: f32) -> Dimensions {
    Dimensions { x, y }
}

pub fn normalized_to_buffer_space(point: Vec2, buffer_size: UDimensions) -> Vec2 {
    Vec2 {
        x: (point.x + 1.) / 2. * buffer_size.x as f32,
        y: (point.y + 1.) / 2. * buffer_size.y as f32,
    }
}

pub fn buffer_to_normalized_space(point: Vec2, buffer_size: UDimensions) -> Vec2 {
    Vec2 {
        x: (2. * point.x) / buffer_size.x as f32 - 1.,
        y: (2. * point.y) / buffer_size.y as f32 - 1.,
    }
}
