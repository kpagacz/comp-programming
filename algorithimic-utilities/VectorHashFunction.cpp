#include <vector>

struct VectorHashFunction {
  template <typename T>
  std::size_t operator()(const std::vector<T>& v) const {
    std::hash<T> hashFunction;
    std::size_t answer = 0;
    for (const auto& element : v) answer ^= hashFunction(element) + 0x9e3779b9 + (answer << 6) + (answer >> 2);
    return answer;
  }
};
