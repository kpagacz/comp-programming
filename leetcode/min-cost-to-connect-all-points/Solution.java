// What did I learn:
// So turns out that Integer(8) != Integer(8)
// Which made the comparison between the results of two finds always false...
// Yeah, a good IDE would tell me to use .equals but I wrote this in VSCode which is not that smart
// to point out the obvious problem (I spent an hour debugging this).
//
// For the future: always compare **objects** using .equals!

// Link to the problem: https://leetcode.com/problems/min-cost-to-connect-all-points/
import java.lang.Comparable;
import java.util.PriorityQueue;
import java.util.ArrayList;

class UnionFind {
  private final ArrayList<Node> nodes = new ArrayList<>();

  public UnionFind() {
  }

  public void makeSet(int element) {
    if (element >= nodes.size())
      nodes.add(new Node(element));
  }

  public int find(int element) {
    Node nodeElement = nodes.get(element);
    if (nodeElement.parent != nodeElement.value)
      nodeElement.parent = find(nodeElement.parent);
    return nodeElement.parent;
  }

  public void union(int first, int second) {
    if (find(first) != find(second)) {
      Node firstRoot = nodes.get(find(first));
      Node secondRoot = nodes.get(find(second));

      if (firstRoot.rank > secondRoot.rank) {
        secondRoot.parent = firstRoot.value;
      } else if (firstRoot.rank < secondRoot.rank) {
        firstRoot.parent = secondRoot.value;
      } else {
        firstRoot.parent = secondRoot.value;
        secondRoot.rank++;
      }
    }
  }

  @Override
  public String toString() {
    StringBuilder out = new StringBuilder();
    nodes.forEach(out::append);
    return out.toString();
  }

  private class Node {
    int parent;
    int rank;
    private final int value;

    public Node(int value) {
      this.value = value;
      parent = value;
    }

    @Override
    public String toString() {
      return "Value: " + value + " Parent: " + parent;
    }
  }

  // For testing
  // public static void main(String[] args) {
  // UnionFind uf = new UnionFind();
  // for (int i = 0; i < 3; i++) {
  // uf.makeSet(i);
  // }

  // uf.nodes.forEach(System.out::println);

  // System.out.println("After Union of 0 and 1:");
  // uf.union(0, 1);
  // uf.nodes.forEach(System.out::println);

  // System.out.println("Root of 0: " + uf.find(0) + " Root of 1: " + uf.find(1));

  // System.out.println("After Union of 0 and 2:");
  // uf.union(2, 0);
  // uf.nodes.forEach(System.out::println);
  // System.out.println("Find 1:" + uf.find(1) + " find 2: " +
  // uf.find(2) + " boolean: " + (uf.find(1) == uf.find(2)));
  // }
}

class Edge implements Comparable<Edge> {
  public final int vertexA, vertexB, weight;

  public Edge(int vertexA, int vertexB, int weight) {
    this.vertexA = vertexA;
    this.vertexB = vertexB;
    this.weight = weight;
  }

  @Override
  public int compareTo(Edge other) {
    return weight - other.weight;
  }
}

class Solution {
  private final PriorityQueue<Edge> edgeMinHeap = new PriorityQueue<>();
  private final UnionFind vertices = new UnionFind();

  public int minCostConnectPoints(int[][] points) {
    for (int i = 0; i < points.length; i++)
      vertices.makeSet(i);

    for (int i = 0; i < points.length; i++)
      for (int j = i + 1; j < points.length; j++)
        edgeMinHeap.add(new Edge(i, j, distance(points[i], points[j])));

    int minimumSpan = 0;
    int vertexCount = 0;
    while (vertexCount < points.length - 1) {
      Edge edge = edgeMinHeap.remove();
      if (vertices.find(edge.vertexA) != vertices.find(edge.vertexB)) {
        minimumSpan += edge.weight;
        vertices.union(edge.vertexA, edge.vertexB);
        vertexCount++;
      }
    }

    return minimumSpan;
  }

  private int distance(int[] firstPoint, int[] secondPoint) {
    return Math.abs(secondPoint[0] - firstPoint[0]) + Math.abs(secondPoint[1] - firstPoint[1]);
  }
}
