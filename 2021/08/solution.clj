(require '(clojure [string :as str]))
(require 'clojure.set)

(def current-dir (.getParent (java.io.File. *file*)))

(def display-map
  {
   (set "abcefg") 0
   (set "cf") 1
   (set "acdeg") 2
   (set "acdfg") 3
   (set "bcdf") 4
   (set "abdfg") 5
   (set "abdefg") 6
   (set "acf") 7
   (set "abcdefg") 8
   (set "abcdfg") 9 })

(defn get-input []
  (->> (-> (str/trim (slurp (java.io.File. current-dir "input.txt")))
           (str/split #"\n"))
       (map #(str/split % #"\|"))
       (map #(map str/trim %))
       (map (fn [line] (map #(str/split % #" ") line)))))

(defn is-1? [display] (= (count display) 2))
(defn is-4? [display] (= (count display) 4))
(defn is-7? [display] (= (count display) 3))
(defn is-8? [display] (= (count display) 7))

(defn find-conf [new-conf pred] (first (filter pred new-conf)))

(defn generate-modified-map [new-configuration]
  (let [new-conf (map set new-configuration)
        new-1 (find-conf new-conf is-1?)
        new-4 (find-conf new-conf is-4?)
        new-7 (find-conf new-conf is-7?)
        new-a (first (clojure.set/difference new-7 new-1))
        new-bd (clojure.set/difference new-4 new-1)
        new-253 (filter #(= 5 (count %)) new-conf)
        new-5 (first (filter #(clojure.set/subset? new-bd %) new-253))
        new-3 (first (filter #(clojure.set/subset? new-1 %) new-253))
        new-2 (first (filter #(and (not= new-5 %) (not= new-3 %)) new-253))
        new-f (first (clojure.set/difference new-1 new-2))
        new-c (first (filter #(not= new-f %) new-1))
        new-b (first (filter #(not= new-f %) (clojure.set/difference new-4 new-2)))
        new-d (first (filter #(not= new-b %) new-bd))
        new-g (first (clojure.set/difference new-5 #{new-a new-b new-d new-f}))
        new-e (first (clojure.set/difference new-2 #{new-a new-c new-d new-g}))]
    (into {}
          (map
            #(vector
               (set (map {\a new-a
                     \b new-b
                     \c new-c
                     \d new-d
                     \e new-e
                     \f new-f
                     \g new-g} (first %)))
               (second %))
            display-map))))

(defn pow [x n] (reduce * (repeat n x)))

(defn decode-number [configuration numbers]
  (->> numbers
       (map set)
       (map configuration)
       (map-indexed #(* %2 (pow 10 (dec (- (count numbers) %1)))))
       (reduce +)))

(defn part-1 [input]
  (->> input
       (map second)
       flatten
       (filter (some-fn is-1? is-4? is-7? is-8?))
       count))

(defn part-2 [input]
  (->> input
       (map #(vector (generate-modified-map (first %)) (second %)))
       (map #(decode-number (first %) (second %)))
       (reduce +)))

(println "Problem 1 solution: " (part-1 (get-input)))
(println "Problem 2 solution: " (part-2 (get-input)))
