import time
import promptml
from promptml import PromptTemplate, PromptFragment
from datasets import load_dataset
from transformers import AutoTokenizer, PreTrainedTokenizer


def main():
    print("parsing template")
    fragments = promptml.parse("[cls]A [mask] news : [text1|limit][text2][sep]")

    for fragment in fragments:
        fragment: PromptFragment
        print(f"\"{fragment.string}\"", fragment.option)

    print("building tokenizer")
    tokenizer: PreTrainedTokenizer = AutoTokenizer.from_pretrained("bert-base-uncased", use_fast=True)

    print("rendering simple template")
    template = PromptTemplate("[cls]A [mask] news : [text1|limit][text2][sep]", tokenizer)
    start = time.time()
    res = template.render({"text1": "hello world fuck", "text2": "world"}, max_length=9)
    end = time.time()
    print("render example time cost", end - start, 's')
    print(tokenizer.decode(res['input_ids']))

    print("rendering imdb template")
    template = PromptTemplate("[cls]A [mask] news : [text|limit][sep]", tokenizer)
    start = time.time()
    imdb = load_dataset("imdb")
    imdb = template.render(imdb, max_length=128)
    end = time.time()
    print("render imdb time cost", end - start, 's')


if __name__ == '__main__':
    main()
