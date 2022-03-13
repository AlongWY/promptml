from itertools import chain
from typing import List, Union
from datasets import Dataset, DatasetDict
from transformers import PreTrainedTokenizer
from promptml import parse_markup, PromptFragment


class PromptTemplate:
    fragments: List[PromptFragment]
    tokenizer: PreTrainedTokenizer

    # todo: write in rust
    def __init__(self, template: str, tokenizer: PreTrainedTokenizer):
        self.fragments = parse_markup(template)
        self.tokenizer = tokenizer
        self.pre_fragments = []
        self.post_fragments: List[Union[List[int], PromptFragment]] = []
        self.template_length = 0
        self.template_mask_bias = 0
        for fragment in self.fragments:
            current = fragment
            if current.option is None:
                current = self.tokenizer(
                    current.string, add_special_tokens=False, return_token_type_ids=False, return_attention_mask=False
                )['input_ids']
            elif current.string == 'cls':
                current = self.tokenizer.cls_token_id
            elif current.string == 'mask':
                current = self.tokenizer.mask_token_id
            elif current.string == 'sep':
                current = self.tokenizer.sep_token_id
            else:
                self.pre_fragments.append(current)

            if len(self.post_fragments) and isinstance(self.post_fragments[-1], list) and isinstance(current, int):
                self.post_fragments[-1].append(current)
                self.template_length += 1
            elif len(self.post_fragments) and isinstance(self.post_fragments[-1], list) and isinstance(current, list):
                self.post_fragments[-1].extend(current)
                self.template_length += len(current)
            elif isinstance(current, int):
                self.post_fragments.append([current])
                self.template_length += 1
            else:
                self.post_fragments.append(current)
                self.template_length += 1

    def render(
            self,
            dataset: Union[Dataset, DatasetDict],
            max_length=512,
    ):

        dataset = dataset

        for pre_fragment in self.pre_fragments:
            key = pre_fragment.string
            fragment_max_length = max_length - self.template_length
            dataset = dataset.map(
                lambda examples: {
                    key: self.tokenizer(
                        examples[key],
                        truncation=True,
                        max_length=fragment_max_length,
                        add_special_tokens=False,
                        return_token_type_ids=False,
                        return_attention_mask=False
                    )['input_ids']
                },
                batched=True
            )

        def build(example):
            input_ids = list(chain.from_iterable(
                [example[p.string] if isinstance(p, PromptFragment) else p for p in self.post_fragments]
            ))
            return {
                'input_ids': input_ids,
                'labels_mask_pos': input_ids.index(self.tokenizer.mask_token_id)
            }

        dataset = dataset.map(build, batched=False)

        return dataset


def __main__():
    from transformers import AutoTokenizer
    from datasets import load_dataset
    tokenizer = AutoTokenizer.from_pretrained("bert-base-uncased", use_fast=True)
    res = PromptTemplate("[cls]A [mask] news : [text][sep|+]", tokenizer)

    imdb = load_dataset("imdb")
    imdb = res.render(imdb, max_length=128)
    print(imdb)


if __name__ == '__main__':
    __main__()
