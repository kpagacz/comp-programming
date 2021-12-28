// link to the problem https://leetcode.com/problems/median-of-two-sorted-arrays/
#include <vector>
#include <iostream>
#include <algorithm>

class Solution
{
public:
  double findMedianSortedArrays(std::vector<int> nums1, std::vector<int> nums2)
  {
    if (nums2.size() > nums1.size())
      std::swap(nums1, nums2);

    if (nums2.size() == 0) {
      return nums1.size() % 2 ? nums1[nums1.size() / 2] : (nums1[nums1.size() / 2] + nums1[nums1.size() / 2 - 1]) / 2;
    }

    // nums2 is the shorter array and has some elements in it
    // invariant in every loop - the number of elements smaller than right1 and right2 is the same - half of the number of all the elements
    int left1 = (nums1.size() + 1) / 2 - 1, right1 = (nums1.size() + 1) / 2;

    std::cout << left1 << " " << right1 << " ";

    return 0;
  }
};

int main() {
  Solution s;
  s.findMedianSortedArrays({1, 2, 3}, {1});
}
