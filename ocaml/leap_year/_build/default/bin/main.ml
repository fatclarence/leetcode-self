(* run dune exec bin/main.exe to build project *)
let leap_year year = 
  (* if year is divisible by 4 and is not divisible by 100, or if a year is divisible by 400 *)
  (* (year mod 4 = 0 && year mod 100 <> 0) || (year mod 400 = 0 && year mod 100 = 0) *)
  (* The above simplifies to (year mod 4 = 0 && year mod 100 <> 0 ) || year mod 400 = 0 *)
  (* (year mod 4 = 0 || year mod 400 = 0) && (year mod 100 <> 0 || year mod 400 = 0) *)
  (* If a year is divisble by 400, it will definitely be divisble by 4 *)
  (* so, simplify further: year mod 4 = 0 && (year mod 100 <> 0 || year mod 400 = 0) *)
  year mod 4 = 0 && (year mod 100 <> 0 || year mod 400 = 0)

let () =
  try
    print_string "Enter a year: ";
    let year = int_of_string (read_line ()) in
    if leap_year year then
      print_endline "Leap year"
    else
      print_endline "Not a leap year"
  with
  | Failure _ -> print_endline "Please enter a valid integer year"
  | End_of_file -> print_endline "End of input reached"
