// link to the problem https://leetcode.com/problems/assign-cookies/
#include<iostream>
#include<vector>
#include<algorithm>

class Solution {
public:
	int findContentChildren(std::vector<int>& g, std::vector<int>& s) {
		std::sort(g.begin(), g.end());
		std::sort(s.begin(), s.end());
		int i = 0, j = 0;
		int answer = 0;
		while (i < g.size() && j < s.size()) {
			if (s[j] >= g[i]) {
				answer++;
				i++;
				j++;
			}
			else {
				j++;
			}
		}
		return answer;
	}
};

int main() {
	Solution sol = Solution();
	std::vector<int> test_g = { 1,2,3 };
	std::vector<int> test_s = { 1,1 };
	std::cout << sol.findContentChildren(test_g, test_s);
}