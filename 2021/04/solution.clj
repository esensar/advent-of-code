(require '(clojure [string :as str]))

(def current-dir (.getParent (java.io.File. *file*)))

(defn to-int [value] (Integer/parseInt value))

(defn get-input []
  (let [lines (filter #(not (str/blank? %)) (str/split (slurp (java.io.File. current-dir "input.txt")) #"\n"))]
    [(map to-int (str/split (first lines) #",")) (partition 5 (map #(map to-int (str/split (str/trim %) #"\s+")) (rest lines)))]))

(defn transpose-board [board]
  (apply mapv vector board))

(defn winning-line? [numbers line]
  (if (every? (fn [x] (some #{x} numbers)) line) line nil))

(defn get-winning-line [numbers board]
  (or
    (some (partial winning-line? numbers) board)
    (some (partial winning-line? numbers) (transpose-board board))))

(defn sum-without-marked [numbers board]
  (let [board-numbers (flatten board)
        marked (filter (fn [x] (some #{x} board-numbers)) numbers)]
    (- (apply + board-numbers) (apply + marked))))

(defn winnder-index [numbers boards]
   (first (keep-indexed #(if (get-winning-line numbers %2) %1 nil) boards)))

(defn get-winner-index
  ([[numbers boards]] (get-winner-index [numbers boards] 5))
  ([[numbers boards] taken]
   (let [winning-board-index (winnder-index (take taken numbers) boards)]
     (if (nil? winning-board-index)
       (recur [numbers boards] (inc taken))
       [(dec taken) winning-board-index]))))

(defn get-last-winner
  ([[numbers boards]] (get-last-winner [numbers boards] 5))
  ([[numbers boards] taken]
   (if (= 1 (count boards))
     [(dec taken) (first boards)]
     (get-last-winner [numbers (remove #(if (get-winning-line (take taken numbers) %) % nil) boards)] (inc taken)))))

(defn part-1 []
  (let [[numbers boards] (get-input)
        [last-number-index board-index] (get-winner-index [numbers boards])]
    (* (nth numbers last-number-index) (sum-without-marked (take (inc last-number-index) numbers) (nth boards board-index)))))

(defn part-2 []
  (let [[numbers boards] (get-input)
        [last-number-index board] (get-last-winner [numbers boards])]
    (* (nth numbers last-number-index) (sum-without-marked (take (inc last-number-index) numbers) board))))

(println "Problem 1 solution: " (part-1))
(println "Problem 2 solution: " (part-2))
