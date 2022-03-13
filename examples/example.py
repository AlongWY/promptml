from promptml import PromptTemplate


def main():
    res = PromptTemplate("[cls]A [mask] news : [sent_0][sep|+]")
    print(res)
    print(res.fragments)
    for p in res.fragments:
        if p.option is not None:
            print(p)
        else:
            print(f"\"{p}\"")


if __name__ == '__main__':
    main()
