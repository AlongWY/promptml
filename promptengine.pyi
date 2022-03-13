from typing import List


class PromptFragment:
    string: str
    is_control: bool

    def __init__(self):
        pass


def parse_markup(template: str) -> List[PromptFragment]:
    pass
