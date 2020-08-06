use crate::obj::{surrounding_box, AABB};
use crate::trace::{Hit, Hittable, Ray};
use crate::utils::rng::uniform_in_range;
use std::rc::Rc;

pub struct BVHNode {
    bbox: AABB,
    left: Rc<dyn Hittable>,
    right: Rc<dyn Hittable>,
}

impl BVHNode {
    pub fn new(elements: Vec<Rc<dyn Hittable>>, time_begin: f32, time_end: f32) -> BVHNode {
        let mut elements = elements;

        let axis = (3.0 * uniform_in_range(0.0, 1.0)) as i32;
        if axis == 0 {
            elements.sort_by(|h1, h2| {
                let box1 = h1
                    .bounding_box(0.0, 0.0);
                let box2 = h2
                    .bounding_box(0.0, 0.0);
                box1.min().0.partial_cmp(&box2.min().0).unwrap()
            });
        } else if axis == 1 {
            elements.sort_by(|h1, h2| {
                let box1 = h1
                    .bounding_box(0.0, 0.0);
                let box2 = h2
                    .bounding_box(0.0, 0.0);
                box1.min().1.partial_cmp(&box2.min().1).unwrap()
            });
        } else {
            elements.sort_by(|h1, h2| {
                let box1 = h1
                    .bounding_box(0.0, 0.0);
                let box2 = h2
                    .bounding_box(0.0, 0.0);
                box1.min().2.partial_cmp(&box2.min().2).unwrap()
            });
        }

        let (left, right) = if elements.len() == 1 {
            let item = elements.into_iter().next().unwrap();
            (item.clone(), item.clone())
        } else if elements.len() == 2 {
            let mut iter = elements.into_iter();
            (iter.next().unwrap(), iter.next().unwrap())
        } else {
            let len = elements.len();
            let mut iter = elements.into_iter();
            let mut v_left = Vec::new();
            let mut v_right = Vec::new();
            for _ in 0..(len / 2) {
                v_left.push(iter.next().unwrap());
            }
            for _ in (len / 2)..(len - 1) {
                v_right.push(iter.next().unwrap());
            }

            (
                Rc::new(BVHNode::new(v_left, time_begin, time_end)) as Rc<dyn Hittable>,
                Rc::new(BVHNode::new(v_right, time_begin, time_end)) as Rc<dyn Hittable>,
            )
        };

        let box_left = left
            .bounding_box(time_begin, time_end);
        let box_right = right
            .bounding_box(time_begin, time_end);

        let bbox = surrounding_box(box_left, box_right);
        BVHNode { left, right, bbox }
    }
}

impl Hittable for BVHNode {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        if !self.bbox.hit(r, t_min, t_max) {
            return None;
        }

        // both left & right are available
        let left_hit = self.left.hit(r, t_min, t_max);
        let right_hit = self.right.hit(r, t_min, t_max);
        if left_hit.is_some() && right_hit.is_some() {
            let left_hit = left_hit?;
            let right_hit = right_hit?;
            if left_hit.t() < right_hit.t() {
                return Some(left_hit);
            } else {
                return Some(right_hit);
            }
        }

        if left_hit.is_some() {
            return left_hit;
        }
        if right_hit.is_some() {
            return right_hit;
        }
        None
    }

    fn bounding_box(&self, _: f32, _: f32) -> AABB {
        self.bbox.clone()
    }
}
