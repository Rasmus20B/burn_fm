#!/usr/bin/env python3

import sys


def string_decrypt(byte_list: [int]):
    return "".join([chr(c ^ 0x5A) for c in byte_list])


if __name__ == "__main__":
    n = [int(x) for x in sys.argv[1].split(',')]
    s = string_decrypt(n)
    print(s)
