pub struct ObsidianLinkRule<const PREFIX: char>;
impl<const PREFIX: char> InlineRule for ObsidianLinkRule<PREFIX> {
    const MARKER: char = PREFIX;
    fn check(state: &mut markdown_it::parser::inline::InlineState) -> Option<usize> {
        let mut chars = state.src[state.pos..state.pos_max].chars();
        if PREFIX == '!' {
            if chars.next() != Some(PREFIX) {
                return None;
            }
        }

        if chars.next() != Some('[') {
            return None;
        }
        if chars.next() != Some('[') {
            return None;
        }

        return Some(3);
    }

    fn run(state: &mut markdown_it::parser::inline::InlineState) -> Option<(Node, usize)> {
        let pos = state.pos;
        let pos_max = state.pos_max;

        let mut chars = state.src[state.pos..state.pos_max].chars();
        let mut offset: usize = 0;
        let mut embed = false;
        if PREFIX == '!' {
            let value = chars.next();
            if value != Some('!') {
                return None;
            }
            offset = 1;
            embed = true;
        }

        if chars.next() != Some('[') {
            return None;
        }
        if chars.next() != Some('[') {
            return None;
        }

        let input = &state.src[(pos + 2 + offset)..pos_max];

        let left_bracket = input.find("[[");
        let right_bracket = input.find("]]");
        if right_bracket.is_none() {
            return None;
        }
        let right_pos = right_bracket.unwrap();
        if left_bracket.is_some() && left_bracket.unwrap() < right_pos {
            return None;
        }

        let ob_link = &input[0..right_pos];
        if let Some(ob) = parse_obsidian_link(ob_link, embed) {
            let node = Node::new(ob);
            let lenght = right_pos + 4 + offset; // [[ 移动了四格，
            return Some((node, lenght));
        }

        None
    }
}
