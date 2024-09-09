class Grammar:
    def __init__(self, verb: str, instraction_determinants: tuple, instractions: dict[tuple, list[str]]) -> None:
        self.verb = verb
        self.instraction_determinants = instraction_determinants
        self.instractions = instractions

    # ("verb" ("instraction-determining-preps(IDP)") ("IDP's objects" "instraction" "operand determinimg prep"*)*)
    def read(tokens: list[str]) -> 'Grammar':
        verb = tokens[0]
        instraction_determinants = tuple(tokens[1])
        instractions = {}
        for rule in tokens[1:]:
            instractions.update({tuple(rule[:len(instraction_determinants)]): rule[len(instraction_determinants):]})
        return Grammar(verb, instraction_determinants, instractions)

    def generate(self) -> str:
        fn_name = 'analyze_{}'.format(self.verb)
        
