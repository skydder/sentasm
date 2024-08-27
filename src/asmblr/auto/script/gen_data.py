from .lib import Const, display_list
from .gen_gen import gen_codegen_verb

def read_data(path: str) -> list[str]:
    with open(path) as file:
        return [line.strip() for line in file.readlines()]

class GenData:
    def __init__(self, name: str, data: list[str]) -> None:
        self.name = name
        self.data = data
    
    def generate(self) -> str:
        data = '&[{}]'.format(display_list(['\"{}\"'.format(item) for item in self.data]))
        return Const("pub", self.name, data, "&[&'static str]").const()
    
if __name__ == '__main__':
    verb = read_data('tokens/VERB.dat')
    print(GenData('VERB', verb).generate())
    print(gen_codegen_verb(verb))