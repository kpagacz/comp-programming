// link to the problem: https://leetcode.com/problems/binary-search-tree-iterator/
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
  TreeNode *left;
  TreeNode *right;
  TreeNode() : val(0), left(nullptr), right(nullptr) {}
  TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
  TreeNode(int x, TreeNode *left, TreeNode *right) : val(x), left(left), right(right) {}
};

class BSTIterator {
 public:
  BSTIterator(TreeNode *root) {
    TreeNode *it = root;
    while (it != nullptr) {
      nodes.push(it);
      it = it->left;
    }
  }

  int next() {
    auto next = nodes.top()->val;
    auto successor = nodes.top()->right;
    nodes.pop();
    while (successor != nullptr) {
      nodes.push(successor);
      successor = successor->left;
    }

    return next;
  }

  bool hasNext() {
    return !nodes.empty();
  }

 private:
  std::stack<TreeNode *> nodes;
};

int main(int argc, char **argv) {}
