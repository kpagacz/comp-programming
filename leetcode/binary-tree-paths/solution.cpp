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

// Runtime: 4 ms, faster than 79.66% of C++ online submissions for Binary Tree Paths.
// Memory Usage: 12.1 MB, less than 97.94% of C++ online submissions for Binary Tree Paths.

//  Definition for a binary tree node.
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
  std::vector<std::string> binaryTreePaths(TreeNode *root) {
    std::vector<std::string> answer;

    std::string currentPath = "";
    binaryTreePaths(root, answer, currentPath);
    return answer;
  }

  void binaryTreePaths(const TreeNode *root, std::vector<std::string> &answer, std::string &currentPath) {
    if (root == nullptr) return;
    if (root->left == nullptr && root->right == nullptr) {
      answer.push_back(currentPath + std::to_string(root->val));
      return;
    }

    const std::string &addition = std::to_string(root->val) + "->";
    currentPath += addition;
    binaryTreePaths(root->left, answer, currentPath);
    binaryTreePaths(root->right, answer, currentPath);
    currentPath = currentPath.erase(currentPath.size() - addition.size(), addition.size());
  }
};

int main(int argc, char **argv) {}
