// link to the problem: https://leetcode.com/problems/vertical-order-traversal-of-a-binary-tree/
#include <algorithm>
#include <array>
#include <cassert>
#include <iostream>
#include <iterator>
#include <map>
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

// Runtime: 7 ms, faster than 71.86% of C++ online submissions for Vertical Order Traversal of a Binary Tree.
// Memory Usage: 12 MB, less than 93.05% of C++ online submissions for Vertical Order Traversal of a Binary Tree.

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
  std::vector<std::vector<int>> verticalTraversal(TreeNode *root) {
    std::vector<std::vector<int>> answer;
    std::queue<std::pair<int, TreeNode *>> queue;
    std::map<int, std::vector<int>> nodesSortedByColumns;

    queue.push({0, root});

    while (!queue.empty()) {
      int levelSize = queue.size();
      std::map<int, std::vector<int>> currentLevelNodesSortedByColumns;
      while (levelSize--) {
        auto [column, currentNode] = queue.front();
        queue.pop();
        currentLevelNodesSortedByColumns[column].push_back(currentNode->val);
        if (currentNode->left) queue.push({column - 1, currentNode->left});
        if (currentNode->right) queue.push({column + 1, currentNode->right});
      }

      for (auto &[column, values] : currentLevelNodesSortedByColumns) {
        std::sort(values.begin(), values.end());
        std::copy(values.begin(), values.end(), std::back_inserter(nodesSortedByColumns[column]));
      }
    }

    for (const auto &[_, values] : nodesSortedByColumns) answer.push_back(values);
    return answer;
  }
};

int main(int argc, char **argv) {}
