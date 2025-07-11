use std::{cmp, collections::HashMap, error::Error, fmt, fs};

use crate::{
    array_nd::{Array3D, Array4D},
    expression::{Expression, ParseExpressionError},
    instance::Sets,
    reach::State,
};

/// Internal `Ruletable` representation.
///
/// # Usage
/// The `Ruletable` is typically constructed using [`Ruletable::from_file`] or
/// [`Ruletable::from_multiline_string`] and then passed to `reach`.
pub struct Ruletable {
    colors: HashMap<String, usize>,
    edges: HashMap<String, (usize, usize)>,
    sets: HashMap<String, usize>,
    start: Vec<(usize, usize, usize)>,
    output: Vec<(usize, usize)>,
    rules: Vec<Rule>,
    to_rulenum: Array4D<usize>,
    to_color: Array3D<Vec<usize>>,
}

impl Ruletable {
    pub(crate) fn num_edges(&self) -> usize {
        self.edges.len()
    }

    pub(crate) fn num_colors(&self) -> usize {
        cmp::max(1, self.colors.len())
    }

    pub(crate) fn num_sets(&self) -> usize {
        self.sets.len()
    }

    pub(crate) fn starts(&self) -> &Vec<(usize, usize, usize)> {
        &self.start
    }

    pub(crate) fn outputs(&self) -> &Vec<(usize, usize)> {
        &self.output
    }

    pub(crate) fn possible_colors(&self, e1: usize, c1: usize, e2: usize) -> &Vec<usize> {
        self.to_color.get(e1, c1, e2)
    }

    pub(crate) fn get_edge_ids(&self, s: &str) -> Option<(usize, usize)> {
        self.edges.get(s).copied()
    }

    pub(crate) fn get_color_id(&self, s: &str) -> Option<usize> {
        self.colors.get(s).copied()
    }

    pub(crate) fn get_set_id(&self, s: &str) -> Option<usize> {
        self.sets.get(s).copied()
    }

    pub(crate) fn pass(&self, sets: &Sets, s1: State, s2: State) -> bool {
        let rule_num = *self.to_rulenum.get(s1.edge, s1.color, s2.edge, s2.color);
        self.rules[rule_num]
            .expression
            .evaluate(sets, s1.node, s2.node)
    }

    pub(crate) fn get_edge_strings(&self) -> Vec<String> {
        let mut edge_strings = vec!["".to_owned(); self.edges.len()];
        self.edges
            .iter()
            .for_each(|(k, &v)| edge_strings[v.0] = k.clone());
        edge_strings
    }

    pub(crate) fn get_color_strings(&self) -> Vec<String> {
        let mut color_strings = vec!["".to_owned(); self.colors.len()];
        self.colors
            .iter()
            .for_each(|(k, &v)| color_strings[v] = k.clone());
        color_strings
    }

    /// Constructs a `Ruletable` from the contents of a file.
    ///
    /// # Errors
    /// Returns an error if the file cannot be read or if parsing fails.
    pub fn from_file(filename: &str) -> Result<Ruletable, ReadRuletableError> {
        let ruletable_str = fs::read_to_string(filename)?;
        Self::from_multiline_string(&ruletable_str)
    }

    /// Constructs a `Ruletable` from a multi-line string.
    ///
    /// # Errors
    /// Returns an error if parsing the input string fails.
    pub fn from_multiline_string(ruletable_str: &str) -> Result<Ruletable, ReadRuletableError> {
        let lines = ruletable_str.split('\n');
        let mut ruletable = Self::new_empty();
        for (line_number, raw_line) in lines.enumerate() {
            let line = raw_line.trim();

            let line_type = Self::id_line(line);

            ruletable.parse_line(line, &line_type).map_err(|err| {
                ParseRuletableError(format!(
                    "line {}: trying to parse a {} line: {} \n  {}",
                    line_number + 1,
                    line_type,
                    err.0,
                    line.chars().take(80).collect::<String>()
                        + if line.len() <= 80 { "" } else { "..." }
                ))
            })?;
        }

        ruletable.precompute();
        Ok(ruletable)
    }

    fn new_empty() -> Ruletable {
        Ruletable {
            colors: HashMap::new(),
            edges: HashMap::new(),
            sets: HashMap::new(),
            start: Vec::new(),
            output: Vec::new(),
            rules: Vec::new(),
            to_rulenum: Array4D::new(0, 0, 0, 0, 0_usize),
            to_color: Array3D::new(0, 0, 0, Vec::new()),
        }
    }

    fn id_line(line: &str) -> LineType {
        if line.is_empty() {
            return LineType::Empty;
        }
        let identifier_lines = [
            LineType::Comment,
            LineType::Edges,
            LineType::Colors,
            LineType::Sets,
            LineType::Start,
            LineType::Output,
        ];
        for line_type in identifier_lines {
            if line.starts_with(
                line_type
                    .get_identifier()
                    .expect("line type should have an identifier"),
            ) {
                return line_type;
            }
        }
        LineType::Rule
    }

    fn parse_line(&mut self, line: &str, line_type: &LineType) -> Result<(), ParseRuletableError> {
        let to_parse = match line_type {
            LineType::Rule => line.to_owned(),
            LineType::Empty | LineType::Comment => "".to_owned(),
            _ => Self::get_remaining(
                line,
                line_type
                    .get_identifier()
                    .expect("line type should have an identifier"),
            )?,
        };

        match line_type {
            LineType::Empty | LineType::Comment => (),
            LineType::Edges => self.edges = Self::parse_edges(&to_parse)?,
            LineType::Colors => self.colors = Self::parse_labels(&to_parse, "colors")?,
            LineType::Sets => self.sets = Self::parse_labels(&to_parse, "sets")?,
            LineType::Start => self.start.append(&mut self.parse_start(&to_parse)?),
            LineType::Output => self.output.append(&mut self.parse_output(&to_parse)?),
            LineType::Rule => self.rules.push(self.parse_rule(&to_parse)?),
        }
        Ok(())
    }

    fn parse_edges(s: &str) -> Result<HashMap<String, (usize, usize)>, ParseRuletableError> {
        let edge_delimiter = ",";
        let tokens = Self::tokenize_with_delimiter(s, edge_delimiter);
        let mut result = HashMap::new();
        let mut cnt = 0;
        for t in tokens.into_iter() {
            let edge_tokens: Vec<_> = Self::tokenize_at_whitespace(&t);
            if edge_tokens.len() > 2 {
                return Err(ParseRuletableError(format!(
                    "found more than two whitespace separated edge strings, expected one string for a symmetric edge or two strings for an asymmetric edge: {}",
                    t
                )));
            }
            for (i, e) in edge_tokens.iter().enumerate() {
                if e.is_empty() {
                    return Err(ParseRuletableError(
                        "found empty string, expected an edge".to_owned(),
                    ));
                }
                if result
                    .insert(e.clone(), (cnt + i, cnt + edge_tokens.len() - i - 1))
                    .is_some()
                {
                    return Err(ParseRuletableError(format!("found '{e}' twice")));
                }
            }
            cnt += edge_tokens.len();
        }
        Ok(result)
    }

    fn parse_labels(
        s: &str,
        description: &str,
    ) -> Result<HashMap<String, usize>, ParseRuletableError> {
        let tokens = Self::tokenize_with_delimiter(s, ",");
        let mut result = HashMap::new();
        for (i, s) in tokens.into_iter().enumerate() {
            if s.is_empty() {
                return Err(ParseRuletableError(format!(
                    "found empty string, expected a {description}"
                )));
            }
            if result.insert(s.clone(), i).is_some() {
                return Err(ParseRuletableError(format!("found '{s}' twice")));
            }
        }
        Ok(result)
    }

    fn parse_start(&self, s: &str) -> Result<Vec<(usize, usize, usize)>, ParseRuletableError> {
        let set_delimiter = " AT ";
        let tokens = Self::tokenize_with_delimiter(s, set_delimiter);

        if tokens.len() < 2 {
            return Err(ParseRuletableError(format!(
                "did not find space-separated keyword '{}', expected one occurence of '{}' followed by comma-separated sets", set_delimiter.trim(), set_delimiter.trim()
            )));
        }
        if tokens.len() > 2 {
            return Err(ParseRuletableError(format!(
                "found space-separated keyword '{}' more than once, expected one occurence of '{}' followed by a list of sets", set_delimiter.trim(), set_delimiter.trim()
            )));
        }

        let edge_color_str = &tokens[0];
        let set_str = &tokens[1];

        let (edge_pattern, color_pattern) = self.parse_edge_color_patterns(edge_color_str)?;
        let edges = edge_pattern.convert_to_vec(self.edges.len());
        let colors = color_pattern.convert_to_vec(self.colors.len());
        let sets = self.find_sets(set_str)?;

        let mut start = Vec::new();
        for &set in sets.iter() {
            for &edge in edges.iter() {
                for &color in colors.iter() {
                    start.push((set, edge, color));
                }
            }
        }
        Ok(start)
    }

    fn parse_output(&self, s: &str) -> Result<Vec<(usize, usize)>, ParseRuletableError> {
        let (edge_pattern, color_pattern) = self.parse_edge_color_patterns(s)?;
        let edges = edge_pattern.convert_to_vec(self.edges.len());
        let colors = color_pattern.convert_to_vec(self.colors.len());

        let mut output = Vec::new();
        for &e in edges.iter() {
            for &c in colors.iter() {
                output.push((e, c));
            }
        }
        Ok(output)
    }

    fn parse_rule(&self, s: &str) -> Result<Rule, ParseRuletableError> {
        let rule_delimiter = "|";
        let rule_split = Self::tokenize_with_delimiter(s, rule_delimiter);

        if rule_split.len() < 3 {
            return Err(ParseRuletableError(format!(
                "expected two occurences of '{}' delimiting previous state, next state and expression, found {} occurences",
                rule_delimiter.trim(),
                rule_split.len() - 1
            )));
        }

        let case = self.parse_case(&rule_split[0], &rule_split[1])?;
        let expression = self.parse_expression(&rule_split[2])?;
        Ok(Rule { case, expression })
    }

    fn parse_edge_color_patterns(
        &self,
        s: &str,
    ) -> Result<(Pattern, Pattern), ParseRuletableError> {
        let braces = ('[', ']');
        let num_open = s.chars().filter(|&c| c == braces.0).count();
        let num_closed = s.chars().filter(|&c| c == braces.1).count();
        if num_open != num_closed {
            return Err(ParseRuletableError(format!(
                "opening '{}' and closing '{}' braces are not matching",
                braces.0, braces.1
            )));
        }
        if num_open > 1 || num_closed > 1 {
            return Err(ParseRuletableError(format!(
                "more than one pair of braces '{}' '{}' found, expected none when colors are not specified or one with comma separated colors", braces.0, braces.1
            )));
        }
        let (edge_str, color_str);
        if num_open == 0 {
            edge_str = s.to_owned();
            color_str = "".to_owned();
        } else {
            if s.chars().last().unwrap() != braces.1 {
                return Err(ParseRuletableError(format!(
                    "last non-whitespace character is not a closing brace '{}', expected brace to close color list", braces.1
                )));
            }
            let open_pos = s.chars().position(|c| c == braces.0).unwrap();
            edge_str = s[..open_pos].to_owned();
            color_str = s[open_pos + 1..s.len() - 1].to_owned();
        }

        Ok((
            self.parse_edge_pattern(&edge_str)?,
            self.parse_color_pattern(&color_str)?,
        ))
    }

    fn parse_edge_pattern(&self, s: &str) -> Result<Pattern, ParseRuletableError> {
        let delimiter = ",";
        let tokens = Self::tokenize_with_delimiter(s, delimiter);

        let edge_wildcard = "...";
        if tokens.len() == 1 {
            let t = &tokens[0];
            if t == edge_wildcard {
                return Ok(Pattern::All);
            } else {
                return Ok(Pattern::Single(self.find_edge(t)?));
            }
        }

        let mut edge_list = Vec::new();
        for t in tokens {
            if t == edge_wildcard {
                return Err(ParseRuletableError(format!(
                "found edge wildcard '{edge_wildcard}' and other edge strings, if you want to match all strings, keep only the wildcard",
                )));
            }
            edge_list.push(self.find_edge(&t)?);
        }
        Ok(Pattern::Many(edge_list))
    }

    fn parse_color_pattern(&self, s: &str) -> Result<Pattern, ParseRuletableError> {
        if s.is_empty() || s.trim() == "..." {
            return Ok(Pattern::All);
        }

        let delimiter = ",";
        let tokens = Self::tokenize_with_delimiter(s, delimiter);
        if tokens.len() == 1 {
            let t = &tokens[0];
            return Ok(Pattern::Single(self.find_color(t)?));
        }

        let mut color_list = Vec::new();
        for t in tokens {
            color_list.push(self.find_color(&t)?);
        }
        Ok(Pattern::Many(color_list))
    }

    fn parse_case(&self, prev_str: &str, next_str: &str) -> Result<Case, ParseRuletableError> {
        let (prev_edge, prev_color) = self.parse_edge_color_patterns(prev_str)?;
        let (next_edge, next_color) = self.parse_edge_color_patterns(next_str)?;
        Ok(Case {
            prev_edge,
            prev_color,
            next_edge,
            next_color,
        })
    }

    fn parse_expression(&self, s: &str) -> Result<Expression, ParseRuletableError> {
        Ok(Expression::from_string(s, &self.sets)?)
    }

    fn find_edge(&self, s: &str) -> Result<usize, ParseRuletableError> {
        Ok(self
            .get_edge_ids(s)
            .ok_or_else(|| {
                ParseRuletableError(format!(
                    "could not find edge '{s}', are you sure you defined it?"
                ))
            })?
            .0)
    }

    fn find_color(&self, s: &str) -> Result<usize, ParseRuletableError> {
        self.get_color_id(s).ok_or_else(|| {
            ParseRuletableError(format!(
                "could not find color '{s}', are you sure you defined it?"
            ))
        })
    }

    fn find_sets(&self, s: &str) -> Result<Vec<usize>, ParseRuletableError> {
        let delimiter = ",";
        let tokens = Self::tokenize_with_delimiter(s, delimiter);

        let mut sets = Vec::new();
        for set_str in tokens {
            sets.push(*self.sets.get(&set_str).ok_or_else(|| {
                ParseRuletableError(format!(
                    "could not find set '{set_str}', are you sure you defined it?",
                ))
            })?);
        }

        Ok(sets)
    }

    fn get_remaining(s: &str, identifier: &str) -> Result<String, ParseRuletableError> {
        s.strip_prefix(identifier)
            .map(|x| x.trim().to_owned())
            .ok_or(ParseRuletableError(format!(
                "expected line starting with '{identifier}'"
            )))
    }

    fn tokenize_with_delimiter(s: &str, delimiter: &str) -> Vec<String> {
        let mut to_tokenize = s.to_owned();
        // insert space to ensure matching leading and trailing space-separated keywords
        to_tokenize.insert(0, ' ');
        to_tokenize.insert(to_tokenize.len(), ' ');
        to_tokenize
            .split(delimiter)
            .map(|s| s.trim().to_owned())
            .collect()
    }

    fn tokenize_at_whitespace(s: &str) -> Vec<String> {
        s.split_whitespace().map(|s| s.trim().to_owned()).collect()
    }

    fn precompute(&mut self) {
        // usize::MAX is overwritten for all colors that will ever get queried
        // choosing usize:MAX ensures panic in case this doesn't hold
        self.to_rulenum = Array4D::new(
            self.num_edges(),
            self.num_colors(),
            self.num_edges(),
            self.num_colors(),
            usize::MAX,
        );
        self.to_color = Array3D::new(
            self.num_edges(),
            self.num_colors(),
            self.num_edges(),
            Vec::new(),
        );
        for e1 in 0..self.num_edges() {
            for c1 in 0..self.num_colors() {
                for e2 in 0..self.num_edges() {
                    for c2 in 0..self.num_colors() {
                        let s1 = State {
                            node: 0,
                            edge: e1,
                            color: c1,
                        };
                        let s2 = State {
                            node: 0,
                            edge: e2,
                            color: c2,
                        };
                        for (i, rule) in self.rules.iter().enumerate() {
                            if rule.case.is_matched(s1, s2) {
                                *self.to_rulenum.get_mut(e1, c1, e2, c2) = i;
                                self.to_color.get_mut(e1, c1, e2).push(c2);
                                break;
                            }
                        }
                    }
                }
            }
        }
    }
}

pub struct Rule {
    case: Case,
    expression: Expression,
}

struct Case {
    prev_edge: Pattern,
    prev_color: Pattern,
    next_edge: Pattern,
    next_color: Pattern,
}

impl Case {
    fn is_matched(&self, prev: State, next: State) -> bool {
        self.prev_edge.is_matched(prev.edge)
            && self.prev_color.is_matched(prev.color)
            && self.next_edge.is_matched(next.edge)
            && self.next_color.is_matched(next.color)
    }
}

#[derive(Clone, Debug)]
pub enum Pattern {
    Single(usize),
    Many(Vec<usize>),
    All,
}

impl Pattern {
    fn convert_to_vec(&self, num: usize) -> Vec<usize> {
        match self {
            Pattern::Single(p) => vec![*p],
            Pattern::Many(ps) => ps.to_vec(),
            Pattern::All => (0..cmp::max(1, num)).collect(),
        }
    }

    fn is_matched(&self, found: usize) -> bool {
        match self {
            Pattern::Single(p) => *p == found,
            Pattern::Many(ps) => ps.iter().any(|&p| p == found),
            Pattern::All => true,
        }
    }
}

#[derive(Debug)]
pub enum ReadRuletableError {
    IoError(std::io::Error),
    ParseError(ParseRuletableError),
}

impl fmt::Display for ReadRuletableError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ReadRuletableError::IoError(e) => write!(f, "I/O Error: {}", e),
            ReadRuletableError::ParseError(e) => write!(f, "Parse Error: {}", e),
        }
    }
}

impl Error for ReadRuletableError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ReadRuletableError::IoError(e) => Some(e),
            ReadRuletableError::ParseError(e) => Some(e),
        }
    }
}

impl From<std::io::Error> for ReadRuletableError {
    fn from(error: std::io::Error) -> Self {
        ReadRuletableError::IoError(error)
    }
}

impl From<ParseRuletableError> for ReadRuletableError {
    fn from(error: ParseRuletableError) -> Self {
        ReadRuletableError::ParseError(error)
    }
}

#[derive(Debug)]
pub struct ParseRuletableError(String);

impl fmt::Display for ParseRuletableError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Parse Ruletable Error: {}", self.0)
    }
}

impl Error for ParseRuletableError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

impl From<ParseExpressionError> for ParseRuletableError {
    fn from(error: ParseExpressionError) -> Self {
        ParseRuletableError(format!("Error parsing expression: {}", error))
    }
}

enum LineType {
    Empty,
    Comment,
    Edges,
    Colors,
    Sets,
    Start,
    Output,
    Rule,
}

impl fmt::Display for LineType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LineType::Empty => write!(f, "empty"),
            LineType::Comment => write!(f, "comment"),
            LineType::Edges => write!(f, "edge declaration"),
            LineType::Colors => write!(f, "color declaration"),
            LineType::Sets => write!(f, "set declaration"),
            LineType::Start => write!(f, "start declaration"),
            LineType::Output => write!(f, "output declaration"),
            LineType::Rule => write!(f, "rule declaration"),
        }
    }
}

impl LineType {
    fn get_identifier(&self) -> Option<&str> {
        match self {
            LineType::Empty | LineType::Rule => None,
            LineType::Comment => Some("#"),
            LineType::Edges => Some("EDGES"),
            LineType::Colors => Some("COLORS"),
            LineType::Sets => Some("SETS"),
            LineType::Start => Some("START"),
            LineType::Output => Some("OUTPUT"),
        }
    }
}
