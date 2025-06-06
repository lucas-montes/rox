import time

def is_even(n):
    return n - (n/2) *2 == 0

def collatz_length(n):
    length = 1
    while n != 1:
        if is_even(n):
            n = n // 2
        else:
            n = 3 * n + 1
        length += 1
    return length

def find_longest_collatz(limit):
    max_length = 0
    number = 1
    i = 1
    while i <= limit:
        length = collatz_length(i)
        if length > max_length:
            max_length = length
            number = i
        i += 1
    return number, max_length

limit = 10_000
start = time.time()
num, length = find_longest_collatz(limit)
end = time.time()

print(f"Longest Collatz sequence under {limit}: {num} (length: {length}) in {end - start:.2f} sec")

