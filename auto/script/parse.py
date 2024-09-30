import sys
import sys
from lib import Match, Defun, Block

prefixes = ['o16', 'o32', 'odf', 'o64', 'o64nw', 'a16', 'a32', 'adf', 'a64', '!osp', '!asp', 'f2i', 'f3i', 'mustrep', 'mustrepne', 'rex.l', 'norexb', 'norexx', 'norexr', 'norexw', 'repe', 'nohi', 'nof3', 'norep', 'wait', 'resb', 'np', 'jcc8', 'jmp8', 'jlen', 'hlexr', 'hlenl', 'hle', 'vsibx', 'vm32x', 'vm64x', 'vsiby', 'vm32y', 'vm64y', 'vsibz', 'vm32z', 'vm64z']

reg_table = {
    'ax': 0,
    'cx': 1,
    'dx': 2,
    'bx': 3,
    'sp': 4,
    'bp': 5,
    'si': 6,
    'di': 7,
    'al': 0,
    'cl': 1,
    'dl': 2,
    'bl': 3,
    'ah': 4,
    'ch': 5,
    'dh': 6,
    'bh': 7,
}

def read_reg(reg: str):
    reg_value = reg_table[reg[-2:]]
    reg_size = 0
    match reg[:-2]:
        case 'e':
            reg_size = 32
        case 'r':
            reg_size = 64
        case '':
            if reg.endswith('l') or reg.endswith('h'):
                reg_size = 8
            else:
                reg_size = 16
        case _:
            print('error: invalid syntax', file=sys.stderr)
    return (reg_value, reg_size)


class MCEmit:
    def __init__(self, rule: list[str]) -> None:
        self.rule = rule[1::]

    def read_item(self, item: str):
        item = item[:-1]
        def read_plus(seq, n):
            plus = seq[n + 1]
            



    def define_item(self): 

        pass
    
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
    
    def parse_prefix(self, n, code):
        prefix = self.rule[n]
        while self.is_prefix(prefix):
            if prefix == 'o64':
                code.append('ins.set_rex_w();')
            elif prefix == 'o64nw':
                code.append('ins.set_rex();')
            elif prefix == 'o16':
                code.append('ins.set_prefix(0x66);')
            elif prefix == 'o32':
                pass
            n += 1
            prefix = self.rule[n]
            
        
        return (n, code)
        
    def parse_opcode(self, n, code):
        try:
            opcode = self.rule[n]
            while not self.is_other(opcode):
                if '+' in opcode:
                    opcode = opcode.split('+')
                    if opcode[1] == 'r':
                        code.append('ins.set_opecode_with_register(0x{}, _r);'.format(opcode[0]))
                else:
                    code.append('ins.set_opcode(0x{});'.format(opcode))
                n += 1                
                opcode = self.rule[n]
        finally:
            return (n, code)
    
    def parse_mod(self, n, code):
        try:
            mod = self.rule[n]
            if mod.startswith('/'):
                code.append('ins.set_rm(_m);')
                match mod[1:]:
                    case 'r':
                        code.append('ins.set_reg(_r);')
                    case '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7':
                        code.append('ins.set_mod_rm_reg({});'.format(mod[1:]))
                n += 1    
        finally:
            return (n, code)
        
        
    def parse_rel(self, n, code):
        try:
            rel = self.rule[n]
            if rel.startswith('rel'):
                code.append('ins.set_disp(_i);')
                n += 1
        finally:
            return (n, code)

    def parse_imm(self, n, code):
        try:
            imm = self.rule[n]
            if imm.startswith('i'):
                code.append('ins.set_imm(_i);')
                n += 1
        finally:
            return (n, code)
    
    def parse(self):
        (_, code) = self.parse_imm(*self.parse_rel(*self.parse_rel(*self.parse_mod(*self.parse_opcode(*self.parse_prefix(0, []))))))
        return code

def test():
    MCEmit(['o32', '11', '/r']).parse()
    print("########")
    MCEmit(['hle', 'o64', '83', '/2', 'ib,s']).parse()
    print("########")
    MCEmit(['hle', 'o64', '83', '/2', 'ib,s']).parse()
    print("########")
    MCEmit(['hle', 'o64', '83', '/2', 'ib,s']).parse()
    print("########")
    MCEmit(['o32', '0f', 'c8+r']).parse()

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
                if token[1:] == 'sreg':
                    return ('_', '_')
                if token[1:] == 'offs':
                    return ('_', '_')
                if token[1:] == 'creg':
                    return ('_', '_')
                if token[1:] == 'dreg':
                    return ('_', '_')
                return read_reg(token[1:])
            # in the future, support other postfixes.
            else:
                try:
                    return ('_', int(token))
                except:
                    print('unexpected syntax, but for now, we axcept', file=sys.stderr)
                    return ('_', '_')
        
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
        MCEmit(self.rule).parse()

class Ins:
    def __init__(self, ins) -> None:
        self.ins = ins[0]
        self.operand = ins[1]
        self.rule = ins[2]
    
    def generate(self):
        code = MCEmit(self.rule).parse()
        code.append('ins')
        return (str(InsMatch(self.ins, self.operand)), Block(code).block())

class Codegen:
    def __init__(self, rules) -> None:
        self.rules = rules
    
    def generate(self): 
        proc = Match('(&ins, &operands)', self.ins()).match()
        return Defun('pub', 'emit_mc', [('ins', '&str'), ('operands', 'Operands')], 'Instruction', [proc]).defun()

    def ins(self):
        matchs = []
        for ins in self.rules:
            matchs.append(Ins(ins).generate())
        return matchs

if __name__ == '__main__':
    with open('t.dat', 't+r') as fp:
        print(Codegen(eval(fp.read())).generate())

        

