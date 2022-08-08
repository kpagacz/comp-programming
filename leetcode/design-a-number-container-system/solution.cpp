// link to the problem: https://leetcode.com/problems/design-a-number-container-system/
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

// Runtime: 887 ms, faster than 55.58% of C++ online submissions for Design a Number Container System.
// Memory Usage: 177.4 MB, less than 41.93% of C++ online submissions for Design a Number Container System.

class NumberContainers {
 public:
  NumberContainers() {}

  void change(int index, int number) {
    if (numbers.count(index) != 0) {
      const auto& oldNumber = numbers[index];
      auto& indices = orderedIndices[oldNumber];
      indices.erase(index);
    }
    numbers[index] = number;
    orderedIndices[number].insert(index);
  }

  int find(int number) {
    const auto& indices = orderedIndices[number];
    if (indices.empty()) return -1;
    return *(indices.begin());
  }
  std::unordered_map<int, int> numbers;
  std::unordered_map<int, std::set<int>> orderedIndices;
};

/**
 * Your NumberContainers object will be instantiated and called as such:
 * NumberContainers* obj = new NumberContainers();
 * obj->change(index,number);
 * int param_2 = obj->find(number);
 */

int main(int argc, char** argv) {
  NumberContainers* container = new NumberContainers();
  std::cout << container->find(10) << '\n';
  container->change(2, 10);
  container->change(3, 10);
  std::cout << container->find(10) << '\n';
  container->change(1, 20);
  std::cout << container->find(10) << '\n';
  std::cout << container->find(20) << '\n';
  container->change(1, 30);
  container->change(5, 40);
  std::cout << container->find(20) << '\n';
  std::cout << container->find(40) << '\n';
}
