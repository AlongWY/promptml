from typing import List

from .promptengine import parse_markup, PromptFragment


class PromptTemplate:
    fragments: List[PromptFragment]

    def __init__(self, template: str):
        self.fragments = parse_markup(template)
