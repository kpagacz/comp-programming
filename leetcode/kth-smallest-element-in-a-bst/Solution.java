// link to the problem: https://leetcode.com/problems/kth-smallest-element-in-a-bst/
// Definition for a binary tree node.
public class TreeNode {
    int val;
    TreeNode left;
    TreeNode right;
    TreeNode() {}
    TreeNode(int val) { this.val = val; }
    TreeNode(int val, TreeNode left, TreeNode right) {
        this.val = val;
        this.left = left;
        this.right = right;
    }
}

class Solution {
    public int kthSmallest(TreeNode root, int k) {
      Stack<TreeNode> nodes = new Stack<>();

      while(!nodes.empty() || root != null) {
        if (root == null) {
          root = nodes.pop();
          if (--k == 0) return root.val;
          root = root.right;
        } else {
          nodes.push(root);
          root = root.left;
        }
      }
      return -1;
    }
}
