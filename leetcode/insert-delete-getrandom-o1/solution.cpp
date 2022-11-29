// link to the problem: https://leetcode.com/problems/insert-delete-getrandom-o1/
#include <algorithm>
#include <array>
#include <cassert>
#include <iostream>
#include <iterator>
#include <numeric>
#include <queue>
#include <random>
#include <set>
#include <sstream>
#include <stack>
#include <stdexcept>
#include <string>
#include <unordered_map>
#include <unordered_set>
#include <vector>

// Runtime: 254 ms, faster than 84.88% of C++ online submissions for Insert Delete GetRandom O(1).
// Memory Usage: 97.2 MB, less than 9.91% of C++ online submissions for Insert Delete GetRandom O(1).

class RandomizedSet {
 public:
  RandomizedSet() {}

  bool insert(int val) {
    if (map.count(val)) return false;
    arr.push_back(val);
    map[val] = arr.size() - 1;
    return true;
  }

  bool remove(int val) {
    if (map.count(val) == 0) return false;
    auto vec_index = map[val];
    auto last_element = arr.back();
    map[last_element] = vec_index;
    std::swap(arr[vec_index], arr.back());
    arr.pop_back();
    map.erase(val);
    return true;
  }

  int getRandom() {
    std::uniform_int_distribution dist(0, (int)arr.size() - 1);
    return arr[dist(gen)];
  }

 private:
  std::unordered_map<int, int> map;  // key: int, value: index of value in vector
  std::vector<int> arr;              // arr[i] = int
  std::mt19937 gen;
};

/**
 * Your RandomizedSet object will be instantiated and called as such:
 * RandomizedSet* obj = new RandomizedSet();
 * bool param_1 = obj->insert(val);
 * bool param_2 = obj->remove(val);
 * int param_3 = obj->getRandom();
 */

int main(int argc, char** argv) {}
