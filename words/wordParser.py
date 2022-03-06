filename = "./allWords.txt"

twoFile = "./two.txt"
threeFile = "./three.txt"
fourFile = "./four.txt"
fiveFile = "./five.txt"
sixFile = "./six.txt"
sevenFile = "./seven.txt"
eightFile = "./eight.txt"
nineFile = "./nine.txt"





words = open(filename).readlines()

for word in words:
    word_len = len(word) -1 
    if word_len == 2:
        open(twoFile,'a').write(word)
    if word_len == 3:
        open(threeFile,'a').write(word)
    if word_len == 4:
        open(fourFile,'a').write(word)
    if word_len == 5:
        open(fiveFile,'a').write(word)
    if word_len == 6:
        open(sixFile,'a').write(word)
    if word_len == 7:
        open(sevenFile,'a').write(word)
    if word_len == 8:
        open(eightFile,'a').write(word)
    if word_len == 9:
        open(nineFile,'a').write(word)
