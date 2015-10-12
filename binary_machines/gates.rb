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
    Invertor.input(ANDGate.input(a, b))
  end
end

class NORGate
  def self.input(a, b)
    Invertor.input(ORGate.input(a, b))
  end
end

class XORGate
  def self.input(a, b)
    ANDGate.input(ORGate.input(a, b), NANDGate.input(a, b))
  end
end

class Invertor
  def self.input(bit)
    case bit
    when "0" then "1"
    when "1" then "0"
    else "Invalid bit"
    end
  end
end

