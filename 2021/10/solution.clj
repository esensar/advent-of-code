(require '(clojure [string :as str]))

(def current-dir (.getParent (java.io.File. *file*)))

(defn get-input []
  (-> (slurp (java.io.File. current-dir "input.txt"))
           (str/split #"\n")))

(def chunk-characters
  {\( \)
   \[ \]
   \{ \}
   \< \> })

(defn parse-line
  ([line] (parse-line line '()))
  ([[character & line] chunk-stack]
   (cond
     (nil? character) [nil chunk-stack]
     (contains? chunk-characters character) (recur line (conj chunk-stack character))
     (= character (chunk-characters (peek chunk-stack))) (recur line (pop chunk-stack))
     :else [character chunk-stack])))

(defn corrupted? [line]
  (first (parse-line line)))

(def score-map
  {\) 3
   \] 57
   \} 1197
\> 25137 })

(def completion-score-map
  {\) 1
   \] 2
   \} 3
\> 4 })

(defn line-score [chunk-stack]
  (reduce #(+ (* 5 %1) (completion-score-map (chunk-characters %2))) 0 chunk-stack))

(defn middle-element [elements]
  (nth elements (int (/ (count elements) 2))))

(defn part-1 [input]
  (->> input
       (map corrupted?)
       (filter (complement nil?))
       (map score-map)
       (reduce +)))

(defn part-2 [input]
  (->> input
       (filter (complement corrupted?))
       (map parse-line)
       (map second)
       (map line-score)
       sort
       middle-element))

(println "Problem 1 solution: " (part-1 (get-input)))
(println "Problem 2 solution: " (part-2 (get-input)))
