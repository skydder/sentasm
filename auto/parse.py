prefixes = ['o16', 'o32', 'odf', 'o64', 'o64nw' 'a16', 'a32', 'adf', 'a64', '!osp', '!asp', 'f2i', 'f3i', 'mustrep', 'mustrepne', 'rex.l', 'norexb', 'norexx', 'norexr', 'norexw', 'repe', 'nohi', 'nof3', 'norep', 'wait', 'resb', 'np', 'jcc8', 'jmp8', 'jlen', 'hlexr', 'hlenl', 'hle', 'vsibx', 'vm32x', 'vm64x', 'vsiby', 'vm32y', 'vm64y', 'vsibz', 'vm32z', 'vm64z']

class Codegen:
    def __init__(self, rule: list[str]) -> None:
        self.rule = rule[1::]
    
    def is_prefix(self, prefix: str) -> bool:
        for pf in prefixes:
            if prefix == pf:
                return True
        return False
    
    def is_other(self, hexa: str) -> bool:
        if hexa.startswith('i'):
            return True
        elif hexa.startswith('/'):
            return True
        elif hexa.startswith('rel'):
            return True
        return False
    
    def parse_prefix(self, n):
        prefix = self.rule[n]
        # print(type(prefix))
        while self.is_prefix(prefix):
            if prefix == 'o64':
                print('ins.set_rex_w()')
            elif prefix == 'o64nw':
                print('ins.set_rex()')
            elif prefix == 'o16':
                print('ins.set_prefix(0x66)')
            elif prefix == 'o32':
                print()

            n += 1                
            prefix = self.rule[n]
            # print(prefix)
        return n
        
    def parse_opcode(self, n):
        try:
            opcode = self.rule[n]
            while not self.is_other(opcode):
                if '+' in opcode:
                    opcode = opcode.split('+')
                    if opcode[1] == 'r':
                        print('ins.set_opecode_with_register(0x{}, _r)'.format(opcode[0]))
                else:
                    print('ins.set_opcode(0x{})'.format(opcode))
                n += 1                
                opcode = self.rule[n]
        finally:
            return n
    
    def parse_mod(self, n):
        # print(n)
        try:
            mod = self.rule[n]
            if mod.startswith('/'):
                print('ins.set_rm(_m)')
                match mod[1:]:
                    case 'r':
                        print('ins.set_reg(_r)')
                    case '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7':
                        print('ins.set_mod_rm_reg({})'.format(mod[1:]))
                n += 1    
        finally:
            return n
        
        
    def parse_rel(self, n):
        # print(n)
        try:
            rel = self.rule[n]
            if rel.startswith('rel'):
                print('ins.set_disp(_i)')
                n += 1
        finally:
            return n

    def parse_imm(self, n):
        try:
            imm = self.rule[n]
            if imm.startswith('i'):
                print('ins.set_imm(_i)')
                n += 1
        finally:
            return n
    
    def parse(self):
        n = 0
        n = self.parse_prefix(n)
        n = self.parse_opcode(n)
        n = self.parse_mod(n)
        n = self.parse_rel(n)
        n = self.parse_imm(n)

def test():
    Codegen(['o32', '11', '/r']).parse()
    print("########")
    Codegen(['hle', 'o64', '83', '/2', 'ib,s']).parse()
    print("########")
    Codegen(['hle', 'o64', '83', '/2', 'ib,s']).parse()
    print("########")
    Codegen(['hle', 'o64', '83', '/2', 'ib,s']).parse()
    print("########")
    Codegen(['o32', '0f', 'c8+r']).parse()

def test2():
    print(InsMatch('bsr', ['reg64', 'reg64']))
    print(InsMatch('btc', ['rm16', 'imm8']))


    


class InsMatch:
    def __init__(self, ins, operands) -> None:
        self.ins = ins
        self.operands = operands

    def __str__(self) -> str:
        return '("{}", {})'.format(self.ins, Operands(self.operands))

class Operands:
    def __init__(self, operands) -> None:
        self.operands = operands

    def read_operand(rule: str) -> str:
        def match_arm(token: str):
            if token.startswith('mem'):
                return 'match_data!(Memory{{size:{1}, ..}})'.format(*read_postfix(token[3:]))
            elif token.startswith('reg'):
                return 'match_data!(Register({0}, {1}, ..))'.format(*read_postfix(token[3:]))
            elif token.startswith('rm'):
                return 'match_data!(Register(_, {1}, ..)) | match_data!(Memory{{size:{1}, ..}})'.format(*read_postfix(token[2:]))
            elif token.startswith('imm'):
                return 'match_data!(Immediate(_, {1}, ..))'.format(*read_postfix(token[3:]))
            elif token.startswith('sbytedword'):
                return 'match_data!(Immediate(..))'
                
        def read_postfix(token: str):
            # in the future, support other register 
            if token == '':
                return ('_', '_')
            if token.startswith('_'):
                match token[1:]:
                    case 'al':
                        return (0, 8)
                    case 'ax':
                        return (0, 16)
                    case 'eax':
                        return (0, 32)
                    case 'rax':
                        return (0, 64)
            # in the future, support other postfixes.
            else:
                return ('_', int(token))
        
        return match_arm(rule)
    
    def convert_operands(self, operands: list[str]) -> list[str]:
        seq = ['None', 'None', 'None', 'None']
        if len(operands) == 1 and operands[0] == 'void':
            return seq
        for (i, opr) in enumerate(operands):
            seq[i] = Operands.read_operand(opr)

        return seq

    def __str__(self) -> str:
        return '({}, {}, {}, {})'.format(*self.convert_operands(self.operands))

class Test:
    def __init__(self, ins) -> None:
        self.ins = ins[0]
        self.operand = ins[1]
        self.rule = ins[2]
    
    def test(self):
        print("test")
        print('arm: {}'.format(InsMatch(self.ins, self.operand)))
        Codegen(self.rule).parse()


if __name__ == '__main__':
    # test()
    # test2()
    with open('t.dat', 't+r') as fp:
        for i in eval(fp.read()):
            try:    
                Test(i).test()
            finally:
                pass

        

