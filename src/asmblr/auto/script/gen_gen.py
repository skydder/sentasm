import sys
from .lib import Match, Defun, FunCall, display_list, read, tokenize
class GenIns:
    def __init__(self, verb_name: str, parameters: list[str], generate_rules: list[str]) -> None:
        self.verb_name = verb_name
        self.parameters = parameters
        self.generate_rules = generate_rules

    def gen_proc(self) -> list[str]:
        seq = []

        for parameter in self.parameters:
            seq.append('let _{} = sentence.preposition_phrases.get_object(Preposition(\"{}\")).map_or_else(|| None, |date| date.expect_object());'.format(parameter, parameter))
        
        seq.append("")
        seq.append(self.gen_match())
        return seq

    def gen_match_arm(self) -> str:
        return '({})'.format(display_list(['&_{}'.format(item) for item in self.parameters]))
    
    def gen_match(self) -> str:
        match_patterns: list[(str, str)] = []
        for pattern in self.generate_rules:
            match_patterns.append((CodeGenMatchPat(pattern[:len(pattern) - 1]).pat(), CodeGenMatchExpr(pattern[-1]).match_expr4nasm()))
        match_patterns.append(('_', CodeGenMatchExpr.match_expr_of_rest()))
        
        return Match(self.gen_match_arm(), match_patterns).match()
    
    def gen_ins(self) -> 'Defun':
        params = [('sentence', 'Sentence')]
        return Defun('','gen_ins_{}'.format(self.verb_name.replace('-', '_').replace('*', '_')), params, 'Result<String>', self.gen_proc()).defun()

# Reg | Imm | Mem | Label | 
class CodeGenMatchPat:
    def __init__(self, match_pattern: str) -> None:
        self.match_pattern = match_pattern
    
    # name = ':' str
    # size = '_' num
    # param = (mem | reg | label | keyword | imm) size* name*
    # params = param ('|' param)*
    @staticmethod
    def read_param(param_s: str) -> list[str]:
        return param_s.replace(':', ' : ').replace('_', ' _ ').replace('|', ' | ').split()
    
    @staticmethod
    def match_pattern_pat(param: list[str]) -> str:
        token = param.pop(0)
        pat = ''
        match token:
            case 'mem':
                pat += 'CaseSome!(Data::Memory(Memory{{size:{}, ..}}))'.format(CodeGenMatchPat.read_size(param))
            case 'reg':
                pat += 'CaseSome!(Data::Register(Register({}, {}, ..)))'.format(CodeGenMatchPat.read_name(param), CodeGenMatchPat.read_size(param))
            case 'label':
                pat += 'CaseSome!(Data::Label(_))'
            case 'imm':
                pat += 'CaseSome!(Data::Immediate(Immediate(_)))'
            case 'keyword':
                pat += 'CaseSome!(Data::Keyword(Keyword({})))'.format(CodeGenMatchPat.read_name(param))
            case 'None':
                pat += 'None'
        
        try:
            if param[0] == '|':
                param.pop(0)
                return pat + ' | ' + CodeGenMatchPat.match_pattern_pat(param)
            else:
                return pat
        except:
            return pat
            
    @staticmethod
    def read_size(param: list[str]) -> str:
        try:
            if param[0] == '_':
                param.pop(0)
                return str(int(param.pop(0))) + ' | 0'
            else:
                return '_'
        except:
            return '_'
    
    def read_name(param: list[str]) -> int:
        try:
            if param[0] == ':' and len(param[1]) != 0:
                param.pop(0)
                return '\"{}\"'.format(param.pop(0))
            else:
                return '_'
        except:
            return '_'
        

    def pat(self) -> str:
        return '({})'.format(display_list(['{}'.format(self.match_pattern_pat(CodeGenMatchPat.read_param(item))) for item in self.match_pattern]))


class CodeGenMatchExpr:
    def __init__(self, gen_rule: list[str]):
        self.gen_rule = gen_rule
    
    def match_expr4nasm(self) -> str:
        params = []
        match_code = "Ok(format!(\""
        for s in self.gen_rule:
            if s.startswith('#'):
                params.append(s.replace('#', '_')+'.unwrap()')
                match_code += ' {:?}'
            else:
                match_code += '{}'.format(s)
        match_code += '\", '

        match_code += '{}))'.format(display_list(params))
        return match_code
    
    @staticmethod
    def match_expr_of_rest() -> str:
        return '{\n\t\temit_error_msg!(\"unmatched operand\", sentence.verb_loc);\n\t\tErr(())\n\t}'

def gen_import() -> str:
    return 'use super::super::{\n\tdata::{Keyword, Register, Immediate, Memory}, Code, Data, DataSet, Loc, Preposition, PrepositionPhrases, Result, Verb, Sentence\n};\n'

def gen_macro() -> str:
    return 'macro_rules! CaseSome {\n\t($data:pat) => {Some(DataSet {data:$data, loc:_})};\n}\nmacro_rules! emit_error_msg {\n\t($msg:expr, $loc:expr) => {\n\t\teprintln!("{}", format!("{}{}", $msg, $loc))\n};\n}\n'

def gen_codegen_verb(verb: list[str]) -> str:
    params = [('sentence', 'Sentence')]
    pat = lambda x: 'Verb(\"{}\")'.format(x)
    expr = lambda x: FunCall('gen_ins_{}'.format(x.replace('-', '_').replace('*', '_')), [i for (i, _) in params]).call()
    match_pat = lambda x: (pat(x), expr(x))
    match_patterns = [match_pat(item) for item in verb]
    match_patterns.append(('_', 'todo!()'))
    match_sentence = Match('sentence.verb', match_patterns).match()
    return Defun('pub','codegen_verb', params, 'Result<String>', [match_sentence]).defun()



if __name__ == '__main__':
    with open('grammar.dat') as file:
        print(gen_import())
        print()
        print(gen_macro())
        # codegen = Defun('codegen_ins', [('_verb', 'Verb'), ('sentence.verb_loc', 'Loc'), ('_object', 'Option<DataSet>'), ('_preposition_phrases', '&mut PrepositionPhrases')])
        for grammar in read(tokenize(file.read())):
            print()
            code = GenIns(grammar[0], grammar[1], grammar[2:])
            code.gen_ins()
        