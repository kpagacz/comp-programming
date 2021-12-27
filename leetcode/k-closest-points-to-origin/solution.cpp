// link to the problem https://leetcode.com/problems/k-closest-points-to-origin/
#include<vector>
#include<algorithm>
#include<iostream>

class Solution {
public:
    std::vector<std::vector<int>> kClosest(std::vector<std::vector<int>>& points, int k) {
      std::nth_element(points.begin(), std::next(points.begin(), k), points.end(), [](auto a, auto b) {
        return a[0] * a[0] + a[1] * a[1] < b[0] * b[0] + b[1] * b[1];
      });
      points.resize(k);
      return points;
    }
};
