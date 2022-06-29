// link to the problem: https://leetcode.com/problems/queue-reconstruction-by-height/
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

// This is good enough, but I still feel like there is a monotonic queue solution in here somewhere
// Memory Usage: 11.5 MB, less than 99.33% of C++ online submissions for Queue Reconstruction by Height.
// Runtime: 125 ms, faster than 70.21% of C++ online submissions for Queue Reconstruction by Height.

class Solution {
 public:
  std::vector<std::vector<int>> reconstructQueue(std::vector<std::vector<int>>& people) {
    const auto comparePeople = [](const auto& first, const auto& second) {
      return first[1] == second[1] ? first[0] > second[0] : first[1] < second[1];
    };
    std::sort(people.begin(), people.end(), comparePeople);

    // print people
    // for (const auto& person : people) {
    //   std::cout << person[0] << " " << person[1] << "\n";
    // }

    std::vector<std::vector<int>> queue;
    auto it = people.begin();
    while (it != people.end() && (*it)[1] == 0) queue.insert(queue.begin(), *(std::move(it++)));

    // print queue
    // std::cout << "Queue after inserting 0:\n";
    // for (const auto& person : queue) {
    //   std::cout << person[0] << " " << person[1] << "\n";
    // }

    while (it != people.end()) {
      const int& peopleTaller = (*it)[1];
      const int& height = (*it)[0];
      auto queueIterator = queue.begin();
      int tallerPeopleCounter = 0;
      while (queueIterator != queue.end() && tallerPeopleCounter < peopleTaller) {
        if ((*queueIterator)[0] >= height) ++tallerPeopleCounter;
        ++queueIterator;
      }
      queue.insert(queueIterator, std::move(*it));
      ++it;
    }

    return queue;
  }
};

int main(int argc, char** argv) {
  Solution s;
  // std::vector<std::vector<int>> people = {{7, 0}, {4, 4}, {7, 1}, {5, 0}, {6, 1}, {5, 2}};
  // std::vector<std::vector<int>> people = {{6, 0}, {5, 0}, {4, 0}, {3, 2}, {2, 2}, {1, 4}};
  std::vector<std::vector<int>> people = {{1, 0}};
  const auto res = s.reconstructQueue(people);
  // print res
  std::cout << "Result:\n";
  for (const auto& person : res) {
    std::cout << person[0] << " " << person[1] << "\n";
  }
}
