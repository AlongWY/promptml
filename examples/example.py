import promptengine


def main():
    res = promptengine.parse_markup("[cls][sent_0][sent_1][sent_0] is a [mask][sep]")
    print(res)
    for p in res:
        print(p)


if __name__ == '__main__':
    main()
