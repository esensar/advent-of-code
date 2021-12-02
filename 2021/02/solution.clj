(require '(clojure [string :as str]))

(def current-dir (.getParent (java.io.File. *file*)))

(defn get-input []
  (->> (str/split (slurp (java.io.File. current-dir "input.txt")) #"\n")
       (map #(str/split % #" "))
       (map #(vec [(first %) (Integer/parseInt (second %))]))))

(defn execute-commands
  ([commands] (execute-commands commands [0 0]))
  ([[current & commands] [pos depth]]
   (if (nil? current)
     [pos depth]
     (let [[direction value] current]
       (cond
         (= direction "forward") (recur commands [(+ pos value) depth])
         (= direction "down") (recur commands [pos (+ depth value)])
         (= direction "up") (recur commands [pos (- depth value)]))))))

(defn execute-commands-with-aim
  ([commands] (execute-commands-with-aim commands [0 0 0]))
  ([[current & commands] [pos depth aim]]
   (if (nil? current)
     [pos depth]
     (let [[direction value] current]
       (cond
         (= direction "forward") (recur commands [(+ pos value) (+ depth (* aim value)) aim])
         (= direction "down") (recur commands [pos depth (+ aim value)])
         (= direction "up") (recur commands [pos depth (- aim value)]))))))

(defn part-1 []
  (->> (get-input)
       execute-commands
       (reduce *)))

(defn part-2 []
  (->> (get-input)
       execute-commands-with-aim
       (take 2)
       (reduce *)))

(println "Problem 1 solution: " (part-1))
(println "Problem 2 solution: " (part-2))
