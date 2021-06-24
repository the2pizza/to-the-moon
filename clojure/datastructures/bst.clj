
; Implementation of Binary Search Tree
; https://en.wikipedia.org/wiki/Binary_search_tree

(ns bst.core
    (:gen-class))

(defrecord TreeNode [key left right])

(defn insert [node key]

      (cond
        (nil? node)  (->TreeNode
                       key
                       nil
                       nil)

        (< key (:key node)) (update node :left insert key)
        (> key (:key node)) (update node :right insert key)
        :else node))


(defn tree->seq [node]
      (when (some? node)
            (lazy-cat
              (tree->seq (:left node))
              [(:key node)]
              (tree->seq (:right node)))))

(defn search [node key]
      (cond
        (nil? tree) nil
        (== (:key node) key) key
        (< key (:key node)) (recur (:left node) key)
        (> key (:key node)) (recur (:right node) key)))


(defn find-max [node]
      (if (some? (:right node))
        (recur (:right node))
        (:key node)))


(defn delete [node key]
      (cond
        (nil? node) nil
        (< key (:key node )) (update node :left delete key)
        (> key (:key node )) (update node :right delete key)
        (nil? (:right node)) (:left node)
        (nil? (:left node)) (:right node)

        :else (let [b (find-max (:left node))]
                   (->TreeNode
                     b
                     (delete (:left node) b)
                     (:right node)))))
