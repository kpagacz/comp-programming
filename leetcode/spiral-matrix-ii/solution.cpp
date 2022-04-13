// link to the problem: https://leetcode.com/problems/spiral-matrix-ii/
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

class Solution {
 public:
  std::vector<std::vector<int>> generateMatrix(int n) {
    std::vector<std::vector<int>> matrix(n, std::vector<int>(n, 0));
    int layer = 0, it = 1;
    while (it <= n * n) drawLayer(matrix, layer++, it);
    return matrix;
  }

  void drawLayer(std::vector<std::vector<int>>& matrix, const int& layer, int& it) {
    int row = layer, col = layer;
    if (layer > matrix.size() / 2) {
      matrix[0][0] = it++;
      return;
    }
    while (col < matrix.size() - layer) matrix[row][col++] = it++;
    col = matrix.size() - layer - 1;
    row++;
    while (row < matrix.size() - layer) matrix[row++][col] = it++;
    row = matrix.size() - layer - 1;
    col--;
    while (col >= layer) matrix[row][col--] = it++;
    col = layer;
    row--;
    while (row >= layer + 1) matrix[row--][col] = it++;
  }
};

int main(int argc, char** argv) {
  // std::vector<std::vector<int>> matrix(n, std::vector<int>(n, 0));
  // int it = 0;
  // Solution s;
  // s.drawLayer(matrix, 0, it);
  Solution s;
  for (const auto& n : {1, 2, 3}) {
    auto matrix = s.generateMatrix(n);
    for (const auto& v : matrix) {
      for (const auto& el : v) std::cout << el << " ";
      std::cout << '\n';
    }
  }
}
