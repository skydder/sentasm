from .lib import Match, Defun, FunCall, display_list, read, tokenize, Words, Line, Block, Code
class GenIns:
    def __init__(self, verb_name: str, parameters: list[str], generate_rules: list[str]) -> None:
        self.verb_name = verb_name
        self.parameters = parameters
        self.generate_rules = generate_rules

    def gen_proc(self):
        seq = []

        for parameter in self.parameters:
            seq.append(Line('let (_{}, _) = get_prep_object!(sentence, "{}", sentence.location);'.format(parameter, parameter), 0))
        
        seq.append(Line('', 0))
        seq.extend(self.gen_match())
        return seq

    def gen_match_arm(self):
        if len(self.parameters) == 1:
            return Words('{}'.format(display_list(['&_{}'.format(item) for item in self.parameters])))
        else:
            return Words('({})'.format(display_list(['&_{}'.format(item) for item in self.parameters])))
    
    def gen_match(self):
        match_patterns= []
        for pattern in self.generate_rules:
            match_patterns.append((CodeGenMatchPat(pattern[:len(pattern) - 1]).pat(), CodeGenMatchExpr(pattern[-1]).match_expr4nasm()))
        match_patterns.append((Words('_'), CodeGenMatchExpr.match_expr_of_rest()))
        
        return Match(self.gen_match_arm(), match_patterns).match()
    
    def gen_ins(self) -> 'Defun':
        params = [('sentence', Words('Sentence'))]
        return Defun(None, 'gen_ins_{}'.format(self.verb_name.replace('-', '_').replace('*', '_')), params, Words('Result<String>'), self.gen_proc()).defun()

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
                # pat += 'CaseSome!(Data::Memory(Memory{{size:{}, ..}}))'.format(CodeGenMatchPat.read_size(param))
                pat += 'match_data!(Memory{{size:{}, ..}})'.format(CodeGenMatchPat.read_size(param))
            case 'reg':
                # pat += 'CaseSome!(Data::Register(Register({}, {}, ..)))'.format(CodeGenMatchPat.read_name(param), CodeGenMatchPat.read_size(param))
                pat += 'match_data!(Register({}, {}, ..))'.format(CodeGenMatchPat.read_name(param), CodeGenMatchPat.read_size(param))
            
            case 'label':
                pat += 'match_data!(Label(_))'
                # pat += 'CaseSome!(Data::Label(_))'
            case 'imm':
                pat += 'match_data!(Immediate(_, _, _))'
            case 'keyword':
                pat += 'match_data!(Keyword({}))'.format(CodeGenMatchPat.read_name(param))
                # pat += 'CaseSome!(Data::Keyword(Keyword({})))'.format(CodeGenMatchPat.read_name(param))
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
                return '\"{}\"'.format(param.pop(0).replace('-', '_'))
            else:
                return '_'
        except:
            return '_'
        

    def pat(self) -> str:
        if len(self.match_pattern) == 1:
            return Words('{}'.format(display_list(['{}'.format(self.match_pattern_pat(CodeGenMatchPat.read_param(item))) for item in self.match_pattern])))
        else:
            return Words('({})'.format(display_list(['{}'.format(self.match_pattern_pat(CodeGenMatchPat.read_param(item))) for item in self.match_pattern])))


class CodeGenMatchExpr:
    def __init__(self, gen_rule: list[str]):
        self.gen_rule = gen_rule
    
    def match_expr4nasm(self) -> str:
        # match_code = 'Ok(gen_nasm!('

        # def stringify(x: str) -> str:
        #     if x.startswith('#'):
        #         return x.replace('#', '_')+'.unwrap()'
        #     else:
        #         return '"{}"'.format(x)

        # match_code += '{}))'.format(display_list(list(map(stringify, self.gen_rule))))
        
        def stringify(x: str) -> str:
            if x.startswith('#'):
                return x.replace('#', '_')
            else:
                return '"{}"'.format(x)
            
        rule = list(map(stringify, self.gen_rule))
        match_code = f'Ok(nasm({rule[0]}, make_operands!('
        match_code += '{})))'.format(display_list(rule[1:]))
        return Words(match_code)
    
    @staticmethod
    def match_expr_of_rest() -> str:
        return Block([Line('emit_error!(sentence.location, \"unmatched operand\");', 0), Line('Err(())', 0)])

def gen_import():
    code = []
    code.append(Line('use data::{', 0))
    code.append(Line('Keyword, Register, Immediate, Memory, Data, DataSet, Preposition, Result, Verb, Sentence, Label', 1))
    code.append(Line('};', 0))
    code.append(Line('use macros::{match_data, get_prep_object, make_operands};', 0))
    code.append(Line('use tokenizer::emit_error;', 0))
    code.append(Line('use crate::{Operands, nasm};', 0))
    code.append(Line('', 0))
    return code

def gen_codegen_verb(verb: list[str]) -> str:
    params = [('sentence', Words('Sentence'))]
    pat = lambda x: Words('Verb(\"{}\")'.format(x))
    expr = lambda x: FunCall('gen_ins_{}'.format(x.replace('-', '_').replace('*', '_')), [i for (i, _) in params]).call()
    match_pat = lambda x: (pat(x), expr(x))
    match_patterns = [match_pat(item) for item in verb]
    match_patterns.append((Words('_'), Words('todo!()')))
    match_sentence = Match(Words('sentence.verb'), match_patterns).match()
    return Defun(Words('pub'),'codegen_verb', params, Words('Result<String>'),  match_sentence).defun()

def generate_codegen(verb, grammars) -> str:
    code = []
    code.extend(gen_import())
    code.extend(gen_codegen_verb(verb))
    
    for grammar in grammars:
        code.append(Line('#[allow(warnings)]', 0))
        code.extend(GenIns(grammar[0], grammar[1], grammar[2:]).gen_ins())

    return Code(code).generate()