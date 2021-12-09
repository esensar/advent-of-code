(require '(clojure [string :as str]))

(def current-dir (.getParent (java.io.File. *file*)))

(defn char-to-int [x] (Character/digit x 10))

(defn get-input []
  (->> (-> (slurp (java.io.File. current-dir "input.txt"))
           (str/split #"\n"))
       (map #(map char-to-int %))))

(defn heightmap-get [heightmap x y]
  (cond
    (< x 0) 10
    (< y 0) 10
    (>= y (count heightmap)) 10
    (>= x (count (nth heightmap 0))) 10
    :else (nth (nth heightmap y) x)))

(defn adjacent [heightmap x y]
  (doall (for [[ax ay] [[-1 0] [1 0] [0 1] [0 -1]]] [(heightmap-get heightmap (+ x ax) (+ y ay)) (+ x ax) (+ y ay)])))

(defn lowpoint? [heightmap x y]
  (let [height (heightmap-get heightmap x y)]
    (if (reduce #(and %1 %2) (map #(< height %) (map first (adjacent heightmap x y)))) height nil)))

(defn generate-basin
  ([heightmap x y] (generate-basin heightmap x y #{}))
  ([heightmap x y visited]
   (let [height (heightmap-get heightmap x y)
         new-visited (conj visited [height x y])]
     (cond
       (= 9 (heightmap-get heightmap x y)) []
       (< x 0) []
       (< y 0) []
       (>= y (count heightmap)) []
       (>= x (count (nth heightmap 0))) []
       (contains? visited [height x y]) []
       :else (set (doall (apply concat [[height x y]] (flatten (map #(generate-basin heightmap (nth % 1) (nth % 2) new-visited) (filter #(<= height (first %)) (adjacent heightmap x y)))))))))))

(defn low-points [heightmap]
  (let [height (count heightmap)
        width (count (nth heightmap 0))]
    (->> (for [x (range 0 width) y (range 0 height)] [(lowpoint? heightmap x y) x y])
         (filter #(not (nil? (first %)))))))

(defn part-1 [input]
  (->> input
       low-points
       (map first)
       (map inc)
       (reduce +)))

(defn part-2 [input]
  (->> input
       low-points
       (map #(generate-basin input (nth % 1) (nth % 2)))
       set
       (map count)
       sort
       (take-last 3)
       (reduce *)))

(println "Problem 1 solution: " (part-1 (get-input)))
(println "Problem 2 solution: " (part-2 (get-input)))
