#include "grammar_cpp_gen.h"

#define stringify( name ) # name

grammar_cpp_gen::grammar_cpp_gen()
{
	keys = new keywords();
}


grammar_cpp_gen::~grammar_cpp_gen()
{

}

void grammar_cpp_gen::gen_string_to_enum_array() {
	std::string symbol;
	std::ifstream enumFile("enums.txt");

	std::string enums = "#include<map>\n#include<string<>";
	std::string out = "std::map<std::string, Symbols> str_to_sym = {\n";

	if (enumFile.is_open()) {
		while (getline(enumFile, symbol)) {
			if (symbol.length() > 2 && symbol.at(0) != '/') {
				std::string sym = symbol.substr(0, symbol.find(','));
				out.append("\t{");
				out.append("\"" + sym + "\"");
				out.append(", ");
				out.append(sym);
				out.append("},\n");
			}
		}
		out.append("};");
		std::cout << out << std::endl;
		enumFile.close();
	}
}

void grammar_cpp_gen::gen_parse_table() {

	/// Setup Prerequsisite Variables
	std::string rule;

	std::map<keywords::Symbols, std::map<keywords::Symbols, int>> parse_table;

	std::vector<std::string> non_terminals;
	std::vector<std::string> terminals;
	terminals.push_back("$");

	std::map<std::string, std::vector<std::string>> rules;
	std::map<std::pair<std::string, int>, std::vector<std::string>> rules_numbered;

	std::map<std::string, std::vector<std::pair<std::string, int>>> first;
	std::map<std::string, std::vector<std::string>> follow;

	std::ifstream grammarFile("grammar.mg");

	/// Generate Prereq Items from File
	int rule_no = 1;
	if (grammarFile.is_open()) {
		while (getline(grammarFile, rule)) {
			if (rule.length() > 2) {
				std::string lhs = rule.substr(0, rule.find("->") - 1);
				std::string rhs = rule.substr(rule.find("->") + 3);
				rules[lhs].push_back(rhs);
				rules_numbered[{lhs, rule_no}].push_back(rhs);

				std::cout << lhs << " -> " << rhs << " = " << rule_no << std::endl;

				if (std::find(non_terminals.begin(), non_terminals.end(), lhs) == non_terminals.end()) {
					non_terminals.push_back(lhs);
					//std::cout << "NTS: " << lhs << std::endl;;
				}

				std::vector<std::string> spl_rhs = split_string(rhs);
				for (int i = 0; i < spl_rhs.size(); i++) {
					if (std::find(terminals.begin(), terminals.end(), spl_rhs.at(i)) == terminals.end() && spl_rhs.at(i).substr(0, 2) == "TS" && spl_rhs.at(i) != "TS_EMPTY") {
						terminals.push_back(spl_rhs.at(i));
						//std::cout << "TS: " << spl_rhs.at(i) << std::endl;
					}
				}

				rule_no++;
			}
		}
		grammarFile.close();
	}
	else {
		std::cout << "Unable to open file" << std::endl;
	}

	/// Generate FIRST() transitions
	std::cout << std::endl << "Starting FIRST()" << std::endl;

	std::map<std::pair<std::string, int>, std::vector<std::string>> rules_copy = rules_numbered;
	std::map<std::pair<std::string, int>, std::vector<std::string>>::iterator it = rules_copy.begin();
	while (it != rules_copy.end()) {
		std::vector<std::string> rhs = it->second;
		while (!rhs.empty()) {
			std::string first_production = rhs.back().substr(0, rhs.back().find(" "));

			rhs.pop_back();
			if (first_production.substr(0, 2) == "TS") {
				first[it->first.first].push_back({ first_production, it->first.second });
			}
			else {
				for (auto additions : rules[first_production]) {
					rhs.push_back(additions);
				}
			}
		}
		it++;
	}

	/// Print out FIRST for debugging
	for (auto NTS_F : first) {
		std::cout << "FIRST(" << NTS_F.first << ") = { ";
		for (auto p : NTS_F.second) {
			std::cout << p.first << ", ";
		}
		std::cout << "}" << std::endl;
	}

	/// Generate FOLLOW() transitions
	std::cout << std::endl << std::endl << "Starting FOLLOW()" << std::endl;

	bool has_changed = true;

	follow[non_terminals.at(0)].push_back("TS_EOF");

	while (has_changed) {
		has_changed = false;
		for (auto non_terminal : non_terminals) {
			for (auto rule : rules) {
				for (auto procedure : rule.second) {
					std::vector<std::string> tokens = split_string(procedure);
					std::vector<std::string>::iterator t_it = tokens.begin();

					while (t_it != tokens.end()) {
						// If the current string is the non-terminal
						if (*t_it == non_terminal) {
							// If the next string is the end, 
							if ((t_it + 1) == tokens.end()) {
								for (auto f : follow[rule.first]) {
									if (std::find(follow[non_terminal].begin(), follow[non_terminal].end(), f) == follow[non_terminal].end()) {
										follow[non_terminal].push_back(f);
										has_changed = true;
									}
								}
							}
							else if ((*(t_it + 1)).substr(0, 2) == "NT") {
								for (auto f : first[(*(t_it + 1))]) {
									if (std::find(follow[non_terminal].begin(), follow[non_terminal].end(), f.first) == follow[non_terminal].end()) {
										follow[non_terminal].push_back(f.first);
										has_changed = true;
									}
								}
							}
							else if ((*(t_it + 1)).substr(0, 2) == "TS" && std::find(follow[non_terminal].begin(), follow[non_terminal].end(), *(t_it + 1)) == follow[non_terminal].end()) {
								follow[non_terminal].push_back((*(t_it + 1)));
								has_changed = true;
							}
						}
						t_it++;
					}
				}
			}
		}
	}

	/// Print out FOLLOW()
	for (auto NTS_F : follow) {
		std::cout << "FOLLOW(" << NTS_F.first << ") = { ";
		for (auto p : NTS_F.second) {
			std::cout << p << ", ";
		}
		std::cout << "}" << std::endl;
	}

	/// Warnings for FIRST/FIRST conflicts
	std::cout << std::endl;
	for (auto rule : rules) {
		std::vector<std::string> productions = rule.second;
		if (productions.size() > 1) {
			std::vector<std::string> v;
			for (auto prod : productions) {
				v.push_back(split_string(prod).at(0));
			}
			std::vector<std::string>::iterator it = v.begin();
			while ((it + 1) != v.end()) {
				std::vector<std::pair<std::string, int>> item1 = first[*it];
				std::vector<std::pair<std::string, int>> item2 = first[*(it + 1)];

				for (auto pair1 : item1) {
					for (auto pair2 : item2) {
						if (pair1.first == pair2.first) {
							std::cout << "ERROR: " << *it << " & " << *(it + 1) << "\t\t\tFIRST/FIRST conflict" << std::endl;
						}
					}
				}
				it++;
			}

		}
	}

	std::cout << std::endl;
	for (auto rule : rules) {
		std::vector<std::string> productions = rule.second;
		if (productions.size() > 1) {
			std::vector<std::string> v;
			for (auto prod : productions) {
				v.push_back(split_string(prod).at(0));
			}
			std::vector<std::string>::iterator it = v.begin();
			while ((it + 1) != v.end()) {
				std::vector<std::string> prodvec = follow[rule.first];
				std::vector<std::pair<std::string, int>> item1 = first[*it];
				std::vector<std::pair<std::string, int>> item2 = first[*(it + 1)];

				for (auto pair1 : item1) {
					if (pair1.first == "TS_EMPTY") {
						for (auto pair3 : prodvec) {
							for (auto pair2 : item2) {
								if (pair3 == pair2.first) {
									std::cout << "ERROR: " << *it << " & " << *(it + 1) << "\t\t\tFIRST/FOLLOW conflict" << std::endl;
								}
							}
						}
					}
				}
				it++;
			}

		}
	}

	/// Warnings for FIRST/FOLLOW conflicts
	for (auto NTS : non_terminals) {
		std::vector<std::pair<std::string, int>> NTS_FI = first[NTS];
		std::vector<std::string> NTS_FO = follow[NTS];

		for (auto pair1 : NTS_FI) {
			for (auto pair2 : NTS_FO) {
				if (pair1.first == pair2) {
					std::cout << "Warning: " << NTS << "\t\t\t\tPossible FIRST/FOLLOW conflict" << std::endl;
					goto WARNING_DONE;
				}
			}
		}
	WARNING_DONE:;
	}

	/// Create the LL(1) Parse Table File
	std::cout << std::endl;

	std::string cpp = "/// This file is generated by crammar_cpp_gen.cpp based on grammar.mg specficiations\n\n#include \"mgparser.h\"\n\nmgparser::mgparser(const char* body) {\n\tlexer = new mglex(body);\n";

	for (auto pair : first) {
		for (auto col : pair.second) {
			if (col.first == "TS_EMPTY") {
				int num = 0;
				for (auto rel_pair : rules_numbered) {
					if (rel_pair.first.first == pair.first && rel_pair.second.at(0) == "TS_EMPTY") {
						num = rel_pair.first.second;
						break;
					}
				}
				for (auto SYM : follow[pair.first]) {
					cpp.append("\ttable[keywords::");
					cpp.append(pair.first);
					cpp.append("][keywords::");
					cpp.append(SYM);
					cpp.append("] = ");
					cpp.append(std::to_string(num));
					cpp.append(";\n");
				}
			}
			else {
				int num = col.second;
				cpp.append("\ttable[keywords::");
				cpp.append(pair.first);
				cpp.append("][keywords::");
				cpp.append(col.first);
				cpp.append("] = ");
				cpp.append(std::to_string(num));
				cpp.append(";\n");
			}
		}
	}

	cpp.append("\tss.push(keywords::TS_EOF);\n\tss.push(keywords::NTS_MANGO);\n}\n\n\nvoid mgparser::ppeval() {\n\std::pair<const char*, keywords::Symbols> token = lexer->lltoken();\n\twhile(ss.size() > 0) {\n\t\twhile (!token.first) { token = lexer->lltoken(); }\n\t\tcout << \"Found a: \" << token.first << endl;\n\t\tif(token.second == ss.top()) {\n\t\t\tcout << \"Matched symbols : \" << token.first << endl;\n\t\t\tss.pop();\n\t\t\ttoken = lexer->lltoken();\n\t\t}\n\t\telse {\n\t\t\tcout << \"Rule \" << table[ss.top()][token.second] << endl;\n\t\t\tswitch(table[ss.top()][token.second]) {\n");

	for (auto rule : rules_numbered) {
		for (auto production : rule.second) {
			std::vector<std::string> prod = split_string(production);
			cpp.append("\t\t\t\tcase ");
			cpp.append(std::to_string(rule.first.second));
			cpp.append(":\n");
			cpp.append("\t\t\t\t\tss.pop();\n");
			std::reverse(prod.begin(), prod.end());
			for (auto p : prod) {
				cpp.append("\t\t\t\t\tss.push(keywords::");
				cpp.append(p);
				cpp.append(");\n");
			}
			cpp.append("\t\t\t\t\tbreak;\n");
		}
	}

	cpp.append("\t\t\t\tdefault:\n\t\t\t\t\tcout << \"parsing table defaulted\" << endl;\n\t\t\t\t\treturn;\n\t\t\t}\n\t\t}\n\t}\ncout << \"finished parsing\" << endl;\nreturn;\n}");

	/// Print the parse table for debugging
	//std::cout << cpp << std::endl;

	/// Write to the parsing file
	std::ofstream myfile;
	myfile.open("parser/mgparser.cpp");
	myfile.clear();
	myfile << cpp;
	myfile.close();

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
