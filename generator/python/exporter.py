from algorithm import ACTION
from algorithm_helpers import convert_to_ttype
from settings import INDEXED_GRAMMAR, PATH

PARSE_TABLE_INSERTION = ''
REDUCE_STATEMENTS = ''
NODE_DEFINITIONS = ''


# Creates the parser.h c++ file 'parse table' from the generated grammar
def export_parse_table():
    global PARSE_TABLE_INSERTION

    # Generate the GOTO table insertions
    for key, item in INDEXED_GRAMMAR.items():
        ttype = convert_to_ttype(key[0])
        if ttype != "Mango":
            if item[0] != '':
                PARSE_TABLE_INSERTION += "\t\tgoto_table.insert({" + \
                                         "{}, ".format(item[0]) + \
                                         "GotoNode{" + \
                                         "TokenType::{}, {}".format(ttype, str(int(int(item[1]) / 2))) + \
                                         "}});\n"
            else:
                PARSE_TABLE_INSERTION += "\t\tgoto_table.insert({" + \
                                         "{}, ".format("1") + \
                                         "GotoNode{" + \
                                         "TokenType::{}, {}".format(ttype, str(int(int(item[1]) / 2))) + \
                                         "}});"

    # Generate the ACTION table insertions
    for keys, item in ACTION.items():
        ttype = convert_to_ttype(keys[1])
        if item[0] == "S":
            PARSE_TABLE_INSERTION += "\t\taction_table.insert({{" + \
                                     "{}, TokenType::{}".format(keys[0], ttype) + \
                                     "}, ActionNode{ParserAction::" + \
                                     "{}, {}".format("Shift", item[1:]) + \
                                     "}});\n"
        elif item[0] == "R":
            PARSE_TABLE_INSERTION += "\t\taction_table.insert({{" + \
                                     "{}, TokenType::{}".format(keys[0], ttype) + \
                                     "}, ActionNode{ParserAction::" + \
                                     "{}, {}".format("Reduce", item[1:]) + \
                                     "}});\n"
        elif item == "ACCEPT":
            PARSE_TABLE_INSERTION += "\t\taction_table.insert({{" + \
                                     "{}, TokenType::{}".format(keys[0], ttype) + \
                                     "}, ActionNode{ParserAction::" + \
                                     "{}, {}".format("Accept", str(-1)) + \
                                     "}});\n"
        else:
            PARSE_TABLE_INSERTION += "\t\taction_table.insert({{" + \
                                     "{}, TokenType::{}".format(keys[0], ttype) + \
                                     "}, ActionNode{ParserAction::" + \
                                     "{}, {}".format("Goto", item) + \
                                     "}});\n"


# Creates the parser.h c++ file 'reduce statements' from the generated grammar

def export_reduce_statements():
    global REDUCE_STATEMENTS

    tokens = None
    name_prev = ''
    dup_counter = 0

    for key, item in INDEXED_GRAMMAR.items():
        index = item[0]
        rhs = key[1]

        tokens = rhs.split(' ')
        tokens.reverse()

        CASE = '\t\t\t\t\tcase {}: '.format(index if index is not None else 0) + \
               '{\n' + \
               '\t\t\t\t\t\t// {} -> {}\n'.format(key[0], key[1])
        VARIABLES = []
        iter = 1
        for token in tokens:
            if token[0:3] == "NTS" or token in ['TS_TERM', 'TS_IDENTIFIER', 'TS_TRIPLE_EQUALS', 'TS_DOUBLE_EQUALS',
                                                'TS_GREATER_THAN_EQUALS', 'TS_GREATER_THAN', 'TS_LESS_THAN_EQUALS',
                                                'TS_LESS_THAN', 'TS_NEGATION']:
                ttype = convert_to_ttype(token).lower()
                if ttype in VARIABLES:
                    ttype = ttype + str(iter)
                    iter += 1
                VARIABLES.append(ttype)
                CASE += "\t\t\t\t\t\tNode* {}".format(ttype) + \
                        " = node_stack.back(); \n" + \
                        "\t\t\t\t\t\tnode_stack.pop_back(); \n"
        VARIABLES.reverse()
        node_name = 'Node' + convert_to_ttype(key[0])

        if node_name == name_prev:
            dup_counter += 1
            name_prev = node_name
            node_name = node_name + "_Production" + str(dup_counter)
        else:
            dup_counter = 0
            name_prev = node_name

        CASE += "\t\t\t\t\t\t{0} *node = new {0}".format(node_name) + \
                "{" + \
                "{}".format(', '.join(VARIABLES)) + \
                "};\n"
        CASE += "\t\t\t\t\t\tnode_stack.push_back(dynamic_cast<Node *>(node));\n" + \
                "\t\t\t\t\t\tbreak;\n" + \
                "\t\t\t\t\t}\n"
        REDUCE_STATEMENTS += CASE


# Creates the parse_tree.h c++ file 'parse tree nodes' from the generated grammar
def export_node_definitions():
    global NODE_DEFINITIONS

    tokens = None
    name_prev = ''
    dup_counter = 0

    for key, item in INDEXED_GRAMMAR.items():
        index = item[0]
        rhs = key[1]

        tokens = rhs.split(' ')

        VARIABLES = []
        iter = 1
        for token in tokens:
            if token[0:3] == "NTS" or token in ['TS_TERM', 'TS_IDENTIFIER', 'TS_TRIPLE_EQUALS', 'TS_DOUBLE_EQUALS',
                                                'TS_GREATER_THAN_EQUALS', 'TS_GREATER_THAN', 'TS_LESS_THAN_EQUALS',
                                                'TS_LESS_THAN', 'TS_NEGATION']:
                ttype = convert_to_ttype(token).lower()
                if ttype in VARIABLES:
                    ttype = ttype + str(iter)
                    iter += 1
                VARIABLES.append(ttype)
        VARIABLES.reverse()
        node_name = 'Node' + convert_to_ttype(key[0])

        if node_name == name_prev:
            dup_counter += 1
            name_prev = node_name
            node_name = node_name + "_Production" + str(dup_counter)
        else:
            dup_counter = 0
            name_prev = node_name

        STRUCT = '// {} -> {}\n'.format(key[0], key[1])
        STRUCT += 'struct {} : virtual Node '.format(node_name) + "{\n" \
                                                                 'public:\n'
        for var in VARIABLES:
            STRUCT += "\tNode* {};\n".format(var)
        STRUCT += '\n\tNode* eval() override {\n' \
                  '\t\treturn {};\n' \
                  '\t};\n\n'
        STRUCT += '\texplicit {} ('.format(node_name)
        for var in VARIABLES:
            STRUCT += "Node* {}, ".format(var)
        STRUCT = STRUCT[:STRUCT.rfind(',')] + ") {\n"

        for var in VARIABLES:
            STRUCT += "\t\tthis->{0} = {0};\n".format(var)
        STRUCT += "\t}\n};\n\n"
        NODE_DEFINITIONS += STRUCT


# Wrapper function that calls the above methods and compiles them together.
def export_parser():
    global PARSE_TABLE_INSERTION, REDUCE_STATEMENTS, NODE_DEFINITIONS

    export_parse_table()
    export_reduce_statements()
    export_node_definitions()

    # Read the parser template file and add the new exported code to its contents
    parser_template_file = open(PATH + "../templates/PARSER_TEMPLATE")
    CONTENTS = parser_template_file.read()
    CONTENTS = CONTENTS.replace("@@PARSE_TABLE_INSERTION@@", PARSE_TABLE_INSERTION)
    CONTENTS = CONTENTS.replace("@@REDUCE_STATEMENTS@@", REDUCE_STATEMENTS)

    # Write the exported contents to the respective file
    parser_file = open(PATH + "../target/src/parser.h", 'w+')
    parser_file.write(CONTENTS)

    # Read the parse tree template file and add the new exported code to its contents
    node_template_file = open(PATH + "../templates/NODE_TEMPLATE")
    CONTENTS = node_template_file.read()
    CONTENTS = CONTENTS.replace("@@NODE_DEFINITIONS@@", NODE_DEFINITIONS)

    # Write the exported contents to the respective file
    parse_tree_file = open(PATH + "../target/src/ast.h", 'w+')
    parse_tree_file.write(CONTENTS)

    # Close the files
    parser_template_file.close()
    node_template_file.close()
    parser_file.close()
    parse_tree_file.close()
