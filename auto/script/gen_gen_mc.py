import sys
import sys
from .lib import Match, Defun, Block, Words, Line, Code

# prefixes = ['o16', 'o32', 'odf', 'o64', 'o64nw', 'a16', 'a32', 'adf', 'a64', '!osp', '!asp', 'f2i', 'f3i', 'mustrep', 'mustrepne', 'rex.l', 'norexb', 'norexx', 'norexr', 'norexw', 'repe', 'nohi', 'nof3', 'norep', 'wait', 'resb', 'np', 'jcc8', 'jmp8', 'jlen', 'hlexr', 'hlenl', 'hle', 'vsibx', 'vm32x', 'vm64x', 'vsiby', 'vm32y', 'vm64y', 'vsibz', 'vm32z', 'vm64z']

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

prefixes = {
    'o16': 'ins.set_prefix(0x66);',
    'o32': '// 32 bit operand',
    'odf': '// default operand',
    'o64': 'ins.set_rex_w();',
    'o64nw': '// ins.set_rex();',
    'a16': '// unimplemented',
    'a32': '// unimplemented',
    'adf': '// unimplemented',
    'a64': '// unimplemented',
    '!osp': '// unimplemented',
    '!asp': '// unimplemented',
    'f2i': '// unimplemented',
    'f3i': '// unimplemented',
    'mustrep': '// unimplemented',
    'mustrepne': '// unimplemented',
    'rex.l': '// unimplemented',
    'norexb': '// unimplemented',
    'norexx': '// unimplemented',
    'norexr': '// unimplemented',
    'norexw': '// unimplemented',
    'repe': '// unimplemented',
    'nohi': '// unimplemented',
    'nof3': '// unimplemented',
    'norep': '// unimplemented',
    'wait': '// unimplemented',
    'resb': '// unimplemented',
    'np': '// unimplemented',
    'jcc8': '// unimplemented',
    'jmp8': '// unimplemented',
    'jlen': '// unimplemented',
    'hlexr': '// unimplemented',
    'hlenl': '// unimplemented',
    'hle': '// unimplemented',
    'vsibx': '// unimplemented',
    'vm32x': '// unimplemented',
    'vm64x': '// unimplemented',
    'vsiby': '// unimplemented',
    'vm32y': '// unimplemented',
    'vm64y': '// unimplemented',
    'vsibz': '// unimplemented',
    'vm32z': '// unimplemented',
    'vm64z': '// unimplemented',
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
    return (reg_value, reg_size, '_')


class MCEmit:
    def __init__(self, rule: list[str]) -> None:
        if rule[0].endswith(':'):
            self.define = rule[0][:-1]
            self.rule = rule[1::]
        else:
            self.define = None
            self.rule = rule

    def read_define(self, defs: str):
        defs = [i for i in defs]
        def is_rmvi(token):
            match token:
                case 'r' | 'm' | 'v' | 'i':
                    return True
                case _:
                    return False
        
        def is_plus(token):
            if token == '+':
                return True
            else:
                return False
        
        operands = []
        cur = []

        while len(defs) != 0:
            item = defs.pop(0)

            if item == '-':
                operands.append(cur)
                cur = []
            elif is_rmvi(item):
                operands.append(cur)
                cur = []
                cur.append(item)
            elif is_plus(item):
                item = defs.pop(0)
                if is_rmvi(item):
                    cur.append(item)
                else:
                    print('unexpected operand type\n-> ', item, file=sys.stderr)
                    exit(0)
            else:
                print('unexpected operand type\n-> ', item, file=sys.stderr)
                exit(0)
        operands.append(cur)
        return operands[1:]
                

    
    def assign(self):
        if self.define ==  None:
            return [Line('// void', 0)]
        code = []
        code.append(Line(f'// {self.define}', 0))
        seq = self.read_define(self.define)
        code.append(Line(f'// {seq}', 0))
        for (i, types) in enumerate(seq):
            if len(types) > 1:
                for ty in types[:-1]:
                    code.append(Line(f'let _{ty} = operands.{i}.clone().unwrap();', 0))
                code.append(Line(f'let _{types[-1]} = operands.{i}.unwrap();', 0))
            elif len(types) == 1:
                code.append(Line(f'let _{types[0]} = operands.{i}.unwrap();', 0))
        return code

    def is_prefix(self, candidate):
        return candidate in prefixes
    
    def is_mod_rm(self, candidate):
        return candidate.startswith('/')
    
    def is_imm(self, candidate):
        return candidate.startswith('i')
    
    def is_disp(self, candidate):
        return candidate.startswith('rel')
    

    def read_pefix(self, prefix):
        assert prefix in prefixes
        return [Line(prefixes[prefix], 0)]
    
    def read_mod_rm(self, mod_rm):
        assert mod_rm.startswith('/')
        code = [Line('ins.set_rm(_m);', 0)]
        match mod_rm[1:]:
            case 'r':
                code.append(Line('ins.set_reg(_r);',0))
            case '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9':
                code.append(Line('ins.set_mod_rm_reg({});'.format(mod_rm[1:]), 0))
            case _:
                print('unexpected value!!', file=sys.stderr)
        return code
    
    def read_imm(self, imm):
        assert imm.startswith('i')
        def read_size(s):
            match s:
                case 'b':
                    return 8
                case 'w':
                    return 16
                case 'd':
                    return 32
                case 'q':
                    return 64
        return [Line(f'ins.set_imm(_i, {read_size(imm[1])});', 0)]
    
    def read_disp(self, disp):
        assert disp.startswith('rel')
        return [Line('ins.set_disp(_i);', 0)]

    def read_opcode(self, opcode):
        if '+' in opcode:
            opcode = opcode.split('+')
            match opcode[1]:
                case 'r':
                    return [Line('ins.set_opecode_with_register(0x{}, _r);'.format(opcode[0]), 0)]
                case 'c':
                    return [Line('ins.set_opcode(0x{});'.format(opcode[0]), 0)]
                case _:
                    print('expected r after +\n',self.rule, file=sys.stderr)
                    exit(0)
        else:
            return [Line('ins.set_opcode(0x{});'.format(opcode), 0)]

    def read_byte(self, byte):
        if self.is_prefix(byte):
            return self.read_pefix(byte)
        elif self.is_mod_rm(byte):
            return self.read_mod_rm(byte)
        elif self.is_disp(byte):
            return self.read_disp(byte)
        elif self.is_imm(byte):
            return self.read_imm(byte)
        else:
            return self.read_opcode(byte)
    
    def parse(self):
        code = self.assign()
        for byte in self.rule:
            code.extend(self.read_byte(byte))
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
        return '("{}", Operands{})'.format(self.ins, Operands(self.operands))

class Operands:
    def __init__(self, operands) -> None:
        self.operands = operands

    def read_operand(rule: str) -> str:
        def match_arm(token: str):
            if token.startswith('mem'):
                return 'match_data!(Memory{{size:{1}, ..}})'.format(*read_postfix(token[3:]))
            elif token.startswith('reg'):
                return 'match_data!(Register(_, {1}, {0}, _, {2}))'.format(*read_postfix(token[3:]))
            elif token.startswith('rm'):
                return 'match_data!(Register(_, {1}, _, _, {2})) | match_data!(Memory{{size:{1}, ..}})'.format(*read_postfix(token[2:]))
            elif token.startswith('imm'):
                return 'match_data!(Immediate(_, {1}, ..))'.format(*read_postfix(token[3:]))
            elif token.startswith('sbytedword'):
                return 'match_data!(Immediate(_, _, true))'
                
        def read_postfix(token: str):
            # in the future, support other register 
            if token == '':
                return ('_', '_', '_')
            if token.startswith('_'):
                if token[1:] == 'sreg':
                    return ('_', '_', 'RegType::SReg')
                if token[1:] == 'offs':
                    return ('_', '_', '_')
                if token[1:] == 'creg':
                    return ('_', '_', 'RegType::CReg')
                if token[1:] == 'dreg':
                    return ('_', '_', 'RegType::DReg')
                return read_reg(token[1:])
            # in the future, support other postfixes.
            else:
                try:
                    return ('_', int(token), '_')
                except:
                    print('unexpected syntax, but for now, we axcept', file=sys.stderr)
                    return ('_', '_', '_')
        
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
        code.append(Line('Ok(ins)', 0))
        return (Words(str(InsMatch(self.ins, self.operand))), Block(code))

class Codegen:
    def __init__(self, rules) -> None:
        self.rules = rules
    
    def codegen(self):
        proc = [Line('let mut ins = Instruction::new();', 0)]
        proc.extend(Match(Words('(ins_name, &operands)'), self.ins()).match())
        return Defun(Words('pub'), 'emit_mc<\'a>', [('ins_name', Words('&str')), ('operands', Words('Operands<\'a>'))], Words('Result<Instruction, Operands<\'a>>'), proc).defun()

    def ins(self):
        matchs = []
        for ins in self.rules:
            matchs.append(Ins(ins).generate())
        matchs.append((Words('_'), Words('Err(operands)')))
        return matchs

def gen_import():
    code = []
    code.append(Line('use data::{', 0))
    code.append(Line('Register, Immediate, Memory, Data, DataSet, RegType', 1))
    code.append(Line('};', 0))
    code.append(Line('use crate::{Operands, Instruction};', 0))
    code.append(Line('use macros::match_data;', 0))
    code.append(Line('', 0))
    return code

def generate_gen_mc(rules):
    code = gen_import()
    code.append(Line('#[allow(warnings)]', 0))
    code.extend(Codegen(rules).codegen())
    return Code(code).generate()

def read_rule(path):
    with open(path, 't+r') as fp:
        return eval(fp.read())

if __name__ == '__main__':
    with open('t.dat', 't+r') as fp:
        print(generate_gen_mc(eval(fp.read())))

        

