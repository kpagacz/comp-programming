// link to the problem: https://leetcode.com/problems/average-of-levels-in-binary-tree/
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

// Runtime: 15 ms, faster than 92.19% of C++ online submissions for Average of Levels in Binary Tree.
// Memory Usage: 22.4 MB, less than 72.83% of C++ online submissions for Average of Levels in Binary Tree.

// Definition for a binary tree node.
struct TreeNode {
  int val;
  TreeNode *left;
  TreeNode *right;
  TreeNode() : val(0), left(nullptr), right(nullptr) {}
  TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
  TreeNode(int x, TreeNode *left, TreeNode *right) : val(x), left(left), right(right) {}
};

class Solution {
 public:
  std::vector<double> averageOfLevels(TreeNode *root) {
    std::queue<TreeNode *> levelNodes;
    std::vector<double> answer;
    levelNodes.push(root);

    while (!levelNodes.empty()) {
      int queueSize = levelNodes.size();
      int64_t sum = 0;
      for (int i = 0; i < queueSize; ++i) {
        sum += levelNodes.front()->val;
        for (const auto &child : {levelNodes.front()->left, levelNodes.front()->right})
          if (child) levelNodes.push(child);
        levelNodes.pop();
      }
      answer.push_back(1.0 * sum / queueSize);
    }
    return answer;
  }
};

int main(int argc, char **argv) {}
