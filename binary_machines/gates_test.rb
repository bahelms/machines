require "minitest/autorun"
require_relative "gates"

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

