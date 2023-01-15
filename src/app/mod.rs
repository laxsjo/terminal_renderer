use crate::math::*;
use crate::render_3d::*;
use crate::test_data;

pub fn init_scene() -> Scene {
    let mut scene = Scene::new();

    let debug_ref = scene.add_object(Object::new(
        ObjMeshLoader::load(test_data::SUZANNE_SMOOTH_OBJ_FILE).expect("failed to load mesh"),
        Transform::identity(),
        hsl(0.5, 1., 0.5).into(),
        // rgb(1., 0., 0.),
    ));

    {
        let debug_object = scene.get_object_mut(debug_ref).unwrap();
        debug_object
            .transform
            .rotate_mut(Quaternion::from_euler_angles(
                25.0.to_radians(),
                45.0.to_radians(),
                0.,
            ));
    }

    scene.camera = OrthographicCamera::new(Transform::identity(), 8., 8., 100., 0.01);

    scene.camera.position += vec3(0., 0., 2.);

    scene
}
