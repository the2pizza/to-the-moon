
# HashMap implementation
# https://en.wikipedia.org/wiki/Hash_table

class Hashmap:
    def __init__(self):
        self.store = [[]]
        self.size = 0
        self.capacity = 1
        self.load_factor = 1
        self.counter = 0
        self.counter_bucket = 0

    def __index(self, key):
        return sum([ord(x) for x in key]) % self.capacity

    def __resize(self):

        store = self.store
        if self.size / self.capacity > self.load_factor:
            self.capacity *= 2
        else:
            self.capacity -= 1
        self.size = 0

        self.store = [[]] * self.capacity

        for bucket in store:
            for k,v in bucket:
                self.assoc(k,v)

    def __len__(self):
        return self.size

    def clear(self):
        self.store = [[]]
        self.size = 0
        self.capacity = 1

    def remove(self, key):

        index = self.__index(key)

        for i, (k, v) in enumerate(self.store[index]):
            if k == key:
                self.store[index].pop(i)

                self.size -= 1
                if (self.size / self.capacity) < self.load_factor:
                    self.__resize()
                return

    def assoc(self, key, value):

        index = self.__index(key)

        if not self.store[index]:
            self.store[index] = []

        for i, (k,v) in enumerate(self.store[index]):
            if k == key:
                self.store[index][i] = (k, value)
                return

        self.size += 1
        self.store[index].append((key,value))

        if (self.size / self.capacity) > self.load_factor:
            self.__resize()

    def get(self, key):
        index = self.__index(key)
        if self.store[index]:
            for k,v in self.store[index]:
                if k == key:
                    return v

    def __iter__(self):
        self.counter = 0
        self.counter_bucket = 0

        return self

    def __next__(self):

        if self.counter >= len(self.store)-1:
            raise StopIteration

        if self.counter_bucket < len(self.store[self.counter]):
            item = self.store[self.counter][self.counter_bucket]
            self.counter_bucket += 1
        else:
            self.counter_bucket = 0
            self.counter += 1

            if len(self.store[self.counter]) == 0:
                self.counter += 1
            item = self.store[self.counter][self.counter_bucket]
            self.counter_bucket += 1

        return item[0]

def test_hashmap():
    a = Hashmap()
    print(a.store)

    a.assoc("foo", "bar")
    assert a.capacity == 1
    a.assoc("foo2", "bar2")
    assert a.capacity == 2
    a.assoc("foo3", "bar3")
    a.assoc("foo4", "bar3")
    a.assoc("foo5", "bar3")
    a.assoc("foo6", "bar3")
    print(a.store)



    assert a.get("foo") == "bar"
    assert a.get("foo3") == "bar3"

    a.assoc("foo", "barnew")
    assert a.get("foo") == "barnew"


    for x in a:
        print(x)



    a.remove("foo")


    a.remove("foo3")
    a.remove("foo3")

    assert a.get("foo2") == "bar2"

    a.remove("foo2")

    a.assoc("f2", "b2")
    a.assoc("f3", "b3")

    a.clear()

    print(a.store)


if __name__ == "__main__":
    test_hashmap()


















