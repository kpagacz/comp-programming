#include<iostream>
#include<iomanip>

int main() {
    int tests;

    std::cin >> tests;

    double r1, r2;

    for(int i = 0; i < tests; i++) {
        std::cin >> r1 >> r2;

        std::cout << std::fixed << std::setprecision(2) << 2 * r1 * r2 << std::endl;
    }

}