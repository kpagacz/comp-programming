import java.util.ArrayList;

class UnionFind {
  ArrayList<Node> nodes = new ArrayList<>();

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

  public static void main(String[] args) {
    UnionFind uf = new UnionFind();
    for (int i = 0; i < 3; i++) {
      uf.makeSet(i);
    }

    uf.nodes.forEach(System.out::println);

    System.out.println("After Union of 0 and 1:");
    uf.union(0, 1);
    uf.nodes.forEach(System.out::println);

    System.out.println("Root of 0: " + uf.find(0) + " Root of 1: " + uf.find(1));

    System.out.println("After Union of 0 and 2:");
    uf.union(2, 0);
    uf.nodes.forEach(System.out::println);
    System.out.println("Find 1:" + uf.find(1) + " find 2: " +
     uf.find(2) + " boolean: " + (uf.find(1) == uf.find(2)));
  }
}
