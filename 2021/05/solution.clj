(require '(clojure [string :as str]))

(def current-dir (.getParent (java.io.File. *file*)))

(defn parse-int [string] (Integer/parseInt string))

(defn parse-line [line]
  (map #(map parse-int (str/split % #",")) (str/split line #" -> ")))

(defn get-input []
  (map parse-line (str/split (slurp (java.io.File. current-dir "input.txt")) #"\n")))

(defn sign [x]
  (cond
    (pos? x) 1
    (neg? x) -1
    :else 0))

(defn direct-range [from to]
  (range from (+ to (sign (- to from))) (sign (- to from))))

(defn generate-line-points [[x1 y1] [x2 y2]]
  (cond
    (= x1 x2) (for [y (range (min y1 y2) (inc (max y1 y2)))] [x1 y])
    (= y1 y2) (for [x (range (min x1 x2) (inc (max x1 x2)))] [x y1])
    :else (for [[x y] (mapv #(vector %1 %2) (direct-range x1 x2) (direct-range y1 y2))] [x y])))

(defn determine-dangerous-points
  ([lines] (determine-dangerous-points lines []))
  ([[[[x1 y1] [x2 y2]] & lines] dangerous-points]
   (cond
     (nil? x1) dangerous-points
     :else (recur lines (concat dangerous-points (generate-line-points [x1 y1] [x2 y2]))))))

(defn horizontal? [[[x1 y1] [x2 y2]]] (or (= x1 x2) (= y1 y2)))

(defn part-1 [input]
  (let [dangerous-points (determine-dangerous-points (filter horizontal? input))]
    (->> (frequencies dangerous-points)
         (map second)
         (filter #(>= % 2))
         count)))

(defn part-2 [input]
  (let [dangerous-points (determine-dangerous-points input)]
    (->> (frequencies dangerous-points)
         (map second)
         (filter #(>= % 2))
         count)))

(println "Problem 1 solution: " (part-1 (get-input)))
(println "Problem 2 solution: " (part-2 (get-input)))
