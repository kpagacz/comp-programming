// link to the problem: https://leetcode.com/problems/robot-bounded-in-circle/
#include <algorithm>
#include <array>
#include <iostream>
#include <iterator>
#include <numeric>
#include <queue>
#include <sstream>
#include <string>
#include <vector>

class Solution {
 public:
  bool isRobotBounded(std::string instructions) {
    int x{0}, y{0}, d{0};

    for (const auto& c : instructions) {
      switch (c) {
        case 'L':
          d = (d + 4 - 1) % 4;
          break;
        case 'R':
          d = (d + 4 + 1) % 4;
          break;
        case 'G':
          switch (d) {
            case 0:
              x++;
              break;
            case 1:
              y++;
              break;
            case 2:
              x--;
              break;
            case 3:
              y--;
              break;
          }
          break;
      }
    }

    return x == 0 && y == 0 || d > 0;
  }
};

int main(int argc, char** argv) {

}
