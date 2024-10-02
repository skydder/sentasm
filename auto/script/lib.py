import sys

# I'm going to manage rust codes by 'Line'

class Code:
    def __init__(self, code: list['Line']) -> None:
        self.code = code

    def generate(self):
        return ''.join([str(line) for line in self.code])

class Line:
    def __init__(self, line, indent):
        self.line = line
        self.indents = indent
    
    def __str__(self) -> str:
        return '\t' * self.indents + self.line + '\n'
    
    def __repr__(self) -> str:
        return f'Stmt(line="{self.line}", indent={self.indents})'
    
    def indent(self):
        self.indents += 1
    
    def into_words(self):
        return Words(self.line)

# this class represents expr'ish' code
class Words:
    def __init__(self, words) -> None:
        self.words = words
    
    def __str__(self) -> str:
        return f'{self.words}'

class Block:
    def __init__(self, lines: list['Line']) -> None:
        self.block = self.into_block(lines)

    def into_block(self, lines) -> 'Line':
        code = []
        code.append(Line('{', 0))
        for line in lines:
            # print(line)
            line.indent()
            code.append(line)
        code.append(Line('}', 0))
        return code
    
class Defun:
    def __init__(self, visibility: 'Words', fn_name: str, params: list[tuple[str, 'Words']], return_type: 'Words', procedure: list['Line']) -> None: # type: ignore
        self.visibility = visibility
        self.fn_name = fn_name
        self.params = params
        self.return_type = return_type
        self.procedure = Block(procedure)

    def defun(self):
        code = []
        def_line = ''
        if self.visibility != None:
            def_line += '{} '.format(self.visibility)
        
        def_line += f'fn {self.fn_name}({self.param()}) -> {self.return_type} {{'
        code.append(Line(def_line, 0))
        
        for line in self.procedure.block[1:-1]:
            code.append(line)

        code.append(Line('}', 0))
        return code

    def param(self) -> str:
        return display_list(['{}: {}'.format(var, _type) for (var, _type) in self.params])

class Match:
    def __init__(self, matchee: 'Words', match_patterns: list[tuple['Words', Block | Words]]) -> None:
        self.matchee = matchee
        self.match_patterns = match_patterns

    def match(self) -> list['Line']:
        code = []
        code.append(Line(f'match {self.matchee} {{', 0))
        for (pat, expr) in self.match_patterns:
            match type(expr).__name__:
                case 'Block':
                    code.append(Line(f'{pat} => {{', 1))
                    for line in expr.block[1:-1]:
                        line.indent()
                        code.append(line)
                    code.append(Line('},', 1))
                case 'Words':
                    code.append(Line(f'{pat} => {expr},', 1))
                case _:
                    print('invalid type', file=sys.stderr)
        code.append(Line('}', 0))
        return code

class FunCall:
    def __init__(self, fn_name: str, params: list[str]) -> None:
        self.fn_name = fn_name
        self.params = params
    
    def call(self) -> 'Words':
        return Words(f'{self.fn_name}({display_list(self.params)})')

class Const:
    def __init__(self, visibility: 'Words', name: str, expr:'Words', type:'Words') -> None:
        self.visibility = visibility
        self.name = name
        self.expr = expr
        self.type = type
    
    def const(self):
        def_line = ''
        if self.visibility != None:
            def_line += '{} '.format(self.visibility)
        
        def_line += f'const {self.name}: {self.type} = {self.expr};'
        return Line(def_line, 0)


def display_list(array) -> str:
    match len(array):
        case 0:
            return ''
        case 1:
            return f'{array.pop(0)}'
        case _:
            return f'{array.pop(0)}, ' + display_list(array) 

def tokenize(s: str) -> list:
    tokens = s.replace('(', ' ( ').replace(')', ' ) ').replace(',', ' , ').replace('#[', ' #[ ').replace(']', ' ] ').split()
    return tokens

def read_tokens(tokens: list[str]) -> list:
    seq = []
    token = tokens.pop(0)
    if token == '(':
        while tokens[0] != ')':
            seq.append(read_tokens(tokens))
        
        tokens.pop(0)
        return seq
    if token == '#[':
        while tokens[0] != ']':
            tokens.pop(0)
        tokens.pop(0)
        return  None
    else:
        return token

def read(tokens: list[str]) -> list:
    seq = []
    while len(tokens) != 0 and (tokens[0] == '(' or tokens[0].startswith('#[')):
        if tokens[0] == '(':
            seq.append(read_tokens(tokens))
        elif tokens[0].startswith('#['):
            read_tokens(tokens)
        else:
            print('Syntax Error!', file=sys.stderr)
            exit(1)
    return seq

def read_list(path:str) -> list:
    with open(path) as file:
        return read(tokenize(file.read()))
