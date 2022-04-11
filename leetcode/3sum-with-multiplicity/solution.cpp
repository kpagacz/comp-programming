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

constexpr int MOD = 1000000000 + 7;

class Solution {
 public:
  int threeSumMulti(std::vector<int>& arr, int target) {
    std::sort(arr.begin(), arr.end());
    int answer = 0;

    for (auto i{0}; i < arr.size() - 2; i++) {
      int twoTarget = target - arr[i];
      if (twoTarget >= 0) {
        answer += twoSumMulti(arr, twoTarget, i + 1);
        answer %= MOD;
      }
    }
    return answer;
  }

  int* counts = new int[101];

  int twoSumMulti(const std::vector<int>& arr, const int& target, const int& begin) {
    int answer = 0;
    std::fill(counts, counts + 101, 0);
    if (begin >= arr.size()) return 0;
    for (auto i{begin}; i < arr.size(); i++) counts[arr[i]]++;

    int left = 0, right = 100;
    while (left < right) {
      auto sum = left + right;
      if (sum < target) {
        left++;
      } else if (sum > target) {
        right--;
      } else {
        answer += counts[left] * counts[right];
        answer %= MOD;
        left++;
      }
    }
    if (left == right && 2 * left == target) {
      answer += counts[left] * (counts[left] - 1) / 2;
      answer %= MOD;
    }

    return answer;
  }
};

int main(int argc, char** argv) {
  std::vector<int> arr{1,1,2, 2};
  Solution s;
  std::cout << "Hello\n";
  // std::cout << s.twoSumMulti(arr, 4, 0);
  std::cout << s.threeSumMulti(arr, 5);
}
