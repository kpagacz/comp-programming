// link to the problem:
// https://leetcode.com/problems/populating-next-right-pointers-in-each-node/
#include <algorithm>
#include <array>
#include <iostream>
#include <iterator>
#include <numeric>
#include <sstream>
#include <string>
#include <unordered_map>
#include <vector>

// Definition for a Node.
class Node {
 public:
  int val;
  Node* left;
  Node* right;
  Node* next;

  Node() : val(0), left(NULL), right(NULL), next(NULL) {}

  Node(int _val) : val(_val), left(NULL), right(NULL), next(NULL) {}

  Node(int _val, Node* _left, Node* _right, Node* _next)
      : val(_val), left(_left), right(_right), next(_next) {}

  void print() {
    Node* it = this;
    while(it != nullptr) {
      std::cout << it->val << " ";
      it = it->next;
    }
    std::cout << "#";
    if (left != nullptr) left->print();
  }
};

class Solution {
 public:
  Node* connect(Node* root) {
    if (root == nullptr) return root;
    if (root->left != nullptr) {
      connect(root->left);
      connect(root->right);
      Node *left_side = root->left, *right_side = root->right;
      while(left_side != nullptr) {
        left_side->next = right_side;
        left_side = left_side->right;
        right_side = right_side->left;
      }
    }
    return (root);
  }
};

int main() {
  Node* root = new Node(1);
  root->left = new Node(2);
  root->right = new Node(3);
  root->left->left = new Node(4);
  root->left->right = new Node(5);
  root->right->left = new Node(6);
  root->right->right = new Node(7);
  Solution s;
  root = s.connect(root);
  root->print();
}
