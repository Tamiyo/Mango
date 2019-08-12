# Helper function, creates a new itemset struct for the CLR Algorithm
def create_itemset(A, a, B, b, t):
    return {
        'A': A.strip(),
        'a': a.strip(),
        'B': B.strip(),
        'b': b.strip(),
        't': t.strip(),
    }

# Helper function, combines two itemset structs for the CLR Algorithm
def combine_item(I):
    return I['A'], I['a'].strip() + I['B'].strip() + I['b'].strip()


# Debugging only, prints an itemset in a specified format
def print_itemset(C):
    iter = 0
    for set in C:
        print('I{0}'.format(iter))
        for i in set:
            print('\t[{0} -> {1} * {2} {3}, {4}]'.format(i['A'],
                                                         i['a'], i['B'], i['b'], i['t']))
        iter += 1


# Converts the grammar name to a converted name
def convert_to_ttype(token):
    return ''.join([str(x).replace("NTS", "").replace("TS", "").lower().capitalize() for x in token.split("_")])
