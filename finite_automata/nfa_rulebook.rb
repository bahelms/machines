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

  def follow_free_moves(states)
    more_states = next_states(states, nil)
    if more_states.subset?(states)
      states
    else
      follow_free_moves(states + more_states)
    end
  end
end

class NFA < Struct.new(:current_states, :accept_states, :rulebook)
  def accepting?
    (current_states & accept_states).any?
  end

  def read_char(char)
    self.current_states = rulebook.next_states(current_states, char)
  end

  def read_string(string)
    string.chars.each { |char| read_char(char) }
  end
  
  def current_states
    rulebook.follow_free_moves(super)
  end
end

class NFADesign < Struct.new(:start_state, :accept_states, :rulebook)
  def accepts?(string)
    to_nfa.tap { |nfa| nfa.read_string(string) }.accepting?
  end

  def to_nfa
    NFA.new(Set[start_state], accept_states, rulebook)
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

puts; puts "Read chars"
nfa = NFA.new(Set[1], [4], rulebook); puts nfa.accepting?
nfa.read_char('b'); puts nfa.accepting?
nfa.read_char('a'); puts nfa.accepting?
nfa.read_char('b'); puts nfa.accepting?

puts; puts "Read string"
nfa = NFA.new(Set[1], [4], rulebook)
nfa.read_string("bbbb"); puts nfa.accepting?

# Creates a new NFA each time
puts; puts "NFA Auto"
nfa_design = NFADesign.new(1, [4], rulebook)
puts nfa_design.accepts?("bab")
puts nfa_design.accepts?("bbbb")
puts nfa_design.accepts?("bbabb")

# Using free moves
puts; puts "Free Moves"
rulebook = NFARulebook.new([
  FARule.new(1, nil, 2), FARule.new(1, nil, 4),
  FARule.new(2, 'a', 3),
  FARule.new(3, 'a', 2),
  FARule.new(4, 'a', 5),
  FARule.new(5, 'a', 6),
  FARule.new(6, 'a', 4)
])
puts rulebook.next_states(Set[1], nil).inspect
puts rulebook.follow_free_moves(Set[1]).inspect
nfa_design = NFADesign.new(1, [2, 4], rulebook)
puts nfa_design.accepts?("aa")
puts nfa_design.accepts?("aaa")
puts nfa_design.accepts?("aaaaa")

