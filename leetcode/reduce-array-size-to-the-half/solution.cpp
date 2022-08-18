// link to the problem: https://leetcode.com/problems/reduce-array-size-to-the-half/
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
  int minSetSize(std::vector<int>& arr) {
    std::unordered_map<int, int> frequencies;
    for (const auto& num : arr) frequencies[num]++;
    std::priority_queue<std::pair<int, int>> freqHeap;
    for (const auto& [num, counts] : frequencies) freqHeap.push({counts, num});

    int size = arr.size();
    int answer = 0;
    while (size > arr.size() / 2) {
      size -= freqHeap.top().first;
      freqHeap.pop();
      answer++;
    }

    return answer;
  }
};

int main(int argc, char** argv) {}
