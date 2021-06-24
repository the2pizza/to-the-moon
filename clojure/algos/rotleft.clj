
; The task https://www.hackerrank.com/challenges/ctci-array-left-rotation/problem

; A left rotation operation on an array shifts each of the array's elements unit to the left.
; For example, if left rotations are performed on array , then the array would become.
; Note that the lowest index item moves to the highest index in a rotation. This is called a circular array.
; Given an array of integers and a number, perform left rotations on the array.
; Return the updated array to be printed as a single line of space-separated integers.

;Sample Input
; 5 4
; 1 2 3 4 5
;
; Sample Output
; 5 1 2 3 4


(defn rotLeft [a d]
  (let [l (count a)]
    (map #(nth a (mod (+ (mod d l) %) l)) (range l))))