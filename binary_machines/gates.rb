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

