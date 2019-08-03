import copy

NONTERMINALS = []
TERMINALS = []
GRAMMAR = {}
INDEXED_GRAMMAR = {}
FIRST_SET = {}
FOLLOW_SET = {}
GRAMMAR_SYMBOLS = []
C = {}


# TODO - Nodes have extra parameters that can be automated
def main():
    global TERMINALS, NONTERMINALS, GRAMMAR, GRAMMAR_SYMBOLS, INDEXED_GRAMMAR
    TERMINALS += ['TS_END_OF_FILE']

    index = 0
    production_set = [x.strip() for x in open(r'../grammar').readlines()]
    for production in production_set:
        if len(production) > 2 and production[0] != "#":
            lhs = production[:production.find('->') - 1]
            rhs = production[production.find('->') + 3:]

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

    for key, item in INDEXED_GRAMMAR.items():
        print(key)
    tGrammar = copy.deepcopy(GRAMMAR)
    FIRST_SET = FIRST(tGrammar)

    tGrammar = copy.deepcopy(GRAMMAR)
    tFirst = copy.deepcopy(FIRST_SET)
    FOLLOW_SET = FOLLOW(tGrammar, tFirst)

    # RULES = TRANSITIONS(INDEXED_GRAMMAR)
    ITEMS(GRAMMAR, FIRST_SET, FOLLOW_SET, RULES=None)
    return


def FIRST(G):
    print('Calculating FIRST_SET(G)')
    first_productions = {}

    for key, item in G.items():
        rhs = item
        while len(rhs) > 0:
            back = rhs.pop()
            first = back[:back.find(' ')] if back.find(' ') != -1 else back

            if first[0:2] == 'TS':
                if key in first_productions.keys():
                    first_productions[key] += [first]
                else:
                    first_productions[key] = [first]
            elif first != '':
                for production in G[first]:
                    rhs.append(production)
    print('first_productions:', first_productions, end='\n\n')
    return first_productions


def FIRST_S(rhs, G):
    first_productions = []
    while len(rhs) > 0:
        back = rhs.pop()
        first = back[:back.find(' ')] if back.find(' ') != -1 else back

        if first[0:2] == 'TS':
            first_productions += [first]
        elif first != '':
            for production in G[first]:
                rhs.append(production)
    return first_productions


def FOLLOW(G, FIRST_SET):
    global NONTERMINALS

    print('Calculating FOLLOW(G)')
    follow_productions = {}

    follow_productions[NONTERMINALS[0]] = ['TS_END_OF_FILE']
    changed = True

    while changed:
        changed = False
        for non_terminal in NONTERMINALS:
            for key, production in G.items():
                for prod in production:
                    tokens = prod.split(' ')
                    index = 0
                    for token in tokens:
                        if token == non_terminal:
                            if index == len(tokens) - 1:
                                for fprod in (follow_productions[key] if key in follow_productions.keys() else []):
                                    if fprod not in (follow_productions[
                                        non_terminal] if non_terminal in follow_productions.keys() else []):
                                        if non_terminal in follow_productions.keys():
                                            follow_productions[non_terminal] += [fprod]
                                        else:
                                            follow_productions[non_terminal] = [fprod]
                                        changed = True
                            elif token[0:2] == "NT":
                                for fprod in (FIRST_SET[token] if token in FIRST_SET.keys() else []):
                                    if fprod not in (follow_productions[
                                        non_terminal] if non_terminal in follow_productions.keys() else []):
                                        if non_terminal in follow_productions.keys():
                                            follow_productions[non_terminal] += [fprod]
                                        else:
                                            follow_productions[non_terminal] = [fprod]
                                        changed = True
                            elif token[0:2] == "TS" and token not in (
                                    follow_productions[
                                        non_terminal] if non_terminal in follow_productions.keys() else []):
                                if non_terminal in follow_productions.keys():
                                    follow_productions[non_terminal] += [token]
                                else:
                                    follow_productions[non_terminal] = [token]
                                changed = True
                    index += 1
    print('follow_productions', follow_productions, end='\n\n')
    return follow_productions


def ITEMS(GRAMMAR, FIRST_SET, FOLLOW_SET, RULES):
    print('Calculating ITEMS(G, F, FL)')

    def __createitemset__(A, a, B, b, t):
        return {
            'A': A.strip(),
            'a': a.strip(),
            'B': B.strip(),
            'b': b.strip(),
            't': t.strip(),
        }

    def combine_item(I):
        return I['A'], I['a'].strip() + I['B'].strip() + I['b'].strip()

    def print_itemset(C):
        iter = 0
        for set in C:
            print('I{0}'.format(iter))
            for i in set:
                print('\t[{0} -> {1} * {2} {3}, {4}]'.format(i['A'], i['a'], i['B'], i['b'], i['t']))
            iter += 1

    def CLOSURE(I):
        itemAdded = True
        while itemAdded:
            itemAdded = False
            for item in I:
                if item['B'] in NONTERMINALS:
                    for production in GRAMMAR[item['B']]:
                        for b in FIRST_S([item['b'] + ' ' + item['t'] if item['b'] != '' else item['t']], GRAMMAR):
                            if b in TERMINALS:
                                titem = __createitemset__(item['B'],
                                                          '',
                                                          production[0:production.find(' ')] if production.find(
                                                              ' ') != -1 else production,
                                                          production[production.find(' ') + 1:] if production.find(
                                                              ' ') != -1 else '',
                                                          b)
                                if titem not in I:
                                    I.append(titem)
                                    itemAdded = True
        return I

    def GOTO(I, X):
        J = []
        for item in I:
            if item['B'] == X:
                J.append(__createitemset__(item['A'],
                                           item['a'] + ' ' + X,
                                           item['b'][:item['b'].find(' ')] if item['b'].find(' ') != -1 else item['b'],
                                           item['b'][item['b'].find(' ') + 1:] if item['b'].find(' ') != -1 else '',
                                           item['t']))
        cJ = CLOSURE(J)
        # print_itemset(cJ)
        return cJ

    def create_CLR_parsing_table(C):
        print('\nCreating CLR Parsing Table')
        # [A -> a*Bb, t]
        IDENTITY = {
            'A': 'NTS_MANGO',
            'a': 'NTS_STATEMENT_SUITE',
            'B': '',
            'b': '',
            't': 'TS_END_OF_FILE'
        }
        # Creating the table matrix
        ACTION = {}
        print('len(C):', len(C))
        for state in range(0, len(C)):
            for item in C[state]:
                GTC = GOTO(C[state], item['B'])
                if GTC in C and item['B'] != '':
                    j = C.index(GTC)
                    if item['B'][0:2] == 'TS':
                        ACTION[state, item['B']] = 'S{0}'.format(j)
                    else:
                        ACTION[state, item['B']] = '{0}'.format(j)
                if item['B'] == '' and item['b'] == '' and item['a'] != '':
                    print('Searching for rule', combine_item(item), 'R', INDEXED_GRAMMAR[combine_item(item)][0])
                    ACTION[state, item['t']] = 'R{0}'.format(INDEXED_GRAMMAR[combine_item(item)][0])
                if IDENTITY in C[state]:
                    ACTION[state, 'TS_END_OF_FILE'] = 'ACCEPT'
        out = ""

        # Gather Parse Table
        PARSER_INIT = ""
        NODE_INIT = ""
        for n in NONTERMINALS:
            ttype = 'Node' + ''.join(
                [str(x).replace("NTS", "").replace("TS", "").lower().capitalize() for x in
                 n.split("_")])
            NODE_INIT += "pub struct {}".format(ttype) + "{\n}" + "\n\nimpl node for {}".format(ttype) + "{\n" \
                                                                                                         "\tfn eval(&self) -> String { return \"\".to_string(); }" \
                                                                                                         "\n}\n\n"
        NODE_SELECTION_INIT = ""
        for key, item in INDEXED_GRAMMAR.items():
            # print("key-item", key, item)
            ttype = ''.join(
                [str(x).replace("NTS", "").replace("TS", "").lower().capitalize() for x in key[0].split("_")])
            if ttype != "Mango":
                if item[0] != '':
                    NODE_SELECTION_INIT += "\t\t\t\t\t\t\t{}".format(
                        item[0]) + " => {\n\t\t\t\t\t\t\t\t//" + "{}".format(
                        key[0] + " -> " + key[1]) + "\n\t\t\t\t\t\t\t}\n"
                    PARSER_INIT += "\t\tself.goto.insert({}, GotoNode".format(
                        item[0]) + "{" + "token_type: TokenType::{}, value: {}".format(ttype, str(
                        int(int(item[1]) / 2))) + "});\n"
                else:
                    NODE_SELECTION_INIT += "\t\t\t\t\t\t\t{}".format(1) + " => {}\n"
                    PARSER_INIT += "\t\tself.goto.insert({}, GotoNode".format(
                        "1") + "{" + "token_type: TokenType::{}, value: {}".format(ttype,
                                                                                   str(int(int(item[1]) / 2))) + "});\n"
        NODE_SELECTION_INIT += "\t\t\t\t\t\t\t_" + " => {\n\t\t\t\t\t\t\t\t//" + "exhaustive" + "\n\t\t\t\t\t\t\t}\n"
        for keys, item in ACTION.items():
            ttype = ''.join(
                [str(x).replace("NTS", "").replace("TS", "").lower().capitalize() for x in keys[1].split("_")])
            if item[0] == "S":
                PARSER_INIT += "\t\tself.action.insert(({}, TokenType::{}), ActionNode".format(keys[0],
                                                                                               ttype) + "{" + "action: ParserAction::{}, value: {}".format(
                    "Shift", item[1:]) + "});\n"
            elif item[0] == "R":
                PARSER_INIT += "\t\tself.action.insert(({}, TokenType::{}), ActionNode".format(keys[0],
                                                                                               ttype) + "{" + "action: ParserAction::{}, value: {}".format(
                    "Reduce", item[1:]) + "});\n"
            elif item == "ACCEPT":
                PARSER_INIT += "\t\tself.action.insert(({}, TokenType::{}), ActionNode".format(keys[0],
                                                                                               ttype) + "{" + "action: ParserAction::{}, value: {}".format(
                    "Accept", -1) + "});\n"
            else:
                PARSER_INIT += "\t\tself.action.insert(({}, TokenType::{}), ActionNode".format(keys[0],
                                                                                               ttype) + "{" + "action: ParserAction::{}, value: {}".format(
                    "Goto", item) + "});\n"

        PARSER_INIT_FILE = open('../target/parser_init.txt', 'w')
        PARSER_INIT_FILE.write(PARSER_INIT)
        PARSER_INIT_FILE.close()

        # PARSER_FILE = open('PARSER_TEMPLATE').read()
        # PARSER_FILE = PARSER_FILE.replace("{PARSER_INIT}", PARSER_INIT)
        # PARSER_FILE = PARSER_FILE.replace("{NODE_SELECTION_INIT}", NODE_SELECTION_INIT)
        #
        # NODE_FILE = open('NODE_TEMPLATE').read()
        # NODE_FILE = NODE_FILE.replace("{NODE_INIT}", NODE_INIT)
        #
        # myfile = open('../../src/parser.rs', 'w')
        # myfile.write(PARSER_FILE)
        # myfile.close()
        #
        # myfile2 = open('../../src/parse_tree.rs', 'w')
        # myfile2.write(NODE_FILE)
        # myfile2.close()

    C = [CLOSURE([{
        'A': 'NTS_MANGO',
        'a': '',
        'B': 'NTS_STATEMENT_SUITE',
        'b': '',
        't': 'TS_END_OF_FILE'
    }])]
    # Note that this should change when the Mango language is implemented

    added = True
    while added:
        added = False
        for I in C:
            for X in GRAMMAR_SYMBOLS:
                GTX = GOTO(I, X)
                if len(GTX) > 0 and GTX not in C:
                    C.append(GTX)
                    added = True

    print_itemset(C)
    create_CLR_parsing_table(C)


if __name__ == '__main__':
    main()
