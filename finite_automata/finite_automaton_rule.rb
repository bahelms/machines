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
