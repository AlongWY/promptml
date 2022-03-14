from typing import List, Optional, Set


class PromptFragment:
    string: str
    option: Optional[Set[str]]

    def __init__(self, string: Optional[str] = None, option: Optional[Set[str]] = None):
        pass

    def __str__(self):
        pass

    def __hash__(self):
        pass

    def __getstate__(self):
        pass

    def __setstate__(self):
        pass


class PromptTemplate:
    fragments: List[PromptFragment]

    def __init__(self, template: str):
        pass


def parse(template: str) -> List[PromptFragment]:
    pass
