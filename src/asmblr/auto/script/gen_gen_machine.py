import sys
from .lib import Match, Defun, display_list, FunCall, Block
from .gen_gen import CodeGenMatchPat

class Grammar:
    def __init__(self, verb: str, instraction_determinants: tuple, instractions: list[tuple[tuple, list[str]]]) -> None:
        self.verb = verb
        self.instraction_determinants = instraction_determinants
        self.instractions = instractions

    # ("verb" ("instraction-determining-preps(IDP)") (("IDP's objects") ("instraction" "operand determinimg prep"*))*)
    def read(tokens: list[str]) -> 'Grammar':
        verb = tokens[0]
        instraction_determinants = tuple(tokens[1])
        instractions = []
        for rule in tokens[1:]:
            instractions.append((tuple(rule[:1]), rule[1:]))
        return Grammar(verb, instraction_determinants, instractions)

    def generate_proc(self) -> list[str]:
        proc = []
        for det in self.instraction_determinants:
            proc.append('let _{} = sentence.preposition_phrases.get_object(Preposition(\"{}\")).map_or_else(|| None, |date| date.expect_object());'.format(det, det))
        
        proc.append(self.generate_match())
        return proc

    def generate_match(self) -> str:
        match_arm = '({})'.format(display_list(['&_{}'.format(item) for item in self.instraction_determinants]))
        def make_operands(operands: list[str]) -> str:
            if len(operands) > 4:
                print('invalid number of operands!!', file=sys.stderr)
            tup = ['None', 'None', 'None', 'None']
            for i, operand in enumerate(operands):
                tup[i] = operand
            return '({})'.format(display_list(tup))
        
        match_patterns = []
        for det, gen in self.instractions:
            block = []
            for prep in gen[1:]:
                block.append('let _{} = sentence.preposition_phrases.get_object(Preposition(\"{}\")).map_or_else(|| None, |date| date.expect_object());'.format(prep, prep))
            block.append(FunCall('codegen', ['"{gen[0]}"', make_operands(gen[1:])]).call())
            match_patterns.append((CodeGenMatchPat(det).pat(), Block(block).block()))

        return Match(match_arm, match_patterns).match()


    def generate(self) -> str:
        fn_name = 'analyze_{}'.format(self.verb)
        params = [('sentence', 'Sentence')]
        return_type = 'Result<String>'
        return Defun('pub', fn_name, params, return_type, self.generate_proc()).defun()
    
if __name__ == '__main__':
    with open('grammar.dat') as file:
        pass
