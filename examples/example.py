import promptengine


def main():
    res = promptengine.parse_markup("[cls][sent_0][sent_1|a,b,c,d][sent_0] is a [mask][sep]")
    print(res)
    for p in res:
        if p.option:
            print(p, p.option)
        else:
            print(p)


if __name__ == '__main__':
    main()
