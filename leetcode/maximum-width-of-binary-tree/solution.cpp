// link to the problem: https://leetcode.com/problems/maximum-width-of-binary-tree/
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

class Solution {
 public:
  int widthOfBinaryTree(TreeNode *root) {
    uint64_t max_width = 1;
    std::queue<std::pair<TreeNode *, uint64_t>> queue;
    queue.push({root, 0});
    while (!queue.empty()) {
      uint64_t first = 0, last = 0;
      std::queue<std::pair<TreeNode *, uint64_t>> new_queue;
      first = queue.front().second;
      while(!queue.empty()) {
        auto front = queue.front();
        queue.pop();
        last = front.second;
        if (front.first->left) new_queue.push({front.first->left, 2 * front.second});
        if (front.first->right) new_queue.push({front.first->right, 2 * front.second + 1});
      }
      max_width = std::max(max_width, last - first + 1);
      queue = new_queue;
    }
    return max_width;
  }
};

int main(int argc, char **argv) {}
