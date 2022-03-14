from itertools import chain
from typing import List, Union
from datasets import Dataset, DatasetDict
from transformers import PreTrainedTokenizer
from .promptml import parse, PromptFragment


class PromptTemplate:
    fragments: List[PromptFragment]
    tokenizer: PreTrainedTokenizer

    # todo: write in rust
    def __init__(self, template: str, tokenizer: PreTrainedTokenizer):
        self.fragments = parse(template)
        self.tokenizer = tokenizer
        self.pre_fragments = []
        self.post_fragments: List[Union[List[int], PromptFragment]] = []
        self.template_length = 0
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

    def render(self, data: Union[dict, Union[Dataset, DatasetDict]], max_length=128):
        if isinstance(data, Dataset) or isinstance(data, DatasetDict):
            return self.render_dataset(data, max_length=max_length)
        elif isinstance(data, dict):
            return self.render_dict(data, max_length=max_length)
        else:
            raise NotImplementedError

    def render_dict(self, example: dict, max_length=128):
        for pre_fragment in self.pre_fragments:
            key = pre_fragment.string
            fragment_max_length = max_length - self.template_length
            tokenizer = self.tokenizer
            example[key] = tokenizer(
                example[key],
                truncation=True,
                max_length=fragment_max_length,
                add_special_tokens=False,
                return_token_type_ids=False,
                return_attention_mask=False
            )['input_ids']

        total_length = sum(
            [len(example[p.string]) if isinstance(p, PromptFragment) else len(p) for p in self.post_fragments]
        )
        if total_length > max_length:
            length_pruning = total_length - max_length
            input_ids = list(chain.from_iterable(
                [
                    (example[p.string][:-length_pruning] if 'limit' in p.option else example[p.string])
                    if isinstance(p, PromptFragment) else p for p in self.post_fragments
                ]
            ))
        else:
            input_ids = list(chain.from_iterable(
                [example[p.string] if isinstance(p, PromptFragment) else p for p in self.post_fragments]
            ))

        input_ids_len = len(input_ids)
        remain_len = max_length - input_ids_len
        input_ids = input_ids + [self.tokenizer.pad_token_id] * remain_len
        attention_mask = [1] * input_ids_len + [0] * remain_len
        return {
            'input_ids': input_ids,
            'attention_mask': attention_mask,
            'labels_mask_pos': input_ids.index(self.tokenizer.mask_token_id)
        }

    def render_dataset(
            self,
            dataset: Union[Dataset, DatasetDict],
            max_length=512,
    ):

        dataset = dataset
        for pre_fragment in self.pre_fragments:
            key = pre_fragment.string
            fragment_max_length = max_length - self.template_length
            tokenizer = self.tokenizer

            dataset = dataset.map(
                lambda examples: {
                    key: tokenizer(
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

        post_fragments = self.post_fragments
        mask_token_id = self.tokenizer.mask_token_id
        pad_token_id = self.tokenizer.pad_token_id

        def build(example):
            total_length = sum(
                [len(example[p.string]) if isinstance(p, PromptFragment) else len(p) for p in post_fragments]
            )
            if total_length > max_length:
                length_pruning = total_length - max_length
                input_ids = list(chain.from_iterable(
                    [
                        (example[p.string][:-length_pruning] if 'limit' in p.option else example[p.string])
                        if isinstance(p, PromptFragment) else p for p in self.post_fragments
                    ]
                ))
            else:
                input_ids = list(chain.from_iterable(
                    [example[p.string] if isinstance(p, PromptFragment) else p for p in self.post_fragments]
                ))
            input_ids_len = len(input_ids)
            remain_len = max_length - input_ids_len
            input_ids = input_ids + [pad_token_id] * remain_len
            attention_mask = [1] * input_ids_len + [0] * remain_len
            return {
                'input_ids': input_ids,
                'attention_mask': attention_mask,
                'labels_mask_pos': input_ids.index(mask_token_id)
            }

        dataset = dataset.map(build, batched=False)

        return dataset
