(* let f ~name1:arg1 ~name2:arg2 = arg1 + arg2;;
let result = f ~name1:5 ~name2:10 in
Printf.printf "result: %d\n" result;; *)

let f ~name1:arg1 ~name2:arg2 = arg1 + arg2;;
let result = 5 |> (fun x -> f ~name1:x ~name2:10) in
Printf.printf "result: %d\n" result;;
