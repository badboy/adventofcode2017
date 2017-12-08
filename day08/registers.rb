#!/usr/bin/env ruby
# encoding: utf-8

class Register
  attr_reader :name
  attr_reader :value

  def initialize(name)
    @name = name
    @value = 0
  end

  def inc(n)
    @value += n
  end

  def dec(n)
    @value -= n
  end

  def ==(n)
    @value == n
  end
end

def method_missing(symbol, *args)
  if [:inc, :dec].include?(symbol)
    return [symbol, args.first]
  end

  if symbol.to_s =~ /^[a-z]+$/
    $registers[symbol] ||= Register.new(symbol)

    if args.empty?
      $last = $registers[symbol]
      return $last
    else
      op, val = args.first
      $registers[symbol].send(op, val)
      max = $registers.map{|k,v| v.value}.sort.last
      $all_time_high = max if max > $all_time_high

      return nil
    end
  end

  if symbol.to_s =~ /[<>=]+/
    return $last.value.send(symbol, args.first)
  end

  super
end

def run_cpu(input)
  $registers = {}
  $all_time_high = 0

  eval(input)

  [$all_time_high, Hash[*$registers.map{|k,v| [k, v.value]}.flatten]]
end

TEST = <<EOF
b inc 5 if a > 1
a inc 1 if b < 5
c dec -10 if a >= 1
c inc -20 if c == 10
EOF

if (file=ARGV.shift)
  content = IO.read(file)
else
  content = TEST
end
all_time_high, reg = run_cpu(content)
name, val = reg.sort_by{|k,v| v}.last
puts "#{name}: #{val}"
puts "Highest: #{all_time_high}"
