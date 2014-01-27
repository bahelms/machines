# Nondeterministic Finite Automaton

require "set"
require_relative "finite_automaton_rule"

class NFARulebook < Struct.new(:rules)
  def next_states(states, char)
    states.flat_map { |state| follow_rules_for(state, char) }.to_set
  end

  def follow_rules_for(state, char)
    rules_for(state, char).map(&:follow)
  end

  def rules_for(state, char)
    rules.select { |rule| rule.applies_to?(state, char) }
  end
end

class NFA < Struct.new(:current_states, :accept_states, :rulebook)
  def accepting?
    (current_states & accept_states).any?
  end
end

# Example usage
rulebook = NFARulebook.new([
  FARule.new(1, 'a', 1), FARule.new(1, 'b', 1), FARule.new(1, 'b', 2),
  FARule.new(2, 'a', 3), FARule.new(2, 'b', 3),
  FARule.new(3, 'a', 4), FARule.new(3, 'b', 4)
])

puts rulebook.next_states(Set[1], 'b').inspect
puts rulebook.next_states(Set[1, 2], 'a').inspect
puts rulebook.next_states(Set[1, 3], 'b').inspect

puts NFA.new(Set[1], [4], rulebook).accepting? 
puts NFA.new(Set[1, 2, 4], [4], rulebook).accepting? 
