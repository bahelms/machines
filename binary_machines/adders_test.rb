require "minitest/autorun"
require_relative "adders"

class AddersTest < Minitest::Test
  def test_full_adder
    results = FullAdder.add(bits: ["1", "1"], carry_over: "1")
    assert_equal({ sum: "1", carry_over: "1" }, results)
  end

  def test_half_adder
    results = HalfAdder.add(bit_one: "1", bit_two: "1")
    assert_equal({ sum: "0", carry_over: "1"}, results)
  end
end

