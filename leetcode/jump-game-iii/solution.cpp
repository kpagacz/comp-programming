// link to the problem: https://leetcode.com/problems/jump-game-iii/
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
  bool canReach(std::vector<int>& arr, int start) {
    std::vector<bool> visited(arr.size(), false);
    std::queue<int> indices;
    indices.push(start);

    while (!indices.empty()) {
      auto front = indices.front();
      indices.pop();
      if (front < 0 || front >= arr.size() || visited[front]) continue;
      if (arr[front] == 0) return true;
      visited[front] = true;
      indices.push(front - arr[front]);
      indices.push(front + arr[front]);
    }

    return false;
  }
};

int main(int argc, char** argv) {}
