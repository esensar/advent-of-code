(require '(clojure [string :as str]))

(defn get-input []
  (map #(Integer/parseInt %) (str/split (slurp "input.txt") #"\n")))

(defn to-pairs
  ([coll] (to-pairs coll []))
  ([[head & tail] pairs]
   (cond
     (nil? (first tail)) pairs
     :else (recur tail (conj pairs [head (first tail)])))))

(defn pair-to-measurement [[l r]]
  (cond
    (> l r) :decreased
    (< l r) :increased
    (= l r) :eq))

(defn to-triples
  ([coll] (to-triples coll []))
  ([[head & tail] triples]
   (cond
     (nil? (first tail)) triples
     (nil? (first (rest tail))) triples
     :else (recur tail (conj triples [head (first tail) (first (rest tail))])))))

(defn part-1 []
  (->> (get-input)
       to-pairs
       (map pair-to-measurement)
       (filter #(= :increased %))
       count))

(defn part-2 []
  (->> (get-input)
       to-triples
       (map #(reduce + %))
       to-pairs
       (map pair-to-measurement)
       (filter #(= :increased %))
       count))

(println "Problem 1 solution: " (part-1))
(println "Problem 2 solution: " (part-2))
