import time
start = time.time()

x = 0
for _ in range(1000000000): x += 1
print(time.time() - start)
