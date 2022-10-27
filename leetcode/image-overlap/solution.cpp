// link to the problem: https://leetcode.com/problems/image-overlap/
#include <algorithm>
#include <array>
#include <cassert>
#include <iostream>
#include <iterator>
#include <numeric>
#include <queue>
#include <set>
#include <sstream>
#include <stack>
#include <stdexcept>
#include <string>
#include <unordered_map>
#include <unordered_set>
#include <vector>

// Runtime: 2356 ms, faster than 5.56% of C++ online submissions for Image Overlap.
// Memory Usage: 394.5 MB, less than 7.64% of C++ online submissions for Image Overlap.

template <typename T>
void printMatrix(const std::vector<std::vector<T>>& matrix) {
  for (const auto& row : matrix) {
    std::copy(row.begin(), row.end(), std::ostream_iterator<T>(std::cout, " "));
    std::cout << '\n';
  }
  std::cout << '\n';
};

class Solution {
 public:
  int largestOverlap(std::vector<std::vector<int>>& img1, std::vector<std::vector<int>>& img2) {
    int maxOverlap = 0;
    for (int i = -1 * (int)img1.size() + 1; i < (int)img1.size(); i++)
      for (int j = -1 * (int)img1.size() + 1; j < (int)img1.size(); j++) {
        auto shifted = shiftHorizontal(img1, i);
        shifted = shiftVertical(shifted, j);
        maxOverlap = std::max(maxOverlap, overlap(shifted, img2));
      }
    return maxOverlap;
  }

  int overlap(const std::vector<std::vector<int>>& img1, std::vector<std::vector<int>>& img2) {
    int overlap = 0;
    for (int i = 0; i < img1.size(); i++)
      for (int j = 0; j < img1.size(); j++)
        if (img1[i][j] == 1 && img2[i][j] == 1) overlap++;
    return overlap;
  }

  std::vector<std::vector<int>> shiftHorizontal(const std::vector<std::vector<int>>& img, int shift) {
    auto shifted(img);
    for (int i = 0; i < img.size(); i++)
      for (int j = 0; j < img.size(); j++)
        if (j - shift >= 0 && j - shift < img.size())
          shifted[i][j] = img[i][j - shift];
        else
          shifted[i][j] = 0;

    return shifted;
  }

  std::vector<std::vector<int>> shiftVertical(const std::vector<std::vector<int>>& img, int shift) {
    auto shifted(img);
    for (int i = 0; i < img.size(); ++i)
      for (int j = 0; j < img.size(); ++j)
        if (i - shift >= 0 && i - shift < img.size())
          shifted[i][j] = img[i - shift][j];
        else
          shifted[i][j] = 0;
    return shifted;
  }
};

int main(int argc, char** argv) {}
