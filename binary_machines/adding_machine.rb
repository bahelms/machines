class AddingMachine
  def initialize(binary1, binary2)
    @bytes = binary1.chars.zip(binary2.chars)
  end

  def calculate
    carry_over = "0"
    sum = ""

    @bytes.reverse.reduce(carry_over) do |co, bits|
      results = FullAdder.add(bits: bits, carry_over: co)
      sum << results[:sum]
      carry_over = results[:carry_over]
    end

    sum << carry_over if carry_over == "1"
    sum.reverse
  end
end

class FullAdder
  def self.add(bits:, carry_over:)
    first  = HalfAdder.add(bit_one: bits.first,  bit_two: bits.last)
    second = HalfAdder.add(bit_one: first[:sum], bit_two: carry_over)

    { sum:        second[:sum], 
      carry_over: ORGate.input(first[:carry_over], second[:carry_over]) }
  end
end

class HalfAdder
  def self.add(bit_one:, bit_two:)
    { sum:        XORGate.input(bit_one, bit_two), 
      carry_over: ANDGate.input(bit_one, bit_two) }
  end
end

class ORGate
  def self.input(a, b)
    a == "1" || b == "1" ? "1" : "0"
  end
end

class ANDGate
  def self.input(a, b)
    a == "1" && b == "1" ? "1" : "0"
  end
end

class NANDGate
  def self.input(a, b)
    a == "1" && b == "1" ? "0" : "1"
  end
end

class XORGate
  def self.input(a, b)
    ANDGate.input(ORGate.input(a, b), NANDGate.input(a, b))
  end
end

