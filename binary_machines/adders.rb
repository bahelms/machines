require_relative "gates"

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

