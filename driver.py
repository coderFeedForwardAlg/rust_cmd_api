import uvicorn


from random import random

num = int(random() * 10)
num = 8000 + num

if __name__ == "__main__":
    uvicorn.run("file:app", host="0.0.0.0", port=num)

print(num)
