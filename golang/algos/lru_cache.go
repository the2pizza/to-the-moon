
// https://leetcode.com/problems/lru-cache/

type LRUCache struct {
	l   *list.List
	m   map[int]*list.Element
	cap int
}

type KVPair struct {
	key   int
	value int
}

func Constructor(capacity int) LRUCache {
	return LRUCache{
		cap: capacity,
		l:   new(list.List),
		m:   make(map[int]*list.Element),
	}
}

func (this *LRUCache) Put(key int, value int) int {
	if node, ok := this.m[key]; ok {
		this.l.MoveToFront(node)
		node.Value.(*list.Element).Value = KVPair{key: key, value: value}
	}  else {
        if this.l.Len() == this.cap {
            idx := this.l.Back().Value.(*list.Element).Value.(KVPair).key
            delete(this.m, idx)
            this.l.Remove(this.l.Back())
        }
        
        node := &list.Element{
            Value: KVPair{
                key:   key,
                value: value,
            },
        }
        
        ptr := this.l.PushFront(node)
        this.m[key] = ptr
    }

	return 0
}

func (this *LRUCache) Get(key int) int {
    
    if node, ok := this.m[key]; ok {
        val := node.Value.(*list.Element).Value.(KVPair).value
        // move node to front
        this.l.MoveToFront(node)
        return val
    }
    return -1
}