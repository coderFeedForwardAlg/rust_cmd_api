
import ray
ray.init()


@ray.remote
def expo(x, y):
    return x ** 2


futures = [expo.remote(i, i) for i in range(4)]



print(ray.get(futures))

# print("hello python rust")

