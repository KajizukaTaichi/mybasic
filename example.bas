10 let number = 5
20 call factorial
30 exit
40 sub factorial
50 let count = 0
60 let result = 1
70 let count = count + 1
80 let result = result * count
90 if count < number then goto 70
100 return
