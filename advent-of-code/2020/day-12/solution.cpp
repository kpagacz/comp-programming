#include <ranges>

int main() {
  for (const auto& i : std::views::iota(0, 2)) {
  }
  return 0;
}
