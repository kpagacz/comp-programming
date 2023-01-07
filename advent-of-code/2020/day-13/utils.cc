#include <functional>
#include <string>
#include <type_traits>

// Requires c++20

namespace utils {
template <typename T>
struct remove_const_and_reference;
template <typename T>
struct remove_const_and_reference<const T&> {
  typedef std::remove_cv_t<std::remove_reference_t<T>> type;
};
template <typename T>
using remove_const_and_reference_t = typename remove_const_and_reference<T>::type;

template <typename... Ts>
struct TupleHash {
  std::size_t operator()(const std::tuple<Ts...>& tuple) const {
    return std::apply(Combine(), tuple);
  }

 private:
  struct Combine {
    std::size_t operator()(const Ts&... args) const {
      const int multiplier = 0b1110110;
      std::size_t hash = 0x123456;
      (
          [&]<typename T>(T&& arg) {
            hash = multiplier * (hash ^ std::hash<remove_const_and_reference_t<decltype(args)>>()(args));
          }(args),
          ...);
      return hash;
    }  // (h(2)^((h(1) ^ init) * m)) * m
  };
};

std::string replaceAll(const std::string_view& text, const std::string_view& pattern,
                       const std::string_view& replacement) {
  std::string out{text};
  auto found = 0;
  while ((found = out.find(pattern, found)) != out.npos) {
    out.replace(found, pattern.size(), replacement);
    found++;
  }
  return out;
}

std::vector<std::string> split(const std::string_view& text, const std::string_view& separator) {
  std::string baseText{text};
  std::vector<std::string> components;
  std::size_t it{0}, found;
  while ((found = text.find(separator, it)) != text.npos) {
    components.push_back(baseText.substr(it, found - it));
    it = found + separator.size();
  }
  components.push_back(baseText.substr(it));
  return components;
}
}  // namespace utils

// int main() {
// TupleHash<int, int> pairHash;
// std::cout << pairHash(std::pair<int, int>{0, 1e10}) << '\n';
// std::cout << pairHash(std::pair<int, int>{1, 0}) << '\n';
// std::cout << pairHash(std::pair<int, int>{0, 0}) << '\n';

// TupleHash<double, int, int64_t> tupleHash;
// std::cout << tupleHash(std::make_tuple(1.1, 0, -7)) << '\n';
//   return 0;
// }
