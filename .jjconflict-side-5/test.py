from time import time


def fibo(n):
    if n<=1:
        return n
    return fibo(n-2) + fibo(n-1)

start = time()

for i in range(20):
    print(fibo(i))

print("loop took")
print(time()-start)
