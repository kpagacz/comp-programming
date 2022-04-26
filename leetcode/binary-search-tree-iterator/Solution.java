import java.util.Stack;

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

class BSTIterator {
  private Stack<TreeNode> nodes;

    public BSTIterator(TreeNode root) {
      nodes = new Stack<>();
      while (root != null) {
        nodes.push(root);
        root = root.left;
      }
    }

    public int next() {
      int next = nodes.peek().val;
      TreeNode it = nodes.pop().right;
      while (it != null) {
        nodes.push(it);
        it = it.left;
      }
    }

    public boolean hasNext() {

    }
}
