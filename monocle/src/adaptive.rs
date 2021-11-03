use std::borrow::{Borrow, BorrowMut};
use std::collections::BinaryHeap;
use std::cmp::Ordering;
use std::rc::Rc;
use ordered_float::OrderedFloat;
use crate::mask::Mask;

#[derive(PartialEq, Eq, Clone)]
enum Quadtree<T> {
    Leaf {
        payload: T,
    },
    Branch {
        children: Box<[Quadtree<T>; 4]>,
        payload: T,
    }
}

impl<T> Quadtree<T> {
    pub fn payload(&self) -> &T {
        match self {
            Quadtree::Leaf { payload } => payload,
            Quadtree::Branch { payload, .. } => payload,
        }
    }

    pub fn payload_mut(&mut self) -> &mut T {
        match self {
            Quadtree::Leaf { payload } => payload,
            Quadtree::Branch { payload, .. } => payload,
        }
    }

    pub fn children(&self) -> Option<&[Quadtree<T>; 4]> {
        match self {
            Quadtree::Leaf { .. } => None,
            Quadtree::Branch { children, .. } => Some(children.borrow()),
        }
    }

    pub fn children_mut(&mut self) -> Option<&mut [Quadtree<T>; 4]> {
        match self {
            Quadtree::Leaf { .. } => None,
            Quadtree::Branch { children, .. } => Some(children.borrow_mut()),
        }
    }

    // pub fn index(&self, key: &Key<T>) -> Option<&Quadtree<T>> {
    //     let mut tree: &Quadtree<T> = &self;
    //     let mut slice: &[u32] = &key.indices[..];
    //     loop {
    //         if let Some((first, rest)) = slice.split_first() {
    //             tree = &(tree.children()?)[*first as usize];
    //             slice = rest;
    //         } else {
    //             break Some(tree);
    //         }
    //     }
    // }
}

#[derive(Clone, PartialEq, Eq)]
struct Key<T> {
    phantom: std::marker::PhantomData<T>,
    indices: Vec<u32>
}

#[derive(PartialEq, Eq, Clone)]
struct Rect {
    bottom_left: (u32, u32),
    size: (u32, u32),
}

impl Rect {
    fn subdivide(&self) -> Option<[Rect; 4]> {
        if (self.size.0 % 2 != 0) || (self.size.1 % 2 != 0) {
            return None;
        }
        let new_size = (self.size.0 / 2, self.size.1 / 2);
        let bottom_left_rect = Rect {
            bottom_left: self.bottom_left,
            size: new_size,
        };
        let bottom_right_rect = Rect {
            bottom_left: (self.bottom_left.0 + new_size.0, self.bottom_left.1),
            size: new_size,
        };
        let top_left_rect = Rect {
            bottom_left: (self.bottom_left.0, self.bottom_left.1 + new_size.1),
            size: new_size,
        };
        let top_right_rect = Rect {
            bottom_left: (self.bottom_left.0 + new_size.0,
                          self.bottom_left.1 + new_size.1),
            size: new_size,
        };
        Some([
            bottom_left_rect,
            bottom_right_rect,
            top_left_rect,
            top_right_rect,
        ])
    }

    fn contains(&self, x: u32, y: u32) -> bool {
        let between = |n: u32, l: u32, u: u32| -> bool { (l <= n) && (n <= u) };
        between(x - self.bottom_left.0, 0, self.size.0 - 1)
            && between(y - self.bottom_left.1, 0, self.size.1 - 1)
    }

    fn to_mask(&self, width: u32, height: u32) -> Mask {
        Mask::from_fn(width, height, |x: u32, y: u32| {
            if self.contains(x, y) {
                image::Luma([255])
            } else {
                image::Luma([0])
            }
        })
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
struct Energy(OrderedFloat<f64>);

#[derive(PartialEq, Eq)]
struct QueueElement {
    node: Key<Energy>,
    rect: Rect,
}

impl Ord for QueueElement {
    fn cmp(&self, other: &Self) -> Ordering {
        todo!() // other.node.payload().cmp(&self.node.payload())
    }
}

impl PartialOrd for QueueElement {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone)]
struct Options {
    width: u32,
    height: u32,
}

trait Adaptive {
    type MeasurementHandle;
    fn new(options: &Options) -> Self;
    fn next(&mut self) -> Option<(Mask, Self::MeasurementHandle)>;
    fn measurement(&mut self, handle: &Self::MeasurementHandle, energy: Energy);
}

// struct QuadtreeSearch<'a> {
//     options: Options,
//     heap: BinaryHeap<QueueElement<'a>>
// }

// fn adaptive(
//     width: u32,
//     height: u32,
//     measure: impl Fn(&Mask) -> f64,
//     stop: impl Fn(&Rect, f64) -> bool,
// ) -> Mask {
//     let mut priority_queue = BinaryHeap::<QueueElement<'_>>::new();
//     let all_white =
//         image::ImageBuffer::from_pixel(width, height, image::Luma([255]));
//     let mut root = Quadtree::Leaf {
//         payload: Energy(OrderedFloat(measure(&all_white))),
//     };
//     priority_queue.push(QueueElement {
//         node: &mut root,
//         rect: Rect {
//             bottom_left: (0, 0),
//             size: (width, height),
//         },
//     });
//     while let Some(best) = priority_queue.pop() {
//         if stop(&best.rect, best.node.payload().0.0) {
//             continue;
//         }
//
//         let subdivided = best.rect.subdivide();
//         if subdivided.is_none() {
//             continue;
//         }
//         let subdivided = subdivided.unwrap();
//
//         let mut children = [
//             Quadtree::Leaf { payload: Energy(OrderedFloat(-1.0)) },
//             Quadtree::Leaf { payload: Energy(OrderedFloat(-1.0)) },
//             Quadtree::Leaf { payload: Energy(OrderedFloat(-1.0)) },
//             Quadtree::Leaf { payload: Energy(OrderedFloat(-1.0)) },
//         ];
//         for (i, rect) in subdivided.iter().enumerate() {
//             *(children[i].payload_mut()) =
//                 Energy(OrderedFloat(measure(&rect.to_mask(width, height))));
//         }
//         *best.node = Quadtree::Branch {
//             payload: *best.node.payload(),
//             children: Box::new(children)
//         };
//         let [ref mut c1, ref mut c2, ref mut c3, ref mut c4] =
//             best.node.children_mut().unwrap();
//         priority_queue.push(QueueElement {
//             node: c1,
//             rect: subdivided[0].clone(),
//         });
//         priority_queue.push(QueueElement {
//             node: c2,
//             rect: subdivided[1].clone(),
//         });
//         priority_queue.push(QueueElement {
//             node: c3,
//             rect: subdivided[2].clone(),
//         });
//         priority_queue.push(QueueElement {
//             node: c4,
//             rect: subdivided[3].clone(),
//         });
//     }
//     todo!()
// }
