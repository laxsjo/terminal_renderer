use crate::math::*;
use crate::referenceable_vec::*;
use crate::render_3d::*;

//  1.000000  1.000000 -1.000000
//  1.000000 -1.000000 -1.000000
//  1.000000  1.000000  1.000000
//  1.000000 -1.000000  1.000000
// -1.000000  1.000000 -1.000000
// -1.000000 -1.000000 -1.000000
// -1.000000  1.000000  1.000000
// -1.000000 -1.000000  1.000000

pub const CUBE_OBJ_FILE: &[u8] = include_bytes!("../assets/cube.obj");
pub const SUZANNE_OBJ_FILE: &[u8] = include_bytes!("../assets/suzanne.obj");
pub const SUZANNE_SMOOTH_OBJ_FILE: &[u8] = include_bytes!("../assets/suzanne_smooth.obj");

pub fn mesh_cube() -> Mesh {
    let vertices = vec![
        vec3(1., 1., -1.),
        vec3(1., -1., -1.),
        vec3(1., 1., 1.),
        vec3(1., -1., 1.),
        vec3(-1., 1., -1.),
        vec3(-1., -1., -1.),
        vec3(-1., 1., 1.),
        vec3(-1., -1., 1.),
    ];
    let triangles = vec![
        RefTriangle(Index::new(4), Index::new(2), Index::new(0)),
        RefTriangle(Index::new(2), Index::new(7), Index::new(3)),
        RefTriangle(Index::new(6), Index::new(5), Index::new(7)),
        RefTriangle(Index::new(1), Index::new(7), Index::new(5)),
        RefTriangle(Index::new(0), Index::new(3), Index::new(1)),
        RefTriangle(Index::new(4), Index::new(1), Index::new(5)),
        RefTriangle(Index::new(4), Index::new(6), Index::new(2)),
        RefTriangle(Index::new(2), Index::new(6), Index::new(7)),
        RefTriangle(Index::new(6), Index::new(4), Index::new(5)),
        RefTriangle(Index::new(1), Index::new(3), Index::new(7)),
        RefTriangle(Index::new(0), Index::new(2), Index::new(3)),
        RefTriangle(Index::new(4), Index::new(0), Index::new(1)),
    ];

    // let edges = vec![
    //     (Index::new(0), Index::new(1)),
    //     (Index::new(0), Index::new(2)),
    //     (Index::new(0), Index::new(4)),
    //     (Index::new(1), Index::new(3)),
    //     (Index::new(1), Index::new(5)),
    //     (Index::new(2), Index::new(3)),
    //     (Index::new(2), Index::new(6)),
    //     (Index::new(3), Index::new(7)),
    //     (Index::new(4), Index::new(5)),
    //     (Index::new(4), Index::new(6)),
    //     (Index::new(5), Index::new(7)),
    //     (Index::new(6), Index::new(7)),
    // ];

    Mesh::new(vertices, triangles, None, None)
}

pub fn mesh_plane() -> Mesh {
    let vertices = vec![
        vec3(1., 1., 0.),
        vec3(1., -1., 0.),
        vec3(-1., -1., 0.),
        vec3(-1., 1., 0.),
    ];
    let triangles = vec![
        RefTriangle(Index::new(0), Index::new(3), Index::new(2)),
        RefTriangle(Index::new(0), Index::new(2), Index::new(1)),
    ];

    Mesh::new(vertices, triangles, None, None)
}

// Source: https://ascii.co.uk/art/alligator
pub const ALLIGATOR_ART: &str = "            .-._   _ _ _ _ _ _ _ _\r
 .-''-.__.-'00  '-' ' ' ' ' ' ' ' '-.\r
'.___ '    .   .--_'-' '-' '-' _'-' '._\r
 V: V 'vv-'   '_   '.       .'  _..' '.'.\r
   '=.____.=_.--'   :_.__.__:_   '.   : :\r
           (((____.-'        '-.  /   : :\r
 snd                         (((-'\\ .' /\r
                           _____..'  .'\r
                          '-._____.-'\r
";
