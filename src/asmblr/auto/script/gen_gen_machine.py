from .lib import Match, Defun, display_list

class Grammar:
    def __init__(self, verb: str, instraction_determinants: tuple, instractions: dict[tuple, list[str]]) -> None:
        self.verb = verb
        self.instraction_determinants = instraction_determinants
        self.instractions = instractions

    # ("verb" ("instraction-determining-preps(IDP)") (("IDP's objects") ("instraction" "operand determinimg prep"*))*)
    def read(tokens: list[str]) -> 'Grammar':
        verb = tokens[0]
        instraction_determinants = tuple(tokens[1])
        instractions = {}
        for rule in tokens[1:]:
            instractions.update({tuple(rule[:1]): rule[1:]})
        return Grammar(verb, instraction_determinants, instractions)

    def generate_proc(self) -> list[str]:
        proc = []
        proc.append('let _obj = sentence.object.map_or_else(|| None, |date| date.expect_object());'.format())
        
        for parameter in self.instraction_determinants:
            proc.append('let _{} = sentence.preposition_phrases.get_object(Preposition(\"{}\")).map_or_else(|| None, |date| date.expect_object());'.format(parameter, parameter))


    def generate_match(self) -> str:
        match_arm = '({})'.format(display_list(list(self.instraction_determinants)))
        pass # todo


    def generate(self) -> str:
        fn_name = 'analyze_{}'.format(self.verb)
        params = [('sentence', 'Sentence')]
        return_type = '(&str, Vec<DataSet>)'
