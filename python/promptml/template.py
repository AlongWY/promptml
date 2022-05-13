from typing import List, Union
from datasets import Dataset, DatasetDict
from transformers import PreTrainedTokenizer
from .promptml import PromptFragment, PromptTemplate as RustPromptTemplate


class PythonPromptTemplate(RustPromptTemplate):
    fragments: List[PromptFragment]
    tokenizer: PreTrainedTokenizer

    # todo: write in rust
    def __init__(self, template: str, tokenizer: PreTrainedTokenizer):
        super(PythonPromptTemplate, self).__init__()
        self.pre_fragments = []
        self.post_fragments: List[Union[List[int], PromptFragment]] = []
        self.template_length = 0
        for fragment in self.fragments:
            current = fragment
            if current.options is None:
                current = self.tokenizer(
                    current.string, add_special_tokens=False, return_token_type_ids=False, return_attention_mask=False
                )['input_ids']
            elif current.string == 'bos':
                current = self.tokenizer.bos_token_id
            elif current.string == 'eos':
                current = self.tokenizer.eos_token_id
            elif current.string == 'unk':
                current = self.tokenizer.unk_token_id
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

        self.auto_limit = len(self.pre_fragments) == 1

    def render(self, data: Union[dict, Union[Dataset, DatasetDict]], max_length=128):
        if isinstance(data, Dataset) or isinstance(data, DatasetDict):
            return self.render_dataset(data, max_length=max_length)
        elif isinstance(data, dict):
            return self.render_dict(data, max_length=max_length)
        else:
            raise NotImplementedError

    def render_dict(self, example: dict, max_length=128):
        new_example = {}
        for pre_fragment in self.pre_fragments:
            # fixme
            key = pre_fragment.string
            fragment_max_length = max_length - self.template_length
            tokenizer = self.tokenizer
            if "key" not in new_example:
                new_example[key] = tokenizer(
                    example[key],
                    truncation=True,
                    max_length=fragment_max_length,
                    add_special_tokens=False,
                    return_token_type_ids=False,
                    return_attention_mask=False
                )['input_ids']

        example = new_example
        auto_limit = self.auto_limit
        post_fragments = self.post_fragments
        total_length = sum(
            [len(example[p.string]) if isinstance(p, PromptFragment) else len(p) for p in post_fragments]
        )

        length_pruning = total_length - max_length
        input_ids = []
        content_attention_mask = []

        for p in post_fragments:
            if isinstance(p, PromptFragment):
                limit_flag = ('limit' in p.options or auto_limit) and (total_length > max_length)
                if limit_flag:
                    input_ids.extend(example[p.string][:-length_pruning])
                    content_attention_mask.extend([1] * len(example[p.string][:-length_pruning]))
                else:
                    input_ids.extend(example[p.string])
                    content_attention_mask.extend([1] * len(example[p.string]))
            else:
                input_ids.extend(p)
                content_attention_mask.extend([0] * len(p))

        input_ids_len = len(input_ids)
        remain_len = max_length - input_ids_len
        input_ids = input_ids + [self.tokenizer.pad_token_id] * remain_len
        attention_mask = [1] * input_ids_len + [0] * remain_len
        content_attention_mask = content_attention_mask + [0] * remain_len
        assert len(input_ids) == max_length
        return {
            'input_ids': input_ids,
            'attention_mask': attention_mask,
            'content_attention_mask': content_attention_mask,
            'labels_mask_pos': input_ids.index(self.tokenizer.mask_token_id)
        }

    def render_dataset(
            self,
            dataset: Union[Dataset, DatasetDict],
            max_length=512,
    ):
        tokenizer = self.tokenizer
        pre_fragments = self.pre_fragments
        template_length = self.template_length

        def tokenize(examples):
            res = {}
            for pre_fragment in pre_fragments:
                key = pre_fragment.string
                fragment_max_length = max_length - template_length
                if key not in res:
                    res[key] = tokenizer(
                        examples[key],
                        truncation=True,
                        max_length=fragment_max_length,
                        add_special_tokens=False,
                        return_token_type_ids=False,
                        return_attention_mask=False
                    )['input_ids']
            return res

        dataset = dataset.map(
            lambda examples: tokenize(examples),
            desc="Promptml Tokenizing",
            batched=True
        )

        auto_limit = self.auto_limit
        post_fragments = self.post_fragments
        mask_token_id = self.tokenizer.mask_token_id
        pad_token_id = self.tokenizer.pad_token_id

        def build_example(example):
            total_length = sum(
                [len(example[p.string]) if isinstance(p, PromptFragment) else len(p) for p in post_fragments]
            )
            length_pruning = total_length - max_length

            input_ids = []
            content_attention_mask = []
            for p in post_fragments:
                if isinstance(p, PromptFragment):
                    limit_flag = ('limit' in p.options or auto_limit) and (total_length > max_length)
                    if limit_flag:
                        input_ids.extend(example[p.string][:-length_pruning])
                        content_attention_mask.extend([1] * len(example[p.string][:-length_pruning]))
                    else:
                        input_ids.extend(example[p.string])
                        content_attention_mask.extend([1] * len(example[p.string]))
                else:
                    input_ids.extend(p)
                    content_attention_mask.extend([0] * len(p))

            input_ids_len = len(input_ids)
            remain_len = max_length - input_ids_len
            input_ids = input_ids + [pad_token_id] * remain_len
            attention_mask = [1] * input_ids_len + [0] * remain_len
            content_attention_mask = content_attention_mask + [0] * remain_len
            assert len(input_ids) == max_length
            return {
                'input_ids': input_ids,
                'attention_mask': attention_mask,
                'content_attention_mask': content_attention_mask,
                'labels_mask_pos': input_ids.index(mask_token_id) if mask_token_id in input_ids else -1
            }

        dataset = dataset.map(
            function=build_example,
            desc="Promptml Building",
            batched=False,
        )

        return dataset
