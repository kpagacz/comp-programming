// link to the problem: https://leetcode.com/problems/recover-binary-search-tree/
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

// Definition for a binary tree node.
struct TreeNode {
  int val;
  TreeNode* left;
  TreeNode* right;
  TreeNode() : val(0), left(nullptr), right(nullptr) {}
  TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
  TreeNode(int x, TreeNode* left, TreeNode* right) : val(x), left(left), right(right) {}
};

class Solution {
 public:
  void recoverTree(TreeNode* root) {
    std::vector<TreeNode*> nodes;
    inOrderTraversal(root, nodes);
    auto nodesComparator = [](const auto& first, const auto& second) { return first->val < second->val; };
    auto outOfOrder = std::is_sorted_until(nodes.begin(), nodes.end(), nodesComparator);
    if (outOfOrder == nodes.end()) return;
    outOfOrder = std::next(outOfOrder, -1);
    auto nextOutOfOrder =
        std::is_sorted_until(nodes.rbegin(), nodes.rend(), std::not_fn(nodesComparator));
    nextOutOfOrder = std::next(nextOutOfOrder, -1);
    std::swap((*outOfOrder)->val, (*nextOutOfOrder)->val);
  }

  void inOrderTraversal(TreeNode* it, std::vector<TreeNode*>& nodes) {
    if (it != nullptr) {
      inOrderTraversal(it->left, nodes);
      nodes.push_back(it);
      inOrderTraversal(it->right, nodes);
    }
  }
};

int main(int argc, char** argv) {}
