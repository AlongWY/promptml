# Generated content DO NOT EDIT
def parse(template):
    """
    Parse promptml template to Fragments

    Args:
        template (:obj:`str`):
            The size of the final vocabulary, including all tokens and alphabet.

    Returns:
        A :obj:`List` of :class:`~prompt.PromptFragment`: The prompt fragments
    """
    pass

class PromptFragment:
    """
    A :obj:`PromptFragment` store template fragments(including string and options).

    Args:
        string (:obj:`str`,`optional`):
            The string or mask name will be rendered.
        option (:obj:`List[str]`):
            The options will be applied to the fragment.
    """

    def __init__(self, string=None, option=None):
        pass
    @property
    def options(self):
        """
        the options os the fragment
        """
        pass
    @staticmethod
    def parse(template):
        """
        Parse promptml template to Fragments

        Args:
            template (:obj:`str`):
                The size of the final vocabulary, including all tokens and alphabet.

        Returns:
            A :obj:`List` of :class:`~prompt.PromptFragment`: The prompt fragments
        """
        pass
    @property
    def string(self):
        """
        the content of the fragment
        """
        pass

class PromptTemplate:
    """
    A :obj:`PromptTemplate` works as a pipeline. It processes some raw text :obj:`Dict[str, str]`
    as input and outputs an :obj:`Dict[str, int]` for language models.

    Args:
        template (:obj:`str`):
            The promptml template to render the raw texts.

    """

    def __init__(self, template, tokenizer):
        pass
    @property
    def fragments(self):
        """
        the fragments of the processed template
        """
        pass
    @staticmethod
    def parse(template):
        """
        Parse promptml template to Fragments

        Args:
            template (:obj:`str`):
                The size of the final vocabulary, including all tokens and alphabet.

        Returns:
            A :obj:`List` of :class:`~prompt.PromptFragment`: The prompt fragments
        """
        pass
    @property
    def tokenizer(self):
        """
        the tokenizer for processing
        """
        pass
