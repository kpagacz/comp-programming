// link to the problem: https://leetcode.com/problems/richest-customer-wealth/
#include<iostream>
#include<vector>
#include<array>
#include<string>
#include<algorithm>
#include<numeric>
#include<sstream>
#include<iterator>
#include<queue>

class Solution {
public:
    int maximumWealth(std::vector<std::vector<int>>& accounts) {
      int max{0};
      std::for_each(accounts.begin(), accounts.end(), [&](const auto& v) {
        auto sum = std::accumulate(v.begin(), v.end(), 0);
        if (sum > max) max = sum;
      });
      return max;
    }
};

int main(int argc, char** argv) {

}
