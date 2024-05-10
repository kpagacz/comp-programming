"""
# Definition for a Node.
class Node(object):
    def __init__(self, val=None, children=None):
        self.val = val
        self.children = children
"""
class Solution(object):
    def preorder(self, root):
        """
        :type root: Node
        :rtype: List[int]
        """
        output = []
        rec(root, output)
        return output

    def rec(self, root, output):
        if not root:
            return
        output.append(root.val)
        for child in root.children:
            self.rec(child, output)
