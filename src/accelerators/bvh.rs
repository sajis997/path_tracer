use crate::ray::Ray;
use crate::utils::aabb::Aabb;
use crate::utils::util::Point3;
use crate::hit::HitRecord;
use crate::hit::World;
use crate::accelerators::accelerator::Accelerator;
use crate::utils::stack::Stack;
use rayon::prelude::*;

pub enum SplitMethod {
    //SAH,
    //HLBVH,
    Middle,
    //EqualCounts,
}


pub struct BVHPrimitiveInfo {
    primitive_index: usize,
    bounds: Aabb,
    centroid: Point3,
}

impl BVHPrimitiveInfo {
    pub fn new(primitive_number: usize, bounds: Aabb, center: Point3) -> Self {
        BVHPrimitiveInfo {
            primitive_index: primitive_number,
            bounds,
            centroid: center,
        }
    }

    // get the bounding box of the primitive
    pub fn bounding_box(&self) -> Aabb {
        self.bounds.clone()
    }

    // get the centroid of the primitive
    pub fn centroid(&self) -> Point3 {
        self.centroid
    }
}


pub struct BvhNode {
    aabb: Aabb, // bounding box of the node
    primitives_per_node: usize, // number of primitives in the node
    // if leaf node, index into the primitives array
    // if interior node, index into the right child in the node list
    // as the left child is be default placed right after the parent 
    // node in the list
    index: usize,
    split_axis: usize, // axis along which the node is split
    leaf: bool, // boolean flag to mark if it is a leaf node or not
}

impl Clone for BvhNode {
    fn clone(&self) -> Self {
        Self {
            aabb: self.aabb.clone(), // assuming aabb implements Clone
            primitives_per_node: self.primitives_per_node,
            index: self.index,
            split_axis: self.split_axis,
            leaf: self.leaf,
        }
    }
}


impl BvhNode {
    pub fn new(aabb: Aabb) -> Self {
        Self {
            aabb,
            primitives_per_node: 0,
            index: usize::MAX,
            split_axis: usize::MAX,
            leaf: false,
        }
    }

    pub fn makeInterior(&mut self, index: usize, split_axis: usize){

        // index into the right child in the node list  
        self.index = index;
        // the coordianate axis along which the node is split
        self.split_axis = split_axis;
        self.leaf = false;
    }

    pub fn makeLeaf(&mut self, index: usize, num_primitives: usize){
        // index into the primitives array
        self.index = index;
        self.primitives_per_node = num_primitives;
        self.leaf = true;
    }

    pub fn isLeaf(&self) -> bool {
        self.leaf
    } 

    // get the AABB of the node
    pub fn bounding_box(&self) -> Aabb {
        self.aabb.clone()
    } 

    pub fn setNumPrimitives(&mut self, num_primitives: usize){
        self.primitives_per_node = num_primitives;
    } 
}

pub struct Bvh {
    nodes: Vec<Box<BvhNode>>,
    primitives: Vec<Box<BVHPrimitiveInfo>>,
    max_primitives_per_node: usize,
    split_method: SplitMethod,
    parent_stack: Stack<usize>,
}

impl Bvh {
    pub fn new(maxPrimitivesPerNode: Option<usize>,
               splitMethod: Option<SplitMethod>) -> Self {
        Self {
            nodes: Vec::new(),
            primitives: Vec::new(),
            max_primitives_per_node: maxPrimitivesPerNode.unwrap_or_else(|| 4),
            split_method: splitMethod.unwrap_or_else(|| SplitMethod::Middle),
            parent_stack: Stack::new(),
        }
    }

    fn build_recursive(&mut self, left_index: usize, right_index: usize, mut depth: usize) -> usize {

        // get the bounding box of the primitives in between 
        // left_index and right_index inclusive
        let mut aabb = Aabb::empty();
        for i in left_index..right_index {
            aabb = aabb.include(&self.primitives[i].bounding_box());
        }

        // get the number of primitives between left_index and right_index
        let num_primitives : usize = (right_index - left_index) as usize;

        // initiate a bvhnode with the expanded/merged bounding box
        let node = Box::new(BvhNode::new(aabb));

        // move the node into the nodes list
        self.nodes[depth] = node;

        // set the number of primitives in the node to num_primitives
        self.nodes[depth].setNumPrimitives(num_primitives);

        // if the number of primitives is less than the maximum number of primitives per node
        // then make the node a leaf node
        if num_primitives <= self.max_primitives_per_node {
            self.nodes[depth].makeLeaf(left_index, num_primitives);
        }
        else {
                    
            // get the largest axis of the bounding box
            // between left_index and right_index
            let largest_axis = self.nodes[depth].bounding_box().largest_axis() as usize;

            if self.nodes[depth].bounding_box().max()[largest_axis] - 
               self.nodes[depth].bounding_box().min()[largest_axis] < f32::EPSILON {
                self.nodes[depth].makeLeaf(left_index, num_primitives);
                return depth;
            }

            // get the split index
            let mut split_index = left_index + right_index / 2;

            match self.split_method {
                SplitMethod::Middle => {
                    // sort the primitives along the largest axis
                    self.primitives[left_index..right_index].sort_by(|a, b| {
                        a.centroid()[largest_axis as usize].partial_cmp(&b.centroid()[largest_axis as usize]).unwrap()
                    });

                    // get the middle index of the sorted primitives
                    split_index = (left_index + right_index) / 2;
                                                  
                }
                _ => {
                    // Handle all other cases
                }
            }

            self.parent_stack.push(depth);

            // build the left child
            depth = self.build_recursive(left_index, split_index, depth + 1); 

            let top = *(self.parent_stack.peek().unwrap());

            self.parent_stack.pop();

            // initiate the node as the right child of the current/parent node
            self.nodes[top].makeInterior(depth + 1, largest_axis);

            depth = self.build_recursive(split_index, right_index, depth + 1);       

        }

        depth
    }
}

impl Accelerator for Bvh {
    fn build(&mut self, world: &World) {

        // find bounding box and centroids of all intersectable objects in world
        // and store them in a vector of BVHPrimitiveInfo       
        let primitives: Vec<_> = world.par_iter()
            .enumerate()
            .filter_map(|(i, object)| {
                if let Some(aabb) = object.bounding_box() {
                    Some(Box::new(BVHPrimitiveInfo::new(i, aabb, (*object).centroid())))
                } else {
                    None
                }
            })
            .collect();

        self.primitives = primitives;

        // store the primitives in the bvh        
        self.nodes.resize(2 * self.primitives.len() - 1, Box::new(BvhNode::new(Aabb::empty())));

        // build the bvh tree recursively
        let depth = self.build_recursive(0, self.primitives.len(), 0);

        println!("BVH node size before cleanup: {}", self.nodes.len());

        // remove empty nodes
        self.nodes.retain(|node| !node.aabb.is_empty());

        println!("BVH node size after cleanup: {}", self.nodes.len());
    }

    fn intersect(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        // Implement the intersect method here.
        // Add your code here.
        None
    }

}