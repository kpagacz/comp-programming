// link to the problem: https://leetcode.com/problems/single-number/
// What did I learn?
// * another repeat of the XOR properties
// * I need to drill in to myself to always check if the problem can be solved with a XOR
// * especially when it's "missing element" and "pairs"
// * XOR is commutative and a ^ a ^ b = b because a ^ a ^ b = 0 ^ b = b
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
#include<functional>

class Solution {
public:
 int singleNumber(std::vector<int>& nums) { return std::accumulate(nums.begin(), nums.end(), 0, std::bit_xor<int>()); }
};

int main(int argc, char** argv) {}
