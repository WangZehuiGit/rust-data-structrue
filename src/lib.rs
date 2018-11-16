pub mod vector;

pub trait PriorityQueue<V: std::cmp::PartialOrd> {
    fn insert(&mut self, value: &V);
    fn size(&self) -> usize;
    fn del_max(&mut self) -> V;
    fn get_max(&self) -> &V;
}
