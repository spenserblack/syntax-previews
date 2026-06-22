#!/usr/bin/env ruby
# frozen_string_literal: true
# This is a comment
begin
  require "some-gem"
rescue LoadError => e
  warn "Couldn't require gem: #{e}"
end

[1, 2, 3].each do |i|
  puts "i = #{i}"
end
