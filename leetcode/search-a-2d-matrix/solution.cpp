// link to the problem:
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
#include<cassert>

class Solution {
 public:
  bool searchMatrix(std::vector<std::vector<int>>& matrix, int target) {
    int left = 0, right = matrix.size() - 1;
    while(left < right) {
      int half = (right - left) / 2 + left;
      if (matrix[half][0] == target) return true;
      else {
        if (matrix[half][0] > target) {
          right = half;
        } else {
          left = half + 1;
        }
      }
    }

    // std::cout << left << " " << right << " value: " << matrix[left][0] << '\n';
    int suspected_row;
    if (matrix[left][0] > target) {
      suspected_row = left - 1;
    } else {
      suspected_row = left;
    }
    if (suspected_row == -1) return false;

    left = 0;
    right = matrix[suspected_row].size() - 1;

    while(left < right) {
      int half = (right - left) / 2 + left;
      if (matrix[suspected_row][half] == target) return true;
      else {
        if (matrix[suspected_row][half] > target) {
          right = half;
        } else {
          left = half + 1;
        }
      }
    }
    if (matrix[suspected_row][left] == target) return true;

    return false;
  }
};

int main(int argc, char** argv) {}
