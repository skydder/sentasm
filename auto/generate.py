
from script.gen_data import GenData, read_data
from script.gen_gen import GenIns, gen_codegen_verb, gen_import
from script.lib import read_list


def generate_data() -> str:
    tokens_path_name = lambda x: ('tokens/{}.dat'.format(x) , x)
    data = ''
    for (path, name) in map(tokens_path_name, ['VERB', 'PREPOSITION', 'KEYWORD', 'PSEUDO', 'REG8', 'REG16', 'REG32', 'REG64']):
        data += '{}\n'.format(GenData(name, read_data(path)).generate())
    return data

def generate_codegen_verb() -> str:
    code = '{}\n'.format(gen_import())
    verb = read_data('tokens/VERB.dat')
    code += '{}\n'.format(gen_codegen_verb(verb))
    grammars = read_list('grammar.dat')
    for grammar in grammars:
        code += '{}\n'.format(GenIns(grammar[0], grammar[1], grammar[2:]).gen_ins())
    
    return code

if __name__ == '__main__':
    with open('../data/src/data_auto.rs', 'wt') as file:
        print(generate_data(), file=file)
    with open('../codegen/src/codegen_verb.rs', 'wt') as file:
        print(generate_codegen_verb(), file=file)