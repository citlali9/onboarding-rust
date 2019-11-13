use std::collections::HashSet;

pub fn find_intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
   let mut vec: Vec<i32> = nums1
                           .into_iter()
                           .filter(|x| nums2.contains(x))
                           .collect::<HashSet<i32>>()
                           .into_iter()
                           .collect::<Vec<i32>>();
   vec.sort();
   vec
}