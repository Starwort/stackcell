import sys

def convert_character(bf_char: str) -> str:
    if bf_char == '>':
        return 'X{X}'
    elif bf_char == '<':
        return '{X}X'
    elif bf_char == '+':
        return '#01+'
    elif bf_char == '-':
        return '#01x-'
    elif bf_char == '.':
        return ':;'
    elif bf_char == ',':
        return '`@'
    elif bf_char == '[':
        return ':['
    elif bf_char == ']':
        return ':]'
    else:
        return ''

if __name__ == '__main__':
    prog_file = sys.argv[1]
    with open(prog_file, 'r') as f:
        prog = f.read()
    print(''.join(convert_character(bf_char) for bf_char in prog))
