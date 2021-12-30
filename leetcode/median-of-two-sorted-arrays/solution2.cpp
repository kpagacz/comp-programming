// link to the problem:
// https://leetcode.com/problems/median-of-two-sorted-arrays/
#include <iostream>
#include <numeric>
#include <vector>

class Solution {
 public:
  double findMedianSortedArrays(std::vector<int>& nums1, std::vector<int>& nums2) {
    if (nums1.size() > nums2.size()) return findMedianSortedArrays(nums2, nums1);

    const int total_size = nums1.size() + nums2.size();
    int min_bound = 0, max_bound = nums1.size();

    int left1, right1, left2, right2;
    int partition1, partition2;
    while (min_bound <= max_bound) {
      partition1 = (max_bound + min_bound) / 2;
      partition2 = (total_size + 1) / 2 - partition1;
      if (partition1 == 0) left1 = INT_MIN;
      else left1 = nums1[partition1 - 1];
      if (partition1 == nums1.size()) right1 = INT_MAX;
      else right1 = nums1[partition1];
      if (partition2 == 0) left2 = INT_MIN;
      else left2 = nums2[partition2 - 1];
      if (partition2 == nums2.size()) right2 = INT_MAX;
      else right2 = nums2[partition2];

      if (left1 <= right2 && left2 <= right1) {
        if (total_size % 2 == 0) return 1.0 * (std::max(left1, left2) + std::min(right1, right2)) / 2;
        else return std::max(left1, left2);
      } else {
        if (left1 > right2) {
          max_bound = partition1 - 1;
        } else {
          min_bound = partition1 + 1;
        }
      }
    }
    return 0;
  }
};

int main() {
  Solution s;
  std::vector<int> v1{3};
  std::vector<int> v2{1, 2};
  std::cout << s.findMedianSortedArrays(v1, v2);
};
