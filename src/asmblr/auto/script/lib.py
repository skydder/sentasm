import sys
class Defun:
    def __init__(self, visibility: str, fn_name: str, params: list[tuple[str, str]], return_type: str, procedure: list[str]) -> None: # type: ignore
        self.visibility = visibility
        self.fn_name = fn_name
        self.params = params
        self.return_type = return_type
        self.procedure = procedure

    def defun(self) -> str:
        code = '{} fn {}({}) -> {} {{\n'.format(self.visibility, self.fn_name, self.param(), self.return_type)
        for step in self.procedure:
            code += '\t{}\n'.format(step.replace('\n', '\n\t'))

        code += '}'
        return code

    def param(self) -> str:
        return display_list(['{}: {}'.format(var, _type) for (var, _type) in self.params])

class Match:
    def __init__(self, match_arm: str, match_patterns: list[tuple[str, str]]) -> None:
        self.match_arm = match_arm
        self.match_patterns = match_patterns

    def match(self) -> str:
        code = 'match {} {{\n'.format(self.match_arm)
        for (pat, expr) in self.match_patterns:
            code += '\t{} => {},\n'.format(pat, expr)

        code += '}'
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

# recursive function might be better idea
def display_list(lists: list[str]) -> str:
    length = len(lists)
    display = ''
    if length != 0:
        for item in lists[:length - 1]:
            display += '{}, '.format(item)

        display += '{}'.format(lists[-1])
    
    return display 