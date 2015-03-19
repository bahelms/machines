require "minitest/autorun"
require_relative "adding_machine"

class AddingMachineTest < Minitest::Test
  def test_adding_machine
    result = AddingMachine.new("10011101", "01110011").calculate
    assert_equal "100010000", result
  end
end

class FullAdderTest < Minitest::Test
  def test_full_adder
    results = FullAdder.add(bits: ["1", "1"], carry_over: "1")
    assert_equal({ sum: "1", carry_over: "1" }, results)
  end
end

class HalfAdderTest < Minitest::Test
  def test_half_adder
    results = HalfAdder.add(bit_one: "1", bit_two: "1")
    assert_equal({ sum: "0", carry_over: "1"}, results)
  end
end

class ORGateTest < Minitest::Test
  def test_or_gate
    assert_equal "1", ORGate.input("0", "1")
    assert_equal "1", ORGate.input("1", "1")
    assert_equal "1", ORGate.input("1", "0")
    assert_equal "0", ORGate.input("0", "0")
  end
end

class ANDGateTest < Minitest::Test
  def test_and_gate
    assert_equal "0", ANDGate.input("0", "1")
    assert_equal "1", ANDGate.input("1", "1")
    assert_equal "0", ANDGate.input("1", "0")
    assert_equal "0", ANDGate.input("0", "0")
  end
end

class NANDGateTest < Minitest::Test
  def test_nand_gate
    assert_equal "1", NANDGate.input("0", "1")
    assert_equal "0", NANDGate.input("1", "1")
    assert_equal "1", NANDGate.input("1", "0")
    assert_equal "1", NANDGate.input("0", "0")
  end
end

class XORGateTest < Minitest::Test
  def test_nand_gate
    assert_equal "1", XORGate.input("0", "1")
    assert_equal "0", XORGate.input("1", "1")
    assert_equal "1", XORGate.input("1", "0")
    assert_equal "0", XORGate.input("0", "0")
  end
end

