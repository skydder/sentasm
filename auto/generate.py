
from script.gen_data import GenData, read_data
from script.gen_gen import generate_codegen
from script.gen_gen_mc import generate_gen_mc, read_rule
from script.lib import read_list


def generate_data() -> str:
    tokens_path_names = map(lambda x: ('tokens/{}.dat'.format(x) , x), ['VERB', 'PREPOSITION', 'KEYWORD', 'PSEUDO', 'REG8', 'REG16', 'REG32', 'REG64'])
    data = ''
    for (path, name) in tokens_path_names:
        data += '{}'.format(GenData(name, read_data(path)).generate())
    return data

def generate_codegen_verb() -> str:
    verb = read_data('tokens/VERB.dat')
    grammars = read_list('grammar.dat')
    return generate_codegen(verb, grammars)

def generate_codegen_mc() -> str:
    rules = read_rule('tokens/rules.dat')
    return generate_gen_mc(rules)

if __name__ == '__main__':
    with open('../data/src/data_auto.rs', 'wt') as file:
        print(generate_data(), file=file)
    with open('../codegen/src/codegen_verb.rs', 'wt') as file:
        print(generate_codegen_verb(), file=file)
    # with open('../codegen/src/codegen_mc.rs', 'wt') as file:
    #     print(generate_codegen_mc(), file=file)

