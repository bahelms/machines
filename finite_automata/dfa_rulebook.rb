class FARule < Struct.new(:state, :char, :next_state)
  def applies_to?(state, char)
    self.state == state && self.char == char
  end

  def follow
    next_state
  end

  def inspect
    "#<FARule #{state.inspect} --#{char}--> #{next_state.inspect}>"
  end
end

class DFARulebook < Struct.new(:rules)
  def next_state(state, char)
    rule_for(state, char).follow
  end

  def rule_for(state, char)
    rules.detect { |rule| rule.applies_to?(state, char) }
  end
end

# Example usage
rulebook = DFARulebook.new([
  FARule.new(1, 'a', 2), FARule.new(1, 'b', 1),
  FARule.new(2, 'a', 2), FARule.new(2, 'b', 3),
  FARule.new(3, 'a', 3), FARule.new(3, 'b', 3),
])

puts rulebook.next_state(1 , 'a')
puts rulebook.next_state(1 , 'b')
puts rulebook.next_state(2 , 'b')
