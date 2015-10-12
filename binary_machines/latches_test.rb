require "minitest/autorun"
require_relative "gates"

class LatchesTest < Minitest::Test
  def test_input
    assert_equal "1", DLatch.input(data: "1", clock: "1")
    assert_equal "1", DLatch.input(data: "0", clock: "0")
    assert_equal "1", DLatch.input(data: "1", clock: "0")

    assert_equal "0", DLatch.input(data: "0", clock: "1")
    assert_equal "0", DLatch.input(data: "0", clock: "0")
    assert_equal "0", DLatch.input(data: "1", clock: "0")
  end
end
