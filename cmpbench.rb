#!/usr/bin/env ruby

system("cargo test digits")

results = `cargo bench digits`

results[%r{test scalar_mul::digits::test::newnaf\s*...\s*bench:\s*(\d+) ns\/iter}]
new_cost = $1.to_f
results[%r{test scalar_mul::digits::test::oldnaf\s*...\s*bench:\s*(\d+) ns\/iter}]
old_cost = $1.to_f
frac = 1.0 - (new_cost / old_cost)

puts "#{(frac*100).to_i}%" + " savings"