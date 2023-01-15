use crate::utils::AnyIter;

use super::*;

#[derive(Debug, Clone, Copy)]
pub struct ObjectId {
    index: usize,
    id: u32,
}
#[derive(Debug)]
pub struct Scene {
    pub camera: OrthographicCamera,
    pub light_direction: Vec3,
    objects: Vec<(u32, Object)>,
    next_id: u32,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            camera: OrthographicCamera::new(Transform::identity(), 1., 1., 1., 0.1),
            light_direction: vec3(0., 0., -1.),
            objects: Vec::new(),
            next_id: 0,
        }
    }

    pub fn add_object(&mut self, object: Object) -> ObjectId {
        let index = self.objects.len();
        self.objects.push((self.next_id, object));

        let id = self.next_id;
        self.next_id += 1;

        ObjectId { index, id }
    }

    pub fn get_object(&self, id: ObjectId) -> Option<&Object> {
        let item = self.objects.get(id.index)?;
        if item.0 != id.id {
            return None;
        }

        Some(&item.1)
    }

    pub fn get_object_mut(&mut self, id: ObjectId) -> Option<&mut Object> {
        let item = self.objects.get_mut(id.index)?;
        if item.0 != id.id {
            return None;
        }

        Some(&mut item.1)
    }

    pub fn objects(&self) -> AnyIter<&Object> {
        let iter = self.objects.iter().map(|(_, object)| object);

        AnyIter::new(iter)
    }

    pub fn object_refs(&self) -> AnyIter<ObjectId> {
        let iter = self
            .objects
            .iter()
            .enumerate()
            .map(|(index, (id, _))| ObjectId { index, id: *id });

        AnyIter::new(iter)
    }
}

impl Default for Scene {
    fn default() -> Self {
        Self::new()
    }
}
