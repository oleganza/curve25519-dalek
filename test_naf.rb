#!/usr/bin/env ruby

scalar = [247, 233, 122, 46, 141, 49, 9, 44, 107, 206, 123, 81, 239, 124, 111, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8];
naf = [-1, 0, -2, -1, 0, 0, 0, 0, -2, 1, -2, -2, -1, 0, 0, -2, -1, -2, -1, 0, 0, 0, -2, 1, -2, 0, 0, -2, -1, -2, 1, -2, -2, -1, 0, -2, 1, 0, -2, 0, -2, 1, 0, -2, 0, -2, 1, -2, -2, 1, -2, -2, 1, 0, 0, 0, 0, -2, 0, -2, -1, -2, 1, -2, 0, -2, -1, -2, -1, 0, -2, 1, -2, 0, 0, -2, 1, -2, 0, 0, 0, -2, -1, 0, 0, 0, -2, -1, -2, 1, 0, -2, -2, -1, -2, -1, 0, 0, 0, -2, -1, 0, 0, -2, 1, -2, 0, 0, 0, 0, -2, -1, 0, 0, 0, -2, -1, 0, -2, 1, -2, -2, -1, -2, 1, 0, 0, 0];
recovered = [159, 255, 97, 126, 230, 31, 126, 134, 121, 224, 129, 135, 97, 0, 120, 134, 103, 30, 126, 248, 231, 159, 129, 153, 127, 126, 248, 159, 127, 30, 102, 0];

r = 2
radix = 2**2

###################

result = 0
scalar.each_with_index do |digit,r|
	result += digit*(256**r)
end
puts "Original:"
puts result

###################

result = 0
naf.each_with_index do |digit,r|
	result += digit*(radix**r)
end
puts "Recovered:"
puts result