from time import time


def if_even_div(a,b):
    c = None
    print(c)
    if (a/2) == 0:
        c = b / (b + 3)
    else:
        c = b * 0.25
    print(c)

a = 0
start = time()
while a<100000:
    if_even_div(a, a * 3.1415)
    a = a + 1

print("loop took")
print(time()-start)
