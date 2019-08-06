import re

test_str = open(r'C:\Users\mm030792\Documents\MFD\scripts\extra\Mango\generator\grammar').read()
matches = re.findall('(\w+ -> [\w ]+)|((?<=\{\n)(.|\n)*?(?=\n\}))', test_str)

for match in matches:
    if match[0] != '':
        print(match[0])
    if match[1] != '':
        print('{\n' + match[1] + '\n}')
