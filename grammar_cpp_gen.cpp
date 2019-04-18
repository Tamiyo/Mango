#include "grammar_cpp_gen.h"


grammar_cpp_gen::grammar_cpp_gen()
{

}


grammar_cpp_gen::~grammar_cpp_gen()
{

}

void grammar_cpp_gen::gen() {


	/// Generate the LL(1) Parse Table
	std::string rule;

	std::vector<std::string> non_terminals;
	std::map<std::string, std::vector<std::string>> rules;
	std::map<std::string, std::vector<std::pair<std::string, int>>> first;

	std::ifstream grammarFile("grammar.mg");
	if (grammarFile.is_open()) {
		while (getline(grammarFile, rule)) {
			if (rule.length() > 2) {
				std::string lhs = rule.substr(0, rule.find("->") - 1);
				std::string rhs = rule.substr(rule.find("->") + 3);
				rules[lhs].push_back(rhs);

				if (std::find(non_terminals.begin(), non_terminals.end(), lhs) == non_terminals.end()) {
					non_terminals.push_back(lhs);
				}
			}
		}
		grammarFile.close();
	}
	else {
		std::cout << "Unable to open file" << std::endl;
	}

	/// Generate FIRST()
	for (auto rule : rules) {
		std::string rhs = rule.second.at(0);
		for (auto production : rule.second) {
			std::string first_production = production.substr(0, production.find(" "));
			printf("rhs: %s\nprod: %s\nfirst_prod: %s\n\n",rhs, production, first_production);
			if (first_production.substr(0, 2) == "TS") {
				first[rule.first].push_back({ first_production , std::distance(non_terminals.begin(), std::find(non_terminals.begin(), non_terminals.end(), rule.first)) + 1 });
			}
			else {
				while (true) {
					std::string prev_production = first_production;
					first_production = rules[prev_production].at(0);
					if (first_production.substr(0, 2) == "TS") {
						first[rule.first].push_back({ first_production , std::distance(non_terminals.begin(), std::find(non_terminals.begin(), non_terminals.end(), prev_production)) + 1 });
						break;
					}
				}
			}
			printf("first(%s) = %s\n", rule.first, first[rule.first]);
		}
		}
}
