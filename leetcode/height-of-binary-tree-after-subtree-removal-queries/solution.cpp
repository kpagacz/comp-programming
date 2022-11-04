// link to the problem: https://leetcode.com/problems/height-of-binary-tree-after-subtree-removal-queries/
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

// Take-out:
// * I learned how to answer queries about tree and subtree heights. Turns out, you only need to know heights
// of all subtrees to compute the heights of a tree after any removal.
// * In this case, the problem asked to remove a subtree, but we could think of removing subtrees via a different
// predictor than just remove a subtree.
// * As long as it is contained to a single tree level, I can manage this one easily. A much more complicated problem
// would be to remove a subtree and something like its nephew at the same time - a removal spanning multiple levels.
// I could not solve such a problem using my approach of doing nodes by level.

// Runtime: 1216 ms, faster than 46.57% of C++ online submissions for Height of Binary Tree After Subtree Removal
// Queries. Memory Usage: 220.5 MB, less than 64.70% of C++ online submissions for Height of Binary Tree After Subtree
// Removal Queries.

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
  std::vector<int> treeQueries(TreeNode *root, std::vector<int> &queries) {
    std::unordered_map<TreeNode *, int> heights;
    computeHeights(root, heights);
    auto precomputedAnswers = computeAnswers(root, heights);
    std::vector<int> answers;
    answers.reserve(queries.size());
    for (const auto &query : queries) answers.push_back(precomputedAnswers[query]);
    return answers;
  }

  void computeHeights(TreeNode *node, std::unordered_map<TreeNode *, int> &heights) {
    if (node == nullptr) return;
    for (const auto &child : {node->left, node->right}) computeHeights(child, heights);
    heights[node] = std::max(heights[node->left], heights[node->right]) + 1;
  }

  std::vector<int> computeAnswers(TreeNode *root, const std::unordered_map<TreeNode *, int> &heights) {
    std::vector<int> answers(heights.size(), 0);
    std::vector<TreeNode *> levelNodes;
    for (const auto &child : {root->left, root->right})
      if (child) levelNodes.push_back(child);

    int depth = 0;
    while (levelNodes.empty() == false) {
      std::vector<TreeNode *> nextLevelNodes;
      TreeNode *best = nullptr, *secondBest = nullptr;
      for (const auto &node : levelNodes) {
        for (const auto &child : {node->left, node->right})
          if (child) nextLevelNodes.push_back(child);
        if (best == nullptr || heights.at(node) > heights.at(best))
          secondBest = best, best = node;
        else if (secondBest == nullptr || heights.at(node) > heights.at(secondBest))
          secondBest = node;
      }

      for (const auto &node : levelNodes) {
        if (node == best)
          answers[node->val] = depth + (secondBest ? heights.at(secondBest) : 0);
        else
          answers[node->val] = depth + heights.at(best);
      }
      levelNodes = nextLevelNodes;
      depth++;
    }

    return answers;
  }
};

int main(int argc, char **argv) {
  Solution s;
  TreeNode *root = new TreeNode(1, nullptr, nullptr);
  root->left = new TreeNode(3);
  root->left->left = new TreeNode(2);
  root->right = new TreeNode(4);
  root->right->left = new TreeNode(6);
  root->right->right = new TreeNode(5);
  root->right->right->right = new TreeNode(7);
  root->right->right->left = new TreeNode(8);
  std::vector<int> queries{3, 2, 4, 8};
  auto answer = s.treeQueries(root, queries);
  std::copy(answer.begin(), answer.end(), std::ostream_iterator<int>(std::cout, " "));
  std::cout << '\n';
}
