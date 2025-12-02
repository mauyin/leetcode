// 88. Merge Sorted Array
// https://leetcode.com/problems/merge-sorted-array/

fn main() {
    let mut nums1 = vec![1, 2, 3, 0, 0, 0];
    let mut nums2 = vec![2, 5, 6];
    Solution::merge(&mut nums1, 3, &mut nums2, 3);
    println!("{:?}", nums1);
}

struct Solution;

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        // Your solution here
    }
}
