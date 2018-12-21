filepath = 'vkeys.txt'
code = ''
num = ''
string = ''

with open(filepath) as fp:
    line = fp.readline()
    cnt = 0
    while line:
        if cnt == 0:
            code = line.strip()
            cnt += 1
        elif cnt == 1:
            num = line.strip()
            cnt += 1
        else:
            cnt = 0
            string = line.strip()
            print(num, code, string)

        line = fp.readline()
