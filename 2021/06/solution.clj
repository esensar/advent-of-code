(require '(clojure [string :as str]))

(def current-dir (.getParent (java.io.File. *file*)))

(defn get-input []
  (map #(Integer/parseInt %) (str/split (str/trim (slurp (java.io.File. current-dir "input.txt"))) #",")))

(defn run-lanternfish-day [state]
  (let [new-state (->> state
                       (filter pos?)
                       (map dec))
        producing-count (- (count state) (count new-state))]
    (concat new-state (repeat producing-count 8) (repeat producing-count 6))))

(defn run-lanternfish-day-into-map [state]
  (->> (-> state
           (assoc 7 (+ (get state 7 0) (get state 0 0)))
           (assoc 9 (get state 0 0)))
       (filter #(pos? (first %)))
       (map #(vector (dec (first %)) (second %)))
       (reduce #(assoc %1 (first %2) (second %2)) {})))

(defn count-after-days [state days]
  (if (<= (- days state) 0) 1
  (let [left-days (- days (inc state))]
    (+ (count-after-days 8 left-days) (count-after-days 6 left-days)))))

(defn part-1 [input]
  (->> input
       (iterate run-lanternfish-day)
       (take 81)
       last
       count))

(defn part-2 [input]
  (->> input
       frequencies
       (iterate run-lanternfish-day-into-map)
       (take 257)
       last
       (reduce #(+ %1 (second %2)) 0)))

(println "Problem 1 solution: " (part-1 (get-input)))
(println "Problem 2 solution: " (part-2 (get-input)))
