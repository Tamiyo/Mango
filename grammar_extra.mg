
NTS_STMT -> NTS_CTRL


%{ 
# Ignore This
NTS_CTRL -> NTS_CTRL_IF NTS_CTRL_ELSEIF NTS_CTRL_ELSE

NTS_CTRL_IF -> TS_IF TS_LPAREN NTS_COND TS_RPAREN TS_LCB NTS_STMTS TS_RCB
NTS_CTRL_ELSEIF -> TS_ELSEIF TS_SPACE NTS_COND TS_SPACE TS_LCB NTS_STMTS TS_RCB
NTS_CTRL_ELSEIF -> ''

NTS_CTRL_ELSE -> TS_ELSE TS_SPACE TS_LCB NTS_STMTS TS_RCB
NTS_CTRL_ELSE -> ''

NTS_COND -> TS_VARIABLE NTS_COND_OPERATOR TS_VARIABLE
NTS_COND -> NTS_COND_S_OPERATOR TS_VARIABLE
%}






	std::cout << "\nProcessing LHS: " << rule.first << std::endl;
		std::string rhs = rule.second.at(0);
		for (auto production : rule.second) {
			int i = 0;
			std::string first_production = production.substr(0, production.find(" "));
			if (first_production.substr(0, 2) == "TS") {
				std::cout << "first(" << rule.first << ") = " << first_production << std::endl;
				first[rule.first].push_back({ first_production, std::distance(non_terminals.begin(), std::find(non_terminals.begin(), non_terminals.end(), first_production))});
			}
			else {
				while (true) {
					std::string prev_production = first_production;
					first_production = rules[prev_production].at(0).substr(0, rules[prev_production].at(0).find(" "));
					std::cout << "\t\t" << first_production << std::endl;
					if (first_production.substr(0, 2) == "TS") {
						std::cout << "first(" << rule.first << ") = " << first_production << std::endl;
						first[rule.first].push_back({ first_production, std::distance(non_terminals.begin(), std::find(non_terminals.begin(), non_terminals.end(), first_production)) });
						break;
					}
				}
			}
		}






		rules_copy = rules;
	for (auto nt : non_terminals) {
		for (auto rule : rules_copy) {
			for (auto prod : rule.second) {
				std::vector<std::string> children = split_string(prod);
				std::vector<std::string>::iterator sp_it = children.begin();
				while (sp_it != children.end()) {
					std::string token = (*sp_it);
					if (token == nt) {
						if ((*(sp_it + 1)).substr(0, 2) == "TS") {
							follow[nt].push_back({ *(sp_it + 1), std::distance(non_terminals.begin(), std::find(non_terminals.begin(), non_terminals.end(), nt)) });
						} else if ((sp_it + 1) == children.end()) {

						}
					}
					else {
						sp_it++;
					}
				}
			}
		}
	}