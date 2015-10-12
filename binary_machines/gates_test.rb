require "minitest/autorun"
require_relative "gates"

class GatesTest < Minitest::Test
  def test_or_gate
    assert_equal "1", ORGate.input("0", "1")
    assert_equal "1", ORGate.input("1", "1")
    assert_equal "1", ORGate.input("1", "0")
    assert_equal "0", ORGate.input("0", "0")
  end

  def test_and_gate
    assert_equal "0", ANDGate.input("0", "1")
    assert_equal "1", ANDGate.input("1", "1")
    assert_equal "0", ANDGate.input("1", "0")
    assert_equal "0", ANDGate.input("0", "0")
  end

  def test_nand_gate
    assert_equal "1", NANDGate.input("0", "1")
    assert_equal "0", NANDGate.input("1", "1")
    assert_equal "1", NANDGate.input("1", "0")
    assert_equal "1", NANDGate.input("0", "0")
  end

  def test_nor_gate
    assert_equal "0", NORGate.input("0", "1")
    assert_equal "0", NORGate.input("1", "1")
    assert_equal "0", NORGate.input("1", "0")
    assert_equal "1", NORGate.input("0", "0")
  end

  def test_xor_gate
    assert_equal "1", XORGate.input("0", "1")
    assert_equal "0", XORGate.input("1", "1")
    assert_equal "1", XORGate.input("1", "0")
    assert_equal "0", XORGate.input("0", "0")
  end

  def test_invertor
    assert_equal "0", Invertor.input("1")
    assert_equal "1", Invertor.input("0")
  end
end

