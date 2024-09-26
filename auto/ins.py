import sys

def read(tokens: list[str]) -> list[any]:
    ins = []
    while len(tokens) != 0:
        seq = []
        token = tokens.pop(0)
        if token == '[':
            while len(tokens) != 0 and tokens[0] != ']':
                seq.append(tokens.pop(0))
            
            tokens.pop(0)
            ins.append(seq)
        else:
            ins.append(token)
    return ins

def read_ins(line: str) -> list[any]:
    tokens = line.replace('[', ' [ ').replace(']', ' ] ').split()
    if len(tokens) == 0 or line.startswith(';'):
        return []
    
    ins = read(tokens)
    ins[0] = ins[0].lower()
    ins[1] = ins[1].split(',')
    return ins

def read_file(path):
    with open(path) as file:
        lens = []
        for line in file.readlines():
            ins = read_ins(line)
            if len(ins):
                lens.append(ins[:3])
                # lens.setdefault(ins[0], {})[ins[1]] = ins[2]
                # if ((len(ins[2]) == 0 or 'v' not in ins[2][0]) and (len(ins[2]) < 2 or 'vex' not in ins[2][1]) and (len(ins[2]) < 3 or 'vex' not in ins[2][2]) 
                #     and ('sbytedword' not in ins[1])):
                #     print(ins[:3])
                if 'NOLONG' not in ins[3]:
                    print(ins[:3], file= sys.stderr)
        return lens

if __name__ == '__main__':
    print(read_file('X86_64'))


