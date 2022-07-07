// link to the problem: https://leetcode.com/problems/interleaving-string/
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
  struct TupleHash {
    auto operator()(const std::tuple<std::string_view, std::string_view, std::string_view>& tuple) const {
      std::size_t hash = std::hash<std::string_view>()(std::get<0>(tuple));
      hash = hash << 4;
      hash ^= std::hash<std::string_view>()(std::get<1>(tuple));
      hash = hash << 4;
      hash ^= std::hash<std::string_view>()(std::get<2>(tuple));
      return hash;
    };
  };

 public:
  using Cache = std::unordered_map<std::tuple<std::string_view, std::string_view, std::string_view>, bool, TupleHash>;
  bool isInterleave(std::string s1, std::string s2, std::string s3) {
    Cache cache;
    return isInterleaveRecursive(s1, s2, s3, cache);
  }

  // The solution without the cache gets TLE.
  // The recursive solution with the cache is:
  // Runtime: 26 ms, faster than 33.67% of C++ online submissions for Interleaving String.
  // Memory Usage: 15.1 MB, less than 34.72% of C++ online submissions for Interleaving String.
  bool isInterleaveRecursive(const std::string_view& s1, const std::string_view& s2, const std::string_view& s3,
                             Cache& cache) {
    if (s1.empty() && s2.empty() && s3.empty()) return true;

    auto tuple = std::make_tuple(s1, s2, s3);
    if (cache.find(tuple) != cache.end()) return cache[tuple];

    auto s1ContinuationPossible = false;
    if (!s1.empty() && !s3.empty())
      s1ContinuationPossible =
          s1.front() == s3.front() ? isInterleaveRecursive(s1.substr(1), s2, s3.substr(1), cache) : false;
    auto s2ContinuationPossible = false;
    if (!s2.empty() && !s3.empty())
      s2ContinuationPossible =
          s2.front() == s3.front() ? isInterleaveRecursive(s1, s2.substr(1), s3.substr(1), cache) : false;

    auto res = s1ContinuationPossible || s2ContinuationPossible;
    cache[tuple] = res;
    return res;
  }
};

int main(int argc, char** argv) {}
