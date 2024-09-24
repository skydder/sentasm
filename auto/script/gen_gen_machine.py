import sys
from lib import Match, Defun, display_list, FunCall, Block, read_list
from gen_gen import CodeGenMatchPat

class Grammar:
    def __init__(self, verb: str, instraction_determinants: tuple, instractions: list[tuple[tuple, list[str]]]) -> None:
        self.verb = verb
        self.instraction_determinants = instraction_determinants
        self.instractions = instractions

    # ("verb" ("instraction-determining-preps(IDP)") (("IDP's objects") ("instraction" "operand determinimg prep"*))*)
    def read(tokens: list[str]) -> 'Grammar':
        print('tokens:', tokens, file=sys.stderr)
        verb = tokens[0]
        print('verb:', verb, file=sys.stderr)
        instraction_determinants = tuple(tokens[1])
        print('idet:', instraction_determinants, file=sys.stderr)
        instractions = []
        for rule in tokens[2:]:
            instractions.append((tuple(rule[0]), rule[1]))
        print('ins:', instractions, file=sys.stderr)
        return Grammar(verb, instraction_determinants, instractions)

    def generate_proc(self) -> list[str]:
        proc = []
        for det in self.instraction_determinants:
            proc.append('let _{} = sentence.preposition_phrases.get_object(Preposition(\"{}\")).map_or_else(|| None, |date| date.expect_object());'.format(det, det))
        if len(self.instraction_determinants):
            proc.append(self.generate_match())
        else:
            for det, gen in self.instractions:
                proc.extend(Grammar.generate_callee(gen))
        return proc

    def generate_match(self) -> str:
        match_arm = '({})'.format(display_list(['&_{}'.format(item) for item in self.instraction_determinants]))
        match_patterns = []
        for det, gen in self.instractions:
            # print(det)
            match_patterns.append((CodeGenMatchPat(det).pat(), Block(Grammar.generate_callee(gen)).block()))

        return Match(match_arm, match_patterns).match()

    def generate_callee(rule: list[str]) -> list[str]:
        def make_operands(operands: list[str]) -> str:
            if len(operands) > 4:
                print('invalid number of operands!!', file=sys.stderr)
            tup = ['None', 'None', 'None', 'None']
            for i, operand in enumerate(operands):
                tup[i] = f'_{operand}'
            return '({})'.format(display_list(tup))
        
        block = []
        for prep in rule[1:]:
            block.append('let_prep!(_{}, "{}");'.format(prep, prep))
        block.append(FunCall('codegen', [f'"{rule[0]}"', make_operands(rule[1:])]).call())
        return block

    def generate(self) -> str:
        fn_name = 'analyze_{}'.format(self.verb)
        params = [('sentence', 'Sentence')]
        return_type = 'Result<String>'
        return Defun('pub', fn_name, params, return_type, self.generate_proc()).defun()

class Instruction:
    def __init__(self, operands: list[str], rule: list[str]) -> None:
        self.operands = operands
        self.rule = rule
    
    def read_operand(rule: str) -> str:
        def match_arm(token: str):
            if token.startswith('mem'):
                return 'match_data!(Memory{{size:{1}, ..}})'.format(read_postfix(token[3:]))
            elif token.startswith('reg'):
                return 'match_data!(Register({0}, {1}, ..))'.format(read_postfix(token[3:]))
            elif token.startswith('rm'):
                return 'match_data!(Register(_, {1}, ..)) | match_data!(Memory{{size:{1}, ..}})'.format(read_postfix(token[2:]))
            elif token.startswith('imm'):
                return 'match_data!(Immediate(..))'
            elif token.startswith('sbytedword'):
                return 'match_data!(Immediate(_))'
                
        def read_postfix(token: str):
            # in the future, support other register 
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
    
    def generate_operands(self) -> str:
        return '({})'.format(display_list([Instruction.read_operand(self.operands[i]) if i < len(self.operands) else 'None' for i in range(4)]))
    

if __name__ == '__main__':
    seq = read_list('grammar.dat')
    print(Grammar.read(seq[0]).generate())
