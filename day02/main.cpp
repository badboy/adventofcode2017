#include <algorithm>
#include <iostream>
#include <vector>
#include <string>
#include <sstream>
#include <numeric>

int main(void)
{
	std::vector<int> csum_numbers;
	for (std::string line; std::getline(std::cin, line);) {
		std::stringstream inp(line);
		std::vector<int> numbers;

		for (std::string number; std::getline(inp, number, ' ');) {
			int n = std::stoi(number);
			numbers.push_back(n);
		}

		int min = *std::min_element(numbers.begin(), numbers.end());
		int max = *std::max_element(numbers.begin(), numbers.end());

		csum_numbers.push_back(max-min);
	}

	int csum = std::accumulate(csum_numbers.begin(), csum_numbers.end(), 0);
	std::cout << csum << std::endl;

    return 0;
}
