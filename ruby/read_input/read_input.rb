puts "Please enter your name:"
name = gets.chomp  # gets reads input, chomp removes the trailing newline

puts "Hello #{name}! You entered: #{name}"

# Let's also demonstrate reading a number
puts "\nEnter a number:"
number = gets.chomp.to_i  # to_i converts the input to an integer

puts "You entered the number: #{number}"
puts "Double your number is: #{number * 2}"

