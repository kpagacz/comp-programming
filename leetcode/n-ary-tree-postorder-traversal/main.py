"""
# Definition for a Node.
class Node(object):
    def __init__(self, val=None, children=None):
        self.val = val
        self.children = children
"""

class Solution(object):
    def postorder(self, root):
        """
        :type root: Node
        :rtype: List[int]
        """
        output = []
        self.rec(root, output)
        return output

    def rec(self, root, output):
        if root is None:
            return
        for child in root.children:
            self.rec(child, output)
        output.append(root.val)
