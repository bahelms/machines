# Deterministic Finite Automaton

require_relative "finite_automaton_rule"

class DFARulebook < Struct.new(:rules)
  def next_state(state, char)
    rule_for(state, char).follow
  end

  def rule_for(state, char)
    rules.detect { |rule| rule.applies_to?(state, char) }
  end
end

class DFA < Struct.new(:current_state, :accept_states, :rulebook)
  def accepting?
    accept_states.include?(current_state)
  end

  def read_char(char)
    self.current_state = rulebook.next_state(current_state, char)
  end

  def read_string(string)
    string.chars.each { |char| read_char(char) }
  end
end

class DFADesign < Struct.new(:start_state, :accept_states, :rulebook)
  def to_dfa
    DFA.new(start_state, accept_states, rulebook)
  end

  def accepts?(string)
    to_dfa.tap { |dfa| dfa.read_string(string) }.accepting?
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

puts; puts "Read character"
dfa = DFA.new(1, [3], rulebook); puts dfa.accepting?
dfa.read_char('b'); puts dfa.accepting?
3.times { dfa.read_char('a') }; puts dfa.accepting?
dfa.read_char('b'); puts dfa.accepting?

puts; puts "Read string"
dfa = DFA.new(1, [3], rulebook); puts dfa.accepting?
dfa.read_string("baab"); puts dfa.accepting?

# Creates a new DFA each time
puts; puts "DFA Auto"
dfa_design = DFADesign.new(1, [3], rulebook)
puts dfa_design.accepts?('a')
puts dfa_design.accepts?("baa")
puts dfa_design.accepts?("baaab")
