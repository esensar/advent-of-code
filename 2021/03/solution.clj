(require '(clojure [string :as str]))

(def current-dir (.getParent (java.io.File. *file*)))

(defn get-input []
  (str/split (slurp (java.io.File. current-dir "input.txt")) #"\n"))

(defn parse-bin [bin-string] (map #(Character/digit % 2) bin-string))

; Increase by 1 for 1, decrease by 1 for 0
; Positive means 1 was most common, negative means 0 was most common
(defn count-binary-bit-repeat [acc new]
  ; doall to prevent StackOverflowError
  (doall (->> new
       (map #(if (= % 0) -1 1))
       (map vector acc)
       (map #(apply + %)))))

(defn flip-bit [bit] (if (= bit 0) 1 0))

(defn get-gamma-rate [input]
  (->> input
       (map parse-bin)
       (reduce count-binary-bit-repeat)
       (map #(if (> % 0) 0 1))))

(defn get-epsilon-rate [gamma-rate]
  (map flip-bit gamma-rate))

(defn most-common-bit [input bit]
  (if (< (->> input
              (map #(nth % bit))
              (map #(if (= % 0) -1 1))
              (reduce +)) 0) 0 1))

(defn get-oxygen-generator-rating
  ([input] (get-oxygen-generator-rating input 0))
  ([input index]
   (cond
     (<= (count input) 1) (first input)
     :else (let [most-common (most-common-bit input index)]
             (recur (filter #(= (nth % index) most-common) input) (inc index))))))

(defn get-co2-scrubber-rating
  ([input] (get-co2-scrubber-rating input 0))
  ([input index]
   (cond
     (<= (count input) 1) (first input)
     :else (let [most-common (flip-bit (most-common-bit input index))]
             (recur (filter #(= (nth % index) most-common) input) (inc index))))))

(defn bin-to-dec [bin]
  (->> bin
       reverse
       (map-indexed #(* %2 (reduce * (repeat %1 2))))
       (reduce +)))

(defn part-1 []
  (let [input (get-input)
        gamma-rate (get-gamma-rate input)
        epsilon-rate (get-epsilon-rate gamma-rate)]
    (* (bin-to-dec gamma-rate) (bin-to-dec epsilon-rate))))

(defn part-2 []
  (let [input (map parse-bin (get-input))
        generator-rating (get-oxygen-generator-rating input)
        scrubber-rating (get-co2-scrubber-rating input)]
    (* (bin-to-dec generator-rating) (bin-to-dec scrubber-rating))))

(println "Problem 1 solution: " (part-1))
(println "Problem 2 solution: " (part-2))
