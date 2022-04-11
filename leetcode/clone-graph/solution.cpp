// link to the problem: https://leetcode.com/problems/clone-graph/
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

// Definition for a Node.
class Node {
 public:
  int val;
  std::vector<Node*> neighbors;
  Node() {
    val = 0;
    neighbors = std::vector<Node*>();
  }
  Node(int _val) {
    val = _val;
    neighbors = std::vector<Node*>();
  }
  Node(int _val, std::vector<Node*> _neighbors) {
    val = _val;
    neighbors = _neighbors;
  }
};

class Solution {
 public:
  Node* cloneGraph(Node* node) {
    std::vector<Node*> visited(100, nullptr);
    std::queue<Node*> to_visit;
    to_visit.push(node);
    while (!to_visit.empty()) {
      Node* current = to_visit.front();
      to_visit.pop();
      if (visited[current->val] != nullptr) continue;
      visited[current->val] = new Node(current->val, current->neighbors);
      std::for_each(current->neighbors.begin(), current->neighbors.end(), [&](const auto& el) { to_visit.push(el); });
    }

    for (const auto& n : visited) {
      if (n != nullptr) {
        for (auto i{0}; i < n->neighbors.size(); i++) n->neighbors[i] = visited[n->neighbors[i]->val];
      }
    }

    return visited[node->val];
  }
};

int main(int argc, char** argv) {}
