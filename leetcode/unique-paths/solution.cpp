// link to the problem: https://leetcode.com/problems/unique-paths/submissions/
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
    int nChoosek( long long n, long long k ) {
        if (k > n) return 0;
        if (k * 2 > n) k = n-k;
        if (k == 0) return 1;
        if (k == n) return 1;

        long long result = n;
        for( int i = 2; i <= k; ++i ) {
            result *= (n-i+1);
            result /= i;
        }
        return result;
    }
    int uniquePaths(int m, int n) {
        return nChoosek(m + n - 2, m - 1);
    }
};

int main(int argc, char** argv) {}
