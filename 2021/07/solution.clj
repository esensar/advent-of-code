(require '(clojure [string :as str]))

(def current-dir (.getParent (java.io.File. *file*)))

(defn get-input []
  (map #(Integer/parseInt %) (str/split (str/trim (slurp (java.io.File. current-dir "input.txt"))) #",")))

(defn calculate-fuel [positions target-position]
  (->> positions
       (map #(Math/abs (- % target-position)))
       (reduce +)))

(defn steps-to-fuel [steps] (int (* (/ steps 2) (inc steps))))

(defn calculate-fuel-2 [positions target-position]
  (->> positions
       (map #(Math/abs (- % target-position)))
       (map steps-to-fuel)
       (reduce +)))

(defn square [x] (* x x))
(defn mean [x] (/ (reduce + x) (count x)))

(defn standarddev [x]
      (Math/sqrt (/
                    (reduce + (map square (map - x (repeat (mean x)))))
                    (- (count x) 1 ))))

(defn get-cheapset-position [positions fuel-f]
  (let [avg (int (mean positions))
        stddev (int (standarddev positions))]
    (->> (range (- avg stddev) (+ avg stddev))
         (map #(fuel-f positions %))
         (apply min))))

(defn part-1 [input] (get-cheapset-position input calculate-fuel))

(defn part-2 [input] (get-cheapset-position input calculate-fuel-2))

(println "Problem 1 solution: " (part-1 (get-input)))
(println "Problem 2 solution: " (part-2 (get-input)))
