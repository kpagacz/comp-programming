// link to the problem: https://leetcode.com/problems/construct-binary-tree-from-preorder-and-inorder-traversal/
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

// Runtime: 20 ms, faster than 85.25% of C++ online submissions for Construct Binary Tree from Preorder and Inorder
// Traversal. Memory Usage: 25.9 MB, less than 79.05% of C++ online submissions for Construct Binary Tree from Preorder
// and Inorder Traversal.

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
  TreeNode* buildTree(std::vector<int>& preorder, std::vector<int>& inorder) {
    if (preorder.empty() || inorder.empty()) return nullptr;
    auto it = preorder.begin();
    return buildTreePreOrder(preorder, it, inorder.begin(), inorder.end());
  }

 private:
  using it = std::vector<int>::iterator;
  TreeNode* buildTreePreOrder(const std::vector<int>& preOrder, it& preOrderIterator, const it& inOrderBegin,
                              const it& inOrderEnd) {
    if (preOrderIterator == preOrder.end()) return nullptr;
    if (inOrderBegin == inOrderEnd) return nullptr;
    const auto leftSubtreeEnd = std::find(inOrderBegin, inOrderEnd, *preOrderIterator);
    if (leftSubtreeEnd == inOrderEnd) return nullptr;
    const auto newNode = new TreeNode(*preOrderIterator);
    ++preOrderIterator;
    newNode->left = buildTreePreOrder(preOrder, preOrderIterator, inOrderBegin, leftSubtreeEnd);
    newNode->right = buildTreePreOrder(preOrder, preOrderIterator, leftSubtreeEnd + 1, inOrderEnd);

    return newNode;
  }
};

int main(int argc, char** argv) {}
