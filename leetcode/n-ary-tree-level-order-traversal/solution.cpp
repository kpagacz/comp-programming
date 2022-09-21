// link to the problem: https://leetcode.com/problems/n-ary-tree-level-order-traversal/
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

// Runtime: 65 ms, faster than 5.44% of C++ online submissions for N-ary Tree Level Order Traversal.
// Memory Usage: 11.9 MB, less than 35.74% of C++ online submissions for N-ary Tree Level Order Traversal.

// Definition for a Node.
class Node {
 public:
  int val;
  std::vector<Node*> children;

  Node() {}

  Node(int _val) { val = _val; }

  Node(int _val, std::vector<Node*> _children) {
    val = _val;
    children = _children;
  }
};

class Solution {
 public:
  std::vector<std::vector<int>> levelOrder(Node* root) {
    std::vector<std::vector<int>> answer;
    std::queue<Node*> queue;
    if (root) queue.push(root);
    while (!queue.empty()) {
      int size = queue.size();
      std::vector<int> levelNodes;
      while (size--) {
        auto front = queue.front();
        queue.pop();
        levelNodes.push_back(front->val);
        for (const auto& child : front->children)
          if (child) queue.push(child);
      }
      answer.push_back(levelNodes);
    }
    return answer;
  }
};

int main(int argc, char** argv) {}
