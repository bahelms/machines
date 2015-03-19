require_relative "adders"

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

