(require '(clojure [string :as str]))

(def current-dir (.getParent (java.io.File. *file*)))

(defn get-input []
  (map #(Integer/parseInt %) (str/split (slurp (java.io.File. current-dir "input.txt")) #"\n")))

(defn to-pairs [coll] (partition 2 1 coll))

(defn pair-to-measurement [[l r]]
  (cond
    (> l r) :decreased
    (< l r) :increased
    (= l r) :eq))

(defn to-triples [coll] (partition 3 1 coll))

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
