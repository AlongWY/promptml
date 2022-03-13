from typing import List, Optional, Set


class PromptFragment:
    string: str
    option: Optional[Set[str]]

    def __init__(self):
        pass


def parse_markup(template: str) -> List[PromptFragment]:
    pass
