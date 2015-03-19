require "minitest/autorun"
require_relative "adding_machine"

class AddingMachineTest < Minitest::Test
  def test_adding_machine
    result = AddingMachine.new("10011101", "01110011").calculate
    assert_equal "100010000", result
  end
end

