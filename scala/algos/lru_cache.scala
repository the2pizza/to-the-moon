// https://leetcode.com/problems/lru-cache/

import scala.collection.mutable.LinkedHashMap

class LRUCache(_capacity: Int) {
    
    private var cache = new LinkedHashMap[Int, Int]
 
    def get(key: Int): Int = {
        
        try {   
            val value = cache(key)
            cache.remove(key)
            cache(key) = value
            cache(key)
        } catch {
            case x: NoSuchElementException => -1
        }
   }

    def put(key: Int, value: Int) = {
        
        cache.remove(key)
        cache(key) = value
   
        if (cache.size > _capacity ){
            cache = cache.drop(1)
        }
    }
    
    
}

/**
 * Your LRUCache object will be instantiated and called as such:
 * var obj = new LRUCache(capacity)
 * var param_1 = obj.get(key)
 * obj.put(key,value)
 */