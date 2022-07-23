#include <vector>

template <typename T>
class BinaryIndexedTree {
 public:
  BinaryIndexedTree(std::size_t size) { bitArray.resize(size + 1, 0); }
  void add(std::size_t index, T value) {
    ++index;
    while (index < bitArray.size()) {
      bitArray[index] += value;
      index += (index & -index);
    }
  }
  T sum(std::size_t index) {
    T sum = 0;
    while (index > 0) {
      sum += bitArray[index];
      index -= (index & -index);
    }
  }

  T sum(std::size_t left, std::size_t right) { return sum(right) - sum(left); }

 private:
  std::vector<T> bitArray;
};
