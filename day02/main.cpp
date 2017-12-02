#include <algorithm>
#include <iostream>
#include <vector>
#include <string>
#include <sstream>
#include <numeric>

int main(void)
{
	std::vector<int> csum_numbers;
	std::vector<int> csum_numbers2;

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

		int len = numbers.size();
		for(int i=0; i<len-1; i++) {
			for (int j=i+1; j<len; j++) {
				int a = std::max(numbers[i], numbers[j]);
				int b = std::min(numbers[i], numbers[j]);
				if (a%b == 0) {
					csum_numbers2.push_back(a/b);
				}
			}
		}
	}

	int csum = std::accumulate(csum_numbers.begin(), csum_numbers.end(), 0);
	int csum2 = std::accumulate(csum_numbers2.begin(), csum_numbers2.end(), 0);

	std::cout << csum << std::endl;
	std::cout << csum2 << std::endl;

    return 0;
}
