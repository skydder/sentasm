import sys

# I'm going to manage rust codes by 'Line'
class Line:
    def __init__(self, line, indent):
        self.line = line
        self.indent = indent
    
    def __str__(self) -> str:
        return '\t' * self.indent + self.line + '\n'
    
    def __repr__(self) -> str:
        return f'Stmt(line="{self.stmt}", indent={self.indent})'
    
    def indent(self):
        self.indent += 1
    
    def into_words(self):
        return Words(self.line)

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
            line.indent()
            code.append(line)
        code.append(Line('}', 0))
        return code
    
class Defun:
    def __init__(self, visibility: str, fn_name: str, params: list[tuple[str, str]], return_type: str, procedure: list[str]) -> None: # type: ignore
        self.visibility = visibility
        self.fn_name = fn_name
        self.params = params
        self.return_type = return_type
        self.procedure = procedure

    def defun(self) -> str:
        code = ''
        if self.visibility != '':
            code += '{} '.format(self.visibility)
        
        code += 'fn {}({}) -> {} {{\n'.format(self.fn_name, self.param(), self.return_type)
        
        for step in self.procedure:
            code += '\t{}\n'.format(step.replace('\n', '\n\t'))

        code += '}'
        return code

    def param(self) -> str:
        return display_list(['{}: {}'.format(var, _type) for (var, _type) in self.params])

    def __str__(self) -> str:
        code = ''
        if self.visibility != '':
            code += f'{self.visibility} '
        
        code += f'fn {self.fn_name}({self.param()}) -> {self.return_type} {{\n'

        


class Match:
    def __init__(self, matchee: str, match_patterns: list[tuple['Words', 'Block' | 'Line']]) -> None:
        self.matchee = matchee
        self.match_patterns = match_patterns

    def match(self) -> list['Block']:
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
    
    def call(self) -> str:
        return '{}({})'.format(self.fn_name, display_list(self.params))

class Const:
    def __init__(self, visibility: str, name: str, expr:str, type:str) -> None:
        self.visibility = visibility
        self.name = name
        self.expr = expr
        self.type = type;
    
    def const(self) -> str:
        return '{} const {}: {} = {};'.format(self.visibility, self.name, self.type, self.expr)

class Block:
    def __init__(self, procedure: list[str]) -> None:
        self.proc = procedure

    def block(self) -> str:
        code = '{\n'
        for step in self.proc:
            code += '\t{}\n'.format(step.replace('\n', '\t\n'))
        code += '}\n'
        return code

# recursive function might be better idea
def display_list(lists: list[str]) -> str:
    length = len(lists)
    display = ''
    if length != 0:
        for item in lists[:length - 1]:
            display += '{}, '.format(item)

        display += '{}'.format(lists[-1])
    
    return display 

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
