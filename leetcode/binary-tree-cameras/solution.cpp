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
  int minCameraCover(TreeNode *root) {
    // print(root, 0);
    bottom_up_traversal(root, 0, nullptr);
    if (root == nullptr) return 0;
    if (is_covered.count(root) == 0) cameras++;
    return cameras;
  }

 private:
  int cameras;
  std::unordered_map<TreeNode *, bool> is_covered;
  void bottom_up_traversal(TreeNode *node, int depth, TreeNode *parent) {
    if (node == nullptr) return;
    ++depth;
    bottom_up_traversal(node->left, depth, node);
    bottom_up_traversal(node->right, depth, node);
    // std::cout << "Node: " << node << " At depth: " << depth - 1 << '\n';

    if ((node->left != nullptr && is_covered.count(node->left) == 0) ||
        (node->right != nullptr) && is_covered.count(node->right) == 0) {
      ++cameras;
      is_covered.insert({node, true});
      if (node->left != nullptr) is_covered.insert({node->left, true});
      if (node->right != nullptr) is_covered.insert({node->right, true});
      if (parent != nullptr) is_covered.insert({parent, true});
    }
    // std::cout << "Camera status of this node: " << std::boolalpha << (has_camera.count(node) == 1) << '\n';
  }

  void print(TreeNode *root, int depth) {
    if (root == nullptr) return;
    print(root->left, depth + 1);
    for (auto i{0}; i < depth; i++) std::cout << "  ";
    std::cout << root << '\n';
    print(root->right, depth + 1);
  }
};
