# Global Variables to be used throughout the generator along with init setup

import os
import re

NONTERMINALS = []
TERMINALS = []
GRAMMAR = {}
INDEXED_GRAMMAR = {}
MATCH_INDEX_GRAMMAR = ""
FIRST_SET = {}
FOLLOW_SET = {}
GRAMMAR_SYMBOLS = []
PATH = ""


def init():
    global NONTERMINALS, TERMINALS, GRAMMAR, INDEXED_GRAMMAR, MATCH_INDEX_GRAMMAR, FIRST_SET, FOLLOW_SET, GRAMMAR_SYMBOLS, C, PATH
    PATH = os.path.dirname(os.path.abspath(__file__))

    TERMINALS += ['TS_END_OF_FILE']

    index = 0
    test_str = open(PATH + r'\..\grammar').read()
    pattern = '(\w+ -> [\w ]+)|((?<=\{\n)(.|\n)*?(?=\n\}))'
    matches = re.findall(pattern, test_str)

    name = ''
    for match in matches:
        print(match)

        # If the current pattern is of match subset '0', then we know that
        # this is a grammar rule, and so we parse the rule to fill out the required
        # variables for future use. This needs to be changes as the regex is no loner being used.
        if match[0] != '':
            production = match[0]
            lhs = production[:production.find('->') - 1]
            rhs = production[production.find('->') + 3:]
            name = production

            if index == 0:
                INDEXED_GRAMMAR[lhs, rhs] = ('', 0)
                index += 1
            else:
                INDEXED_GRAMMAR[lhs, rhs] = (index, len(rhs.split(' ') * 2))
                index += 1

            # Add to Grammar
            if lhs in GRAMMAR.keys():
                GRAMMAR[lhs] += [rhs]
            else:
                GRAMMAR[lhs] = [rhs]

            # Add to Nonterminals
            if lhs not in NONTERMINALS:
                NONTERMINALS += [lhs]
                if lhs not in GRAMMAR_SYMBOLS:
                    GRAMMAR_SYMBOLS += [lhs]

            # Add to terminals
            rhs_tokens = rhs.split(' ')
            for token in rhs_tokens:
                if token not in TERMINALS:
                    TERMINALS += [token]
                    if token not in GRAMMAR_SYMBOLS:
                        GRAMMAR_SYMBOLS += [token]
        # Deprecated
        elif match[1] != '':
            pass
