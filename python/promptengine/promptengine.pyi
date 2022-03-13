from typing import List, Optional, Set


class PromptFragment:
    string: str
    option: Optional[Set[str]]


class PromptTemplate:
    fragments: List[PromptFragment]

    def __init__(self, template: str):
        pass


def parse_markup(template: str) -> List[PromptFragment]:
    pass
