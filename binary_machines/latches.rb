require_relative "gates"

class DLatch
  def self.input(data:, clock:)
    and1 = ANDGate.input(data, clock)
    and2 = ANDGate.input(Invertor.input(data), clock)
    NORGate.input(and1, "")
    NORGate.input(and2, "")
  end
end

class EightBitLatch
end

