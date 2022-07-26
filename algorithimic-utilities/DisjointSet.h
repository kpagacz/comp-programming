#include <numeric>
#include <vector>

class DisjointSet {
 public:
  DisjointSet(std::size_t size) {
    parent.resize(size);
    std::iota(parent.begin(), parent.end(), 0);
    rank.resize(size, 0);
  }

  DisjointSet() : DisjointSet(0) {}

  void makeSet() {
    parent.push_back(parent.size());
    rank.push_back(0);
  }

  int findSet(int element) {
    if (element >= parent.size()) return -1;
    if (parent[element] == element) return element;
    return parent[element] = findSet(parent[element]);
  }

  void unionSet(int element1, int element2) {
    if (element1 >= parent.size() || element2 >= parent.size()) return;
    const auto& root1 = findSet(element1);
    const auto& root2 = findSet(element2);

    if (root1 == root2) return;

    if (rank[root1] > rank[root2]) {
      parent[root2] = root1;
    } else {
      parent[root1] = root2;
      if (rank[root1] == rank[root2]) ++rank[root2];
    }
  }

 private:
  std::vector<int> parent, rank;
};
