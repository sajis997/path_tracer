use crate::aabb::Aabb;
use crate::hit::HitRecord;

pub enum SplitMethod {
    SAH,
    HLBVH,
    Middle,
    EqualCounts,
}

pub struct BVHPrimitiveInfo {
    primitive_index: usize,
    bounds: Bounds3f,
    centroid: Point3f,
}

impl BVHPrimitiveInfo {
    pub fn new(primitive_number: usize, bounds: Aabb) -> Self {
        BVHPrimitiveInfo {
            primitive_index: primitive_number,
            bounds,
            centroid: bounds.min() * 0.5 + bounds.max() * 0.5,
        }
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

impl BvhNode {
    pub fn new(aabb: Aabb) -> Self {
        Self {
            aabb,
            primitives_per_node: 0,
            index: -1,
            split_axis: -1,
            leaf: false,
        }
    }

    pub fn makeLeaf(index: usize, primitives_per_node: usize){
        this.index = index;
        this.primitives_per_node = primitives_per_node;
        this.leaf = true;
    }

    pub fn makeInterior(index: usize, split_axis: usize){

        // index into the right child in the node list  
        this.index = index;
        // the coordianate axis along which the node is split
        this.split_axis = split_axis;
        this.leaf = false;
    }
}

pub struct Bvh {
    nodes: Vec<BvhNode>,
}