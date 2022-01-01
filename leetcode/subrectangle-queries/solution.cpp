// link to the problem: https://leetcode.com/problems/subrectangle-queries/
#include <algorithm>
#include <array>
#include <iostream>
#include <iterator>
#include <numeric>
#include <queue>
#include <sstream>
#include <string>
#include <vector>

class SubrectangleQueries {
 public:
  std::vector<std::vector<int>> rect;
  SubrectangleQueries(std::vector<std::vector<int>>& rectangle) : rect(rectangle) {}

  void updateSubrectangle(int row1, int col1, int row2, int col2, int newValue) {
    for(int i{row1}; i <= row2; i++) {
      for(int j {col1}; j <= col2; j++) {
        rect[i][j] = newValue;
      }
    }
  }

  int getValue(int row, int col) {
    return rect[row][col];
  }
};

int main(int argc, char** argv) {}
