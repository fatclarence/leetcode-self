let () = 
  let result = 5 |> Utils.inc |> Utils.square in
  Printf.printf "result: %d\n" result;;

  (* Ran `dune build && dune exec bin/main.exe` *)