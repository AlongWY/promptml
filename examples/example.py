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

    print("building template")
    template = PromptTemplate("[cls]A [mask] news : [text1|limit][text2][sep]", tokenizer)

    print("rendering template")
    start = time.time()
    res = template.render({"text1": "hello world fuck", "text2": "world"}, max_length=9)
    end = time.time()
    print("render time cost", end - start, 's')
    print(tokenizer.decode(res['input_ids']))

    start = time.time()
    imdb = load_dataset("imdb")
    imdb = res.render(imdb, max_length=128)
    print(res)
    print("render time cost", end - start, 's')
    print(imdb)


if __name__ == '__main__':
    main()

if __name__ == '__main__':
    main()
