from promptengine import PromptTemplate


def main():
    res = PromptTemplate("[cls][sent_0][sent_1|a,b,c,d][sent_0] is a [mask][sep]")
    print(res)
    print(res.fragments)
    for p in res.fragments:
        if p.option is not None:
            print(p)
        else:
            print(f"\"{p}\"")


if __name__ == '__main__':
    main()
