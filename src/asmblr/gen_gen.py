import sys

class Defun:
    def __init__(self, fn_name: str, params: list[(str, str)], return_type: str, procedure: list[str]) -> None:
        self.fn_name = fn_name
        self.params = params
        self.return_type = return_type
        self.procedure = procedure

    def defun(self) -> str:
        code = 'pub(crate)fn {}({}) -> {} {{\n'.format(self.fn_name, self.param(), self.return_type)
        for step in self.procedure:
            code += '\t{}\n'.format(step.replace('\n', '\n\t'))

        code += '}'
        return code

    def param(self) -> str:
        code = ""
        for (var, _type) in self.params[:len(self.params) - 1]:
            code += '{}: {}, '.format(var, _type)
        
        code += '{}: {}'.format(*self.params[-1])
        return code

class Match:
    def __init__(self, match_arm: str, match_patterns: list[(str, str)]) -> None:
        self.match_arm = match_arm
        self.match_patterns = match_patterns

    def match(self) -> str:
        code = 'match {} {{\n'.format(self.match_arm)
        for (pat, expr) in self.match_patterns:
            code += '\t{} => {},\n'.format(pat, expr)

        code += '}'
        return code

###################
def tokenize_grammar(s: str) -> list:
    tokens = s.replace('(', ' ( ').replace(')', ' ) ').replace(',', ' , ').split()
    return tokens

def read_tokens(tokens: list[str]) -> list:
    seq = []
    token = tokens.pop(0)
    if token == '(':
        while tokens[0] != ')':
            seq.append(read_tokens(tokens))
        
        tokens.pop(0)
        return seq
    else:
        return token

def read(tokens: list[str]) -> list:
    seq = []
    while len(tokens) != 0 and tokens[0] == '(':
        seq.append(read_tokens(tokens))
    return seq


class InsGenerater:
    def __init__(self, verb_name: str, parameters: list[str], generate_rules: list[str]) -> None:
        self.verb_name = verb_name
        self.parameters = parameters
        self.generate_rules = generate_rules

    def gen_proc(self) -> list[str]:
        seq = []
        seq.append('let _obj = _object.map_or_else(|| None, |date| date.expect_object());'.format())
        
        for parameter in self.parameters[1::]:
            seq.append('let _{} = _preposition_phrases.get_object(Preposition(\"{}\")).map_or_else(|| None, |date| date.expect_object());'.format(parameter, parameter))
        
        seq.append("")
        seq.append(self.gen_match())
        return seq

    def gen_match_arm(self) -> str:
        match_arm = '('

        for parameter in self.parameters[:len(self.parameters) - 1]:
            match_arm += '&_{}, '.format(parameter)
        
        match_arm += '&_{}'.format(self.parameters[-1])

        match_arm += ')'
        return match_arm
    
    def gen_match(self) -> str:
        match_patterns = []
        for pattern in self.generate_rules:
            match_patterns.append((CodeGenMatchPat(pattern[:len(pattern) - 1]).pat(), CodeGenMatchExpr(pattern[-1]).match_expr4nasm()))
        match_patterns.append(('_', CodeGenMatchExpr.match_expr_of_rest()))
        
        return Match(self.gen_match_arm(), match_patterns).match()
    
    def gen_func(self) -> 'Defun':
        params = [('_verb', 'Verb'), ('_verb_loc', 'Loc'), ('_object', 'Option<DataSet>'), ('_preposition_phrases', '&mut PrepositionPhrases')]
        return Defun('gen_ins_{}'.format(self.verb_name.replace('-', '_').replace('*', '_')), params, 'Result<String>', self.gen_proc())

    def generate(self, file=sys.stdout):
        print(self.gen_func().defun(), file=file)

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
        # print('____', file=sys.stderr)
        # print(param, file=sys.stderr)
        token = param.pop(0)
        pat = ''
        match token:
            case 'mem':
                pat += 'CaseSome!(Data::Memory(Memory{{size:{}, ..}}))'.format(CodeGenMatchPat.read_size(param))
            case 'reg':
                pat += 'CaseSome!(Data::Register(Register({}, {}, _)))'.format(CodeGenMatchPat.read_name(param), CodeGenMatchPat.read_size(param))
            case 'label':
                pat += 'CaseSome!(Data::Label(_))'
            case 'imm':
                pat += 'CaseSome!(Data::Immediate(Immediate(_)))'
            case 'keyword':
                pat += 'CaseSome!(Data::Keyword(Keyword({})))'.format(CodeGenMatchPat.read_name(param))
            case 'None':
                pat += 'None'
        # print(param, file=sys.stderr)
        try:
            if param[0] == '|':
                param.pop(0)
                return pat + ' | ' + CodeGenMatchPat.match_pattern_pat(param)
            else:
                # print('else', file=sys.stderr)
                return pat
        except:
            # print('except', file=sys.stderr)
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
        match_pattern = '('
        for parameter in self.match_pattern[:len(self.match_pattern) - 1]:
            match_pattern += '{}, '.format(self.match_pattern_pat(CodeGenMatchPat.read_param(parameter)))
        
        match_pattern += self.match_pattern_pat(CodeGenMatchPat.read_param(self.match_pattern[-1]))
        
        match_pattern += ')'
        return match_pattern


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

        for param in params[:len(params) - 1]:
            match_code += '{}, '.format(param)
        if len(params) != 0:
            match_code += params[-1]
        match_code += '))'

        return match_code
    
    @staticmethod
    def match_expr_of_rest() -> str:
        return '{\n\t\temit_error_msg!(\"unmatched operand\", _verb_loc);\n\t\tErr(())\n\t}'

def gen_import() -> str:
    return 'use crate::emit_error_msg;\nuse super::{\n\tdata::{Keyword, Register, Immediate, Memory}, Code, Data, DataSet, Loc, Preposition, PrepositionPhrases, Result, Verb\n};'

def gen_macro() -> str:
    return 'macro_rules! CaseSome {\n\t($data:pat) => {Some(DataSet {data:$data, loc:_})};\n}'



if __name__ == '__main__':
    with open('src/asmblr/grammar.dat') as file:
        print(gen_import())
        print()
        print(gen_macro())
        for grammar in read(tokenize_grammar(file.read())):
            print()
            code = InsGenerater(grammar[0], grammar[1], grammar[2:])
            code.generate()