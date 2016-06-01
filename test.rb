require 'rubygems'
require 'bundler/setup'
Bundler.require
require 'fiddle'
library = Fiddle.dlopen('/Users/becker/trash/hash_color/target/release/libhash_color.dylib')


Fiddle::Function.new(library['initialize_rust_color'], [], Fiddle::TYPE_VOIDP).call
puts "".becker
puts "".length_equals?(1)

