import sys


a=int(input())
for i in range(a):
    l=[1]
    b=int(input())
    t=0
    h=1
    k=t+h
    o=0
    p=0
    while k<b:
        t=h
        h=k
        k=t+h
        if k<b:
            l.append(k)


    for t in l:
        if t%2==0:
            p=p+t
    print(p)