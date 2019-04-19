#include "grammar_cpp_gen.h"


grammar_cpp_gen::grammar_cpp_gen()
{

}


grammar_cpp_gen::~grammar_cpp_gen()
{

}

void grammar_cpp_gen::gen() {

	split_string("Hello World Good Day");

	/// Generate the LL(1) Parse Table
	std::string rule;

	std::vector<std::string> non_terminals;
	std::map<std::string, std::vector<std::string>> rules;
	std::map<std::string, std::vector<std::pair<std::string, int>>> first;
	std::map<std::string, std::vector<std::string>> follow;

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

	std::cout << "Number of Non-Terminals: " << non_terminals.size() << std::endl;

	/// Generate FIRST()

	std::cout << "Starting FIRST()" << std::endl;

	std::map<std::string, std::vector<std::string>> rules_copy = rules;
	std::map<std::string, std::vector<std::string>>::iterator it = rules_copy.begin();
	while (it != rules_copy.end()) {
		std::vector<std::string> rhs = it->second;
		std::cout << "\nProcessing LHS: " << it->first << std::endl;
		while (!rhs.empty()) {
			std::string first_production = rhs.back().substr(0, rhs.back().find(" "));
			rhs.pop_back();
			if (first_production.substr(0, 2) == "TS") {
				std::cout << "first(" << it->first << ") = " << first_production << std::endl;
				first[it->first].push_back({ first_production, std::distance(non_terminals.begin(), std::find(non_terminals.begin(), non_terminals.end(), first_production)) });
			}
			else {
				for (auto additions : rules[first_production]) {
					rhs.push_back(additions);
				}
			}
		}
		it++;
	}

	/// Generate FOLLOW()
	std::cout << std::endl << "Starting FOLLOW()" << std::endl;

	bool has_changed = true;

	while (has_changed) {
		has_changed = false;
		for (auto non_terminal : non_terminals) {
			std::cout << non_terminal << std::endl;
			for (auto rule : rules) {
				for (auto procedure : rule.second) {
					std::vector<std::string> tokens = split_string(procedure);
					std::vector<std::string>::iterator t_it = tokens.begin();

					while (t_it != tokens.end()) {
						// If the current string is the non-terminal
						if (*t_it == non_terminal) {
							std::cout << "\t" << non_terminal << " in: " << rule.first << " -> " << procedure << std::endl;
							// If the next string is the end, 
							if ((t_it + 1) == tokens.end()) {
								std::cout << "\t\t - Changed : No more symbols, FOLLOW() Caller" << std::endl;
								for (auto f : follow[rule.first]) {
									if (std::find(follow[non_terminal].begin(), follow[non_terminal].end(), f) == follow[non_terminal].end()) {
										follow[non_terminal].push_back(f);
										std::cout << "\t\t" << "Added " << f << " to " << non_terminal << std::endl;
										has_changed = true;
									}
								}
							}
							else if ((*(t_it + 1)).substr(0, 2) == "NT") {
								std::cout << "\t\t - Changed : NTS Detected, Getting FIRST()" << std::endl;
								for (auto f : first[(*(t_it + 1))]) {
									if (std::find(follow[non_terminal].begin(), follow[non_terminal].end(), f.first) == follow[non_terminal].end()) {
										follow[non_terminal].push_back(f.first);
										std::cout << "\t\t" << "Added " << f.first << " to " << non_terminal << std::endl;
										has_changed = true;
									}
								}
							}
							else if ((*(t_it + 1)).substr(0, 2) == "TS" && std::find(follow[non_terminal].begin(), follow[non_terminal].end(), *(t_it + 1)) == follow[non_terminal].end()) {
								std::cout << "\t\t - Changed : Terminal Symbol Detected" << std::endl;
								follow[non_terminal].push_back((*(t_it + 1)));
								std::cout << "\t\t" << "Added " << (*(t_it + 1)) << " to " << non_terminal << std::endl;	
								has_changed = true;
							}
						}
						t_it++;
					}
				}
			}
		}
	}

	for (auto pair : follow) {
		for (auto s : pair.second) {
			std::cout << "follow(" << pair.first << ") = " << s << std::endl;
		}
	}
}

std::vector<std::string> grammar_cpp_gen::split_string(std::string str) {
	std::vector<std::string> out;

	while (str.find(" ") != -1) {
		out.push_back(str.substr(0, str.find(" ")));
		str = str.substr(str.find(" ") + 1);
	}
	out.push_back(str);
	return out;

}
