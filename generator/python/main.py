import copy

import algorithm as algorithm
import algorithm_steps as algorithm_steps
import exporter as exporter
import settings as settings
from settings import GRAMMAR

FIRST_SET = None
FOLLOW_SET = None


# Presetup for generating c++ code
def first_follow_presetup():
    global FIRST_SET, FOLLOW_SET
    tGrammar = copy.deepcopy(GRAMMAR)
    FIRST_SET = algorithm_steps.FIRST(tGrammar)

    tGrammar = copy.deepcopy(GRAMMAR)
    tFirst = copy.deepcopy(FIRST_SET)
    FOLLOW_SET = algorithm_steps.FOLLOW(tGrammar, tFirst)


# Runs the c++ code generation. This will generate 2 files located at /generator/target/src/
# that are a parser.h file and a parse_tree.h file. These should be moved to / when you are
# ready to merge the newly created files over.
def main():
    print('Beginning Parse Table Generation...')
    settings.init()
    first_follow_presetup()
    algorithm.generate_parse_table()
    print('Parse Table Generation Complete!')
    print('Exporting Generated Code...')
    exporter.export_parser()
    print('Exporting Complete!')


if __name__ == '__main__':
    main()
