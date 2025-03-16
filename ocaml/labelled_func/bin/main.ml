(* let f ~name1:arg1 ~name2:arg2 = arg1 + arg2;;
let result = f ~name1:5 ~name2:10 in
Printf.printf "result: %d\n" result;; *)

(* let f ~name1:arg1 ~name2:arg2 = arg1 + arg2;;
let result = 5 |> (fun x -> f ~name1:x ~name2:10) in
Printf.printf "result: %d\n" result;; *)

(* open Z;; *)
let rec zfact_aux n acc = 
  if Z.equal n Z.zero then acc else zfact_aux(Z.pred n)(Z.mul acc n);;
let zfact_tr n = zfact_aux n Z.one;;

let result = zfact_tr (Z.of_int 50) in
  Printf.printf "result %s\n" (Z.to_string result);;
