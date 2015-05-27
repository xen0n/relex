use super::Wstr;
use super::LexerResult;
use super::handler::PostProcessor;
use super::rule::LexRule;
use super::rule::LexRuleMatch;


pub struct Lexer<'a, T, S> {
    source: &'a String,
    rules: Vec<LexRule<T, S>>,
    postprocessor: Option<Box<PostProcessor<T>>>,
    state: S,
}


impl<'a, T, S> Lexer<'a, T, S> {
    pub fn new(source: &'a String, initial_state: S) -> Lexer<'a, T, S> {
        Lexer {
            source: source,
            rules: Vec::new(),
            postprocessor: None,
            state: initial_state,
        }
    }

    pub fn add_rule(&mut self, rule: LexRule<T, S>) {
        self.rules.push(rule);
    }

    pub fn set_postprocessor(&mut self, f: Box<PostProcessor<T>>) {
        self.postprocessor = Some(f);
    }

    fn unpack_source_into_wstr(&self) -> Wstr {
        self.source.chars().collect()
    }

    pub fn lex(&mut self) -> LexerResult<T> {
        let last_pos = self.source.len() - 1usize;
        debug!("lex: input={}, last_pos={}", self.source, last_pos);

        let input_unpacked = self.unpack_source_into_wstr();
        let mut pos = 0usize;
        let mut result = Vec::new();
        while pos < last_pos {
            let next_state = self.advance(pos, &input_unpacked);

            if let Some(tokens) = next_state.result {
                // stash result
                result.extend(tokens.into_iter());

                // update lex position
                pos += next_state.advance;
            } else {
                return None;
            }
        }

        if let Some(ref postprocessor_fn) = self.postprocessor {
            postprocessor_fn(Some(result))
        } else {
            Some(result)
        }
    }

    fn advance(&mut self, pos: usize, input_unpacked: &Wstr) -> LexRuleMatch<T> {
        // slice the input
        let input_ahead = &self.source[pos ..];
        let eof = input_ahead.len() == 0;

        for rule in self.rules.iter() {
            let rule_match = rule.execute(input_ahead, pos, eof, input_unpacked, &mut self.state);
            if let Some(_) = rule_match.result {
                return rule_match;
            }
        }

        LexRuleMatch {
            result: None,
            advance: 0,
        }
    }
}
